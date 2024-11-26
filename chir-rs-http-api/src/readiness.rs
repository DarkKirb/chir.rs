//! APIs for testing readiness

use bincode::{Decode, Encode};

/// Current Ready State
#[derive(Copy, Clone, Debug, PartialEq, Eq, Encode, Decode)]
pub enum ReadyState {
    /// Indicates that this service is ready to receive requests
    Ready,
    /// Indicates that this service is not yet ready to receive requests
    NotReady,
}
