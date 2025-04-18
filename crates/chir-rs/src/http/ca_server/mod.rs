//! Module that serves CA files

use std::path::{Path, PathBuf};

use axum::{
    body::Body,
    extract::State,
    http::{
        header::{ACCEPT, CACHE_CONTROL, CONTENT_LENGTH, CONTENT_TYPE, ETAG, IF_NONE_MATCH},
        HeaderMap, Request, StatusCode, Uri,
    },
    response::Response,
};
use chir_rs_common::{
    http_api::{auth::Scope, errors::APIError},
    lexicographic_base64,
    queue::QueueAction,
};
use chrono::Utc;
use eyre::Context as _;
use futures::{AsyncReadExt, TryStreamExt};
use mime::MimeIter;
use mime_guess::guess_mime_type;
use tokio::fs;
use tokio_util::io::ReaderStream;
use tracing::{debug, error, info};

use crate::{db::file::File, queue};

use super::{auth::req_auth::auth_header::AuthHeader, AppState};

/// Formats an eyre error message
#[expect(clippy::expect_used, reason = "fatal error in error handling function")]
#[expect(clippy::needless_pass_by_value, reason = "Ergonomics")]
fn format_error(report: eyre::Report) -> Response {
    error!("Error while handling request: {report:?}");
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .header(CONTENT_TYPE, mime::TEXT_PLAIN_UTF_8.as_ref())
        .body(Body::new(format!("{report:?}")))
        .context("constructing error response body")
        .expect("Valid response body")
}

/// Serves a local path
async fn serve_local_file(path: &Path, headers: &HeaderMap) -> eyre::Result<Response> {
    if !fs::metadata(path).await?.is_file() {
        eyre::bail!("Not a file: {path:?}");
    }
    // The etag is simply the raccanonical path of this.
    let etags = headers
        .get(IF_NONE_MATCH)
        .and_then(|v| v.to_str().ok())
        .unwrap_or_default();
    let expected_etag = path.canonicalize()?.to_string_lossy().to_string();
    for etag in etags.split(',') {
        let etag = etag.trim().trim_start_matches("W/").trim_matches('"');
        if etag == expected_etag {
            return Response::builder()
                .status(StatusCode::NOT_MODIFIED)
                .header(ETAG, format!("\"{expected_etag}\""))
                .body(Body::empty())
                .with_context(|| format!("Creating not modified response for {}", path.display()));
        }
    }
    let mime = mime_guess::from_path(path).first_or_octet_stream();
    let file = fs::File::open(path)
        .await
        .with_context(|| format!("Opening static file {}", path.display()))?;

    let reader_stream = ReaderStream::new(file);

    Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, mime.to_string())
        .header(ETAG, format!("\"{expected_etag}\""))
        // Since this is a static file, the contents are public…
        .header(
            CACHE_CONTROL,
            "max-age=86400, s-max-age=31556926, must-revalidate, public",
        )
        .body(Body::from_stream(reader_stream))
        .context("Constructing response body")
}

