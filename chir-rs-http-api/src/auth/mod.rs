//! Authentication APIs
use b64_ct::{FromBase64, ToBase64, URL_SAFE};
use bincode::{Decode, Encode};
use educe::Educe;
use eyre::{bail, eyre, Result};
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, fmt::Debug};

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

/// Compacted PASETO v4.local token
#[derive(Clone, Serialize, Deserialize, Encode, Decode)]
pub struct PasetoToken {
    /// The encoded token
    pub token: Vec<u8>,
}

impl Debug for PasetoToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PasetoToken").finish_non_exhaustive()
    }
}

impl PasetoToken {
    /// Parses a compressed `PasetoToken` from string
    ///
    /// # Errors
    /// This function returns an error if the paseto token passed is invalid
    pub fn from_paseto(text: &str) -> Result<Self> {
        if !text.starts_with("v4.local.") {
            bail!("Invalid paseto token!");
        }
        Ok(Self {
            token: text
                .trim_start_matches("v4.local.")
                .from_base64()
                .map_err(|e| eyre!("{e:?}"))?,
        })
    }

    /// Converts a compressed `PasetoToken` to string
    pub fn to_paseto(&self) -> Result<String> {
        let res = self.token.to_base64(URL_SAFE);

        Ok(format!("v4.local.{res}"))
    }
}
