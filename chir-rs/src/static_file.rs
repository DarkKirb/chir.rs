//! Static file handling

use anyhow::{anyhow, Context, Result};
use axum::{
    body::Body,
    extract::Path,
    http::{Response, StatusCode},
    response::IntoResponse,
};
use chir_rs_macros::static_embeds;
use tokio_util::io::ReaderStream;
use tracing::{debug, error, instrument};

use crate::err::RespResult;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum StaticFileContent<'a> {
    Embedded(&'a [u8]),
    External(&'a str),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct StaticFile<'a> {
    mime_type: &'a str,
    content: StaticFileContent<'a>,
    file_hash: &'a str,
}

impl StaticFile<'static> {
    pub fn mime_type(&self) -> &'static str {
        self.mime_type
    }

    pub fn content(&self) -> StaticFileContent<'static> {
        self.content
    }

    pub fn file_hash(&self) -> &'static str {
        self.file_hash
    }

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
        Ok(builder
            .with_context(|| format!("Constructing response for static file {}", self.file_hash))?)
    }
}

static_embeds!();

#[instrument]
pub async fn static_file(Path(file): Path<String>) -> RespResult<impl IntoResponse> {
    debug!("Received request for static file {}", file);
    async fn static_file(file: String) -> Result<StaticFile<'static>> {
        let static_file = STATIC_FILES
            .get(&file)
            .ok_or_else(|| anyhow!("Static file not found: {}", file))?;
        Ok(*static_file)
    }

    match static_file(file.clone())
        .await
        .with_context(|| format!("Could not handle request for static file {file}"))
    {
        Ok(response) => Ok(Ok(response.serve_file().await?)),
        Err(e) => {
            error!("{:?}", e);
            Ok(Err((
                StatusCode::NOT_FOUND,
                format!("Could not find static file {file}"),
            ))
            .into())
        }
    }
}
