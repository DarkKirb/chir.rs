//! Chir.rs database models

use std::{sync::Arc, time::Duration};

use eyre::{eyre, Context, Result};
use sqlx::{migrate, query, PgPool};
use tracing::instrument;

pub mod file;
pub mod user;

/// Opaque database handle
#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct Database(Arc<PgPool>);

impl Database {
    /// This function verifies an active connection to the database.
    ///
    /// # Errors
    /// This function returns an error if the database connection has failed or a timeout of 1s occurred
    #[instrument]
    pub async fn ping(&self) -> Result<()> {
        let fut = async {
            match query!("SELECT 1 as running").fetch_one(&*self.0).await {
                Ok(v) if v.running == Some(1) => Ok::<_, eyre::Report>(()),
                Err(e) => Err(e).context("Checking for readiness"),
                r => Err(eyre!("Unknown database response: {r:#?}")),
            }
        };

        tokio::time::timeout(Duration::from_secs(1), fut)
            .await
            .context("Awaiting a ping")??;

        Ok(())
    }
}

/// Opens the database
///
/// # Errors
/// This function returns an error if a connection to the database could not be established
#[instrument]
pub async fn open_database(path: &str) -> Result<Database> {
    let db_string = tokio::fs::read_to_string(path)
        .await
        .with_context(|| format!("Reading db connection string at {path}"))?;
    let pool = PgPool::connect(&db_string)
        .await
        .with_context(|| format!("Opening database {path}"))?;
    migrate!().run(&pool).await?;
    Ok(Database(Arc::new(pool)))
}
