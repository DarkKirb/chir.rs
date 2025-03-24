//! Binary serialization format

use axum_core::{
    extract::{FromRequest, Request},
    response::{IntoResponse, Response},
};
use bincode::{Decode, Encode};
use bytes::{BufMut as _, Bytes, BytesMut};
use http::{header::CONTENT_TYPE, HeaderValue, StatusCode};
use tracing::error;

use crate::errors::APIError;

/// Bincode wrapper
#[derive(Debug, Clone, Copy, Default)]
#[repr(transparent)]
#[must_use]
pub struct Bincode<T>(pub T);

impl IntoResponse for APIError {
    fn into_response(self) -> Response {
        (self.status_code(), Bincode(self)).into_response()
    }
}

impl<T, S> FromRequest<S> for Bincode<T>
where
    T: Decode<()>,
    S: Send + Sync,
{
    type Rejection = APIError;
    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        match req.headers().get(CONTENT_TYPE) {
            Some(c) if c == "application/x+bincode" => {
                let bytes = Bytes::from_request(req, state).await?;
                match bincode::decode_from_slice(&bytes, bincode::config::standard()) {
                    Ok(v) => Ok(Self(v.0)),
                    Err(e) => {
                        error!("Body decode error: {e:?}");
                        Err(APIError::PayloadInvalid)
                    }
                }
            }
            Some(c) => Err(APIError::ClientInvalidContentType {
                expected: "application/x+bincode".to_string(),
                received: c
                    .to_str()
                    .map_or_else(|_| format!("{c:?}"), ToString::to_string),
            }),
            _ => Err(APIError::ClientMissingContentType {
                expected: "application/x+bincode".to_string(),
            }),
        }
    }
}

impl<T> IntoResponse for Bincode<T>
where
    T: Encode,
{
    fn into_response(self) -> Response {
        let mut buf = BytesMut::with_capacity(128).writer();
        match bincode::encode_into_std_write(self.0, &mut buf, bincode::config::standard()) {
            Ok(_) => (
                [(
                    CONTENT_TYPE,
                    HeaderValue::from_static("application/x+bincode"),
                )],
                buf.into_inner().freeze(),
            )
                .into_response(),
            Err(err) => {
                error!("Failed to encode bincode response: {err:?}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    [(
                        CONTENT_TYPE,
                        HeaderValue::from_static(mime::TEXT_PLAIN_UTF_8.as_ref()),
                    )],
                    "internal server error".to_string(),
                )
                    .into_response()
            }
        }
    }
}
