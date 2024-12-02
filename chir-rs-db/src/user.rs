//! User-related APIs

use bincode::{Decode, Encode};
use eyre::{Context, Result};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, query};

use crate::Database;

/// User record
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromRow, Encode, Decode)]
pub struct User {
    /// ID of the user
    pub id: u64,
    /// Name of the user
    pub username: String,
    /// Password Hash
    pub password_hash: String,
}

impl User {
    /// Loads a user by username
    ///
    /// # Errors
    /// An error occurs if accessing the database fails
    #[allow(clippy::missing_panics_doc, reason = "sqlx moment")]
    pub async fn get(db: &Database, username: &str) -> Result<Option<Self>> {
        #[allow(clippy::panic, reason = "sqlx moment")]
        let res = query!(r#"SELECT * FROM "user" WHERE username = $1"#, username)
            .fetch_optional(&*db.0)
            .await
            .with_context(|| format!("Loading user information for {username}"))?;

        if let Some(res) = res {
            Ok(Some(Self {
                id: res.id.try_into()?,
                username: res.username,
                password_hash: res.password_hash,
            }))
        } else {
            Ok(None)
        }
    }
}
