//! Chir.rs database models

use std::sync::Arc;

use eyre::{Context, Result};
use sqlx::{migrate, SqlitePool};
use tracing::instrument;

pub mod file;

/// Opaque database handle
#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct Database(Arc<SqlitePool>);

/// Opens the database
///
/// # Errors
/// This function returns an error if a connection to the database could not be established
#[instrument]
pub async fn open_database(path: &str) -> Result<Database> {
    let pool = SqlitePool::connect(path)
        .await
        .with_context(|| format!("Opening database {path}"))?;
    migrate!().run(&pool).await?;
    Ok(Database(Arc::new(pool)))
}
