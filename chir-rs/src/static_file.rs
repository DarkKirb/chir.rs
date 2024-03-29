//! Static file handling

use anyhow::{anyhow, Context, Result};
use axum::{
    body::Body,
    extract::Path,
    http::{HeaderMap, Response, StatusCode},
    response::IntoResponse,
};
use chir_rs_macros::static_embeds;
use tokio_util::io::ReaderStream;
use tracing::{debug, error, instrument};

use crate::err::RespResult;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[allow(clippy::module_name_repetitions)]
/// Content of a static file
pub enum StaticFileContent<'a> {
    /// Content of static file embedded into the binary
    Embedded(&'a [u8]),
    /// External static file content
    External(&'a str),
}

/// Static file info
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct StaticFile<'a> {
    /// MIME type of the file
    mime_type: &'a str,
    /// Content of the file
    content: StaticFileContent<'a>,
    /// Blake3 hash of the file
    file_hash: &'a str,
}

impl StaticFile<'static> {
    /// Return the mime type
    #[must_use]
    pub const fn mime_type(&self) -> &'static str {
        self.mime_type
    }

    /// Return the content of the file
    #[must_use]
    pub const fn content(&self) -> StaticFileContent<'static> {
        self.content
    }

    /// Return the file hash of the file
    #[must_use]
    pub const fn file_hash(&self) -> &'static str {
        self.file_hash
    }

    /// Serve the static file
    #[instrument]
    pub async fn serve_file(&self) -> Result<impl IntoResponse> {
        let builder = Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", self.mime_type())
            .header("Cache-Control", "public,max-age=31536000,immutable")
            .header("ETag", self.file_hash());
        let builder = match self.content {
            StaticFileContent::Embedded(v) => builder.body(Body::from(v)),
            StaticFileContent::External(path) => {
                let file = tokio::fs::OpenOptions::new()
                    .read(true)
                    .open(path)
                    .await
                    .with_context(|| format!("Opening static file {path}"))?;
                builder.body(Body::from_stream(ReaderStream::new(file)))
            }
        };
        builder.with_context(|| format!("Constructing response for static file {}", self.file_hash))
    }
}

static_embeds!();

/// Static file handler
#[instrument]
pub async fn static_file(Path(file): Path<String>, headers: HeaderMap) -> RespResult<impl IntoResponse> {
    #[allow(clippy::missing_docs_in_private_items)]
    fn static_file(file: &str) -> Result<StaticFile<'static>> {
        let static_file = STATIC_FILES
            .get(file)
            .ok_or_else(|| anyhow!("Static file not found: {}", file))?;
        Ok(*static_file)
    }

    debug!("Received request for static file {}", file);

    match static_file(&file)
        .with_context(|| format!("Could not handle request for static file {file}"))
    {
        Ok(response) => {
            let expected_etag = format!("\"{}\"", response.file_hash());
            for header in headers.get_all("If-None-Match") {
                if let Ok(etag) = header.to_str() {
                    for etag in etag.split(',') {
                        if etag.trim() == expected_etag {
                            return Ok(Err(Response::builder()
                               .status(StatusCode::NOT_MODIFIED)
                               .body(Body::empty())
                               .context("Building not modified body")?));
                        }
                    }
                }
            }
            Ok(Ok(response.serve_file().await?))
        }
        Err(e) => {
            error!("{:?}", e);
            Ok(Err(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::empty())
                .context("Building not found body")?))
        }
    }
}
