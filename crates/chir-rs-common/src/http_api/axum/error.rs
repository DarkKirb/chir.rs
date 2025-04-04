//! Error type impls

use axum_core::extract::rejection::{
    BytesRejection, FailedToBufferBody, LengthLimitError, UnknownBodyError,
};

use crate::http_api::errors::APIError;

impl From<LengthLimitError> for APIError {
    fn from(_: LengthLimitError) -> Self {
        Self::PayloadTooBig
    }
}

impl From<UnknownBodyError> for APIError {
    fn from(_: UnknownBodyError) -> Self {
        Self::PayloadLoadError
    }
}

impl From<FailedToBufferBody> for APIError {
    fn from(value: FailedToBufferBody) -> Self {
        match value {
            FailedToBufferBody::LengthLimitError(length_limit_error) => length_limit_error.into(),
            FailedToBufferBody::UnknownBodyError(unknown_body_error) => unknown_body_error.into(),
            rest => Self::Unknown(format!("Unknown FailedToBufferBody: {rest:?}")),
        }
    }
}

impl From<BytesRejection> for APIError {
    fn from(value: BytesRejection) -> Self {
        match value {
            BytesRejection::FailedToBufferBody(failed_to_buffer_body) => {
                failed_to_buffer_body.into()
            }
            rest => Self::Unknown(format!("Unknown BytesRejection: {rest:?}")),
        }
    }
}
