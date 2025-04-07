//! Authentication APIs
use b64_ct::{FromBase64, ToBase64, URL_SAFE};
use bincode::{Decode, Encode};
use educe::Educe;
use eyre::{bail, eyre, Result};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    fmt::{Debug, Display},
};

/// List of supported scopes for authentication
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Encode, Decode, Hash)]
pub enum Scope {
    /// Full scope granted by logging in.
    Full,
    /// The ability to create or update files.
    CreateUpdateFile,
    /// The ability to modify robots.txt
    Robots,
}

impl Display for Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Full => write!(f, "Full permissions"),
            Self::CreateUpdateFile => write!(f, "Create and update files"),
            Self::Robots => write!(f, "Modify robots.txt"),
        }
    }
}

impl Scope {
    /// Converts the scope into an integer suitable for database storage
    #[must_use]
    pub const fn to_i64(self) -> i64 {
        match self {
            Self::Full => 0,
            Self::CreateUpdateFile => 1,
            Self::Robots => 2,
        }
    }

    /// Converts a scope ID to the scope
    ///
    /// # Errors
    /// This function returns an error if the scope ID is invalid.
    pub fn from_i64(id: i64) -> Result<Self> {
        match id {
            0 => Ok(Self::Full),
            1 => Ok(Self::CreateUpdateFile),
            2 => Ok(Self::Robots),
            _ => bail!("Invalid scope ID {id}"),
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
    #[must_use]
    pub fn to_paseto(&self) -> String {
        let res = self.token.to_base64(URL_SAFE);

        format!("v4.local.{res}")
    }
}
