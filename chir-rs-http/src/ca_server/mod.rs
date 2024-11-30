//! Module that serves CA files

use axum::{
    body::Body,
    extract::State,
    http::{
        header::{ACCEPT, CACHE_CONTROL, CONTENT_LENGTH, CONTENT_TYPE, ETAG, IF_NONE_MATCH},
        HeaderMap, StatusCode, Uri,
    },
    response::Response,
};
use chir_rs_db::file::File;
use chir_rs_misc::lexicographic_base64;
use eyre::Context as _;
use mime::MimeIter;
use tracing::{debug, error, info};

use crate::AppState;

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

/// Serve static files
///
/// # Errors
///
/// This function returns an error if the request fails.
///
/// # Panics
///
/// This function panics if the error handling function panics
pub async fn serve_files(
    State(state): State<AppState>,
    uri: Uri,
    headers: HeaderMap,
) -> Result<Response, Response> {
    let path = uri.path();
    debug!("Fetching information about {path}");
    let files = File::get_by_path(&state.db, path)
        .await
        .with_context(|| format!("Fetching path {path} from database"))
        .map_err(format_error)?;
    if files.is_empty() {
        info!("Unknown file {path} requested.");
        return Err(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())
            .with_context(|| format!("Constructing 404 response for {path}"))
            .map_err(format_error)?);
    }

    let accept_media_type = match headers.get(ACCEPT) {
        Some(v) => v
            .to_str()
            .with_context(|| format!("Parsing accept header in request for {path}"))
            .map_err(format_error)?,
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
                .with_context(|| format!("Creating mismatched content type response for {path}"))
                .map_err(format_error)?;
        return Err(Response::builder()
            .status(StatusCode::NOT_ACCEPTABLE)
            .body(Body::from(accepted_mimes_resp))
            .with_context(|| format!("Creating mismatched content type response for {path}"))
            .map_err(format_error)?);
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
                .with_context(|| format!("Creating not modified response for {path}"))
                .map_err(format_error);
        }
    }

    // At this point we know that the client does not have the correct version of the path

    let (content_size, file_body) = state
        .ca
        .download(matched_file.b3hash)
        .await
        .with_context(|| format!("Downloading file for {path}"))
        .map_err(format_error)?;

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
        .map_err(format_error)
}