/// Serve static files
///
/// # Errors
///
/// This function returns an error if the request fails.
///
/// # Panics
///
/// This function panics if the error handling function panics
async fn serve_files_2(state: AppState, uri: Uri, headers: HeaderMap) -> eyre::Result<Response> {
    let path = uri.path();
    debug!("Fetching information about {path}");
    let files = File::get_by_path(&state.global.db, path)
        .await
        .with_context(|| format!("Fetching path {path} from database"))?;
    if files.is_empty() {
        info!("Unknown file {path} requested.");
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())
            .with_context(|| format!("Constructing 404 response for {path}"));
    }

    let accept_media_type = match headers.get(ACCEPT) {
        Some(v) => v
            .to_str()
            .with_context(|| format!("Parsing accept header in request for {path}"))?,
        None => "*/*",
    };

    let mut matched_file = None;
    let mut match_score = 0;

    'outer: for accept_mime in MimeIter::new(accept_media_type).filter_map(Result::ok) {
        if accept_mime.type_() == mime::STAR && match_score != 0 {
            continue;
        }
        for file in &files {
            if accept_mime.type_() == mime::STAR {
                matched_file = Some(file);
                match_score = 1;
                continue 'outer;
            }
            if accept_mime.type_() != file.mime.type_() {
                continue;
            }
            if accept_mime.subtype() == mime::STAR || accept_mime.subtype() == file.mime.subtype() {
                matched_file = Some(file);
                break 'outer;
            }
        }
    }

    let Some(matched_file) = matched_file else {
        info!("Mismatched accept header {accept_media_type} for file path {path}");
        let accepted_mimes = files
            .iter()
            .map(|v| v.mime.as_ref().to_string())
            .collect::<Vec<_>>();
        let accepted_mimes_resp =
            bincode::encode_to_vec(accepted_mimes, bincode::config::standard())
                .with_context(|| format!("Creating mismatched content type response for {path}"))?;
        return Response::builder()
            .status(StatusCode::NOT_ACCEPTABLE)
            .body(Body::from(accepted_mimes_resp))
            .with_context(|| format!("Creating mismatched content type response for {path}"));
    };

    let etags = headers
        .get(IF_NONE_MATCH)
        .and_then(|v| v.to_str().ok())
        .unwrap_or_default();

    let expected_etag = lexicographic_base64::encode(matched_file.b3hash.as_bytes());
    for etag in etags.split(',') {
        let etag = etag.trim().trim_start_matches("W/").trim_matches('"');
        if etag == expected_etag {
            return Response::builder()
                .status(StatusCode::NOT_MODIFIED)
                .header(ETAG, format!("\"{expected_etag}\""))
                .body(Body::empty())
                .with_context(|| format!("Creating not modified response for {path}"));
        }
    }

    // At this point we know that the client does not have the correct version of the path

    let (content_size, file_body) = state
        .global
        .castore
        .download(matched_file.b3hash)
        .await
        .with_context(|| format!("Downloading file for {path}"))?;

    let mut response_builder = Response::builder()
        .status(StatusCode::OK)
        .header(ETAG, format!("\"{expected_etag}\""))
        // Since this is a static file, the contents are public…
        .header(
            CACHE_CONTROL,
            "max-age=86400, s-max-age=31556926, must-revalidate, public",
        )
        .header(CONTENT_TYPE, matched_file.mime.to_string());

    if let Some(content_size) = content_size {
        response_builder = response_builder.header(CONTENT_LENGTH, content_size);
    }

    response_builder
        .body(Body::new(file_body))
        .with_context(|| format!("Serving file for {path}"))
}

pub async fn serve_files(
    State(state): State<AppState>,
    uri: Uri,
    headers: HeaderMap,
) -> Result<Response, Response> {
    let mut path = state.global.cfg.static_dir.clone();

    for segment in uri.path().split('/').filter(|s| *s != "..") {
        path.push(segment);
    }

    if !path.starts_with(&state.global.cfg.static_dir) {
        Err(eyre::eyre!(
            "Path {path:?} is not in static dir {:?}",
            state.global.cfg.static_dir
        ))
        .map_err(format_error)?;
    }

    match serve_local_file(&path, &headers).await {
        Ok(r) => Ok(r),
        Err(e) => serve_files_2(state, uri, headers)
            .await
            .context(e)
            .map_err(format_error),
    }
}

/// Creates a static file
///
/// # Errors
///
/// This function returns an error if the request fails.
pub async fn create_files(
    State(state): State<AppState>,
    session: AuthHeader,
    uri: Uri,
    headers: HeaderMap,
    req: Request<Body>,
) -> Result<(), APIError> {
    session.assert_scope(Scope::CreateUpdateFile)?;
    let mime = headers
        .get(CONTENT_TYPE)
        .ok_or_else(|| APIError::ClientMissingContentType {
            expected: "*/*".to_string(),
        })?
        .to_str()
        .map_err(|e| APIError::ClientInvalidContentType {
            expected: "*/*".to_string(),
            received: format!("{e:?}"),
        })?;
    let mut data = Vec::new();
    TryStreamExt::map_err(req.into_body().into_data_stream(), std::io::Error::other)
        .into_async_read()
        .read_to_end(&mut data)
        .await
        .context("Reading the body")?;
    let mut txn = state
        .global
        .db
        .0
        .begin()
        .await
        .context("Starting raccontext")?;
    let ca_id = queue::queue(
        QueueAction::UploadCA(data),
        &mut txn,
        Utc::now(),
        0,
        Vec::new(),
    )
    .await?;
    queue::queue(
        QueueAction::RaccreateFile(uri.path().to_string(), mime.to_string()),
        &mut txn,
        Utc::now(),
        0,
        vec![ca_id],
    )
    .await?;
    txn.commit().await.context("Queuing jobs")?;
    Ok(())
}
