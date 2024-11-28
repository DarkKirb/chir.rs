//! Chir.rs database models

use std::sync::Arc;

use eyre::{Context, Result};
use sqlx::{migrate, PgPool};
use tracing::instrument;

pub mod file;

/// Opaque database handle
#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct Database(Arc<PgPool>);

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
