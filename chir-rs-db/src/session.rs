//! Session-related functionality

use std::time::Duration;

use crate::Database;
use chir_rs_misc::id_generator;
use eyre::Result;
use rand::{thread_rng, Rng as _};
use sqlx::query;
use tracing::{error, info, instrument};

/// Expires outdated sessions
///
/// # Errors
///
/// This function returns an error if accessing the database failed.
#[instrument]
pub async fn expire(db: &Database) -> Result<()> {
    let id = id_generator::generate();
    let oldest_acceptable_id = id - ((24 * 60 * 60) << 48);
    let oldest_acceptable_id = oldest_acceptable_id.to_be_bytes();
    query!(
        r#"DELETE FROM "session_scopes" WHERE session_id < $1"#,
        &oldest_acceptable_id
    )
    .execute(&*db.0)
    .await?;
    Ok(())
}

/// Automatically expires outdated sessions
///
/// This is intended to be called on a dedicated job.
#[instrument]
pub async fn expire_sessions_job(db: Database) {
    info!("Starting expire sessions job thread");
    loop {
        info!("Reaping expired sessions");
        if let Err(e) = expire(&db).await {
            error!("Failed to reap expired sessions: {e:?}");
        }
        let secs_to_sleep = thread_rng().gen_range(1800..=5400);
        info!("Done. Sleeping for {secs_to_sleep}s");
        tokio::time::sleep(Duration::from_secs(secs_to_sleep)).await;
    }
}
