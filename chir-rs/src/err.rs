//! Error Handling

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use tracing::error;

#[repr(transparent)]
#[derive(Debug)]
pub struct Error(anyhow::Error);

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        error!("Failed to handle request: {:?}", self.0);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Internal server error: {:?}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for Error
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

pub type RespResult<T> = Result<T, Error>;
