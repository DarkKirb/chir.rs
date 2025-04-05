//! User-related APIs

use std::{collections::HashSet, fmt::Debug};

use bincode::{Decode, Encode};
use chir_rs_common::{http_api::auth::Scope, id_generator};
use eyre::{Context, Result};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, query};
use tracing::instrument;

use super::Database;

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
    #[instrument(skip(db))]
    pub async fn get(db: &Database, username: &str) -> Result<Option<Self>> {
        #[allow(clippy::panic, reason = "sqlx moment")]
        let res = query!(r#"SELECT * FROM "user" WHERE username = $1"#, username)
            .fetch_optional(&db.0)
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

    /// Creates a new session for user
    ///
    /// The caller has to ensure that the user has authorized this.
    ///
    /// # Errors
    /// An error occurs if accessing the database fails
    #[instrument(skip(db))]
    #[allow(clippy::panic, reason = "sqlx moment")]
    pub async fn new_session(&self, db: &Database, scopes: HashSet<Scope>) -> Result<u128> {
        let mut txn = db.0.begin().await?;
        let user_id: i64 = self.id.try_into()?;
        let session_id_num = id_generator::generate();
        let session_id = session_id_num.to_be_bytes();

        query!(
            "INSERT INTO sessions (id, user_id) VALUES ($1, $2)",
            &session_id,
            user_id
        )
        .execute(&mut *txn)
        .await?;

        for scope in scopes {
            query!(
                "INSERT INTO session_scopes (session_id, scope) VALUES ($1, $2)",
                &session_id,
                scope.to_i64()
            )
            .execute(&mut *txn)
            .await?;
        }

        txn.commit().await?;

        Ok(session_id_num)
    }
}
