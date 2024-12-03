use std::collections::HashSet;

/// Authentication APIs
use bincode::{Decode, Encode};
use educe::Educe;
use serde::{Deserialize, Serialize};

/// List of supported scopes for authentication
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Encode, Decode, Hash)]
pub enum Scope {
    /// Full scope granted by logging in.
    Full,
}

impl Scope {
    /// Converts the scope into an integer suitable for database storage
    #[must_use]
    pub const fn to_i64(self) -> i64 {
        match self {
            Self::Full => 0,
        }
    }
}

/// Login request for the user
#[derive(Clone, Educe, Serialize, Deserialize, Encode, Decode)]
#[educe(Debug)]
pub struct LoginRequest {
    /// Username
    pub username: String,
    /// Password
    #[educe(Debug(ignore))]
    pub password: String,
    /// Scopes
    pub scopes: HashSet<Scope>,
}
