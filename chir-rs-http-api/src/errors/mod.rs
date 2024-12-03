//! Main error type

use bincode::{Decode, Encode};
use http::StatusCode;
use thiserror::Error;

/// The main error type
#[derive(Clone, Debug, PartialEq, Eq, Encode, Decode, Error)]
pub enum APIError {
    /// Returned when the client sends the wrong content type to the server.
    #[error("Invalid content type: Expected {expected}, Received {received}")]
    ClientInvalidContentType {
        /// Expected value of the content type
        expected: String,
        /// Received value of the content type
        received: String,
    },
    /// Returned when the client does not send a content type header
    #[error("Missing content type: Expected {expected}")]
    ClientMissingContentType {
        /// Expected value of the content type
        expected: String,
    },
    /// Returned when the client payload is too large.
    #[error("Invalid payload: Too large")]
    PayloadTooBig,
    /// Returned when there is an unknown error loading the client payloud
    #[error("Failed to load payload")]
    PayloadLoadError,
    /// Returned when the client payload is malformed
    #[error("Invalid payload")]
    PayloadInvalid,
    /// Returned when the error is unknown
    #[error("Unknown Error")]
    Unknown(String),
    /// Returned when there is a database error
    #[error("Database error: {0}")]
    DatabaseError(String),
    /// Returned when the specified user was not found
    #[error("Resource not found: {0}")]
    UserNotFound(String),
    /// Invalid password
    #[error("Invalid password for user {0}")]
    InvalidPassword(String),
}

impl APIError {
    /// Returns the HTTP Status code of the error
    #[must_use]
    pub const fn status_code(&self) -> StatusCode {
        match *self {
            Self::ClientInvalidContentType { .. } | Self::ClientMissingContentType { .. } => {
                StatusCode::UNSUPPORTED_MEDIA_TYPE
            }
            Self::PayloadTooBig => StatusCode::PAYLOAD_TOO_LARGE,
            Self::PayloadLoadError | Self::PayloadInvalid => StatusCode::BAD_REQUEST,
            Self::UserNotFound(_) | Self::InvalidPassword(_) => StatusCode::UNAUTHORIZED,
            Self::Unknown(_) | Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
