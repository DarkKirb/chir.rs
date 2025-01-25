//! Session-related functionality

use std::{collections::HashSet, time::Duration};

use crate::Database;
use chir_rs_http_api::auth::Scope;
use chir_rs_misc::id_generator;
use eyre::Result;
use futures::StreamExt as _;
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
    let oldest_acceptable_id = id - ((24 * 60 * 60 * 1_000_000_000) << 48);
    let oldest_acceptable_id = oldest_acceptable_id.to_be_bytes();
    query!(
        r#"DELETE FROM "sessions" WHERE id < $1"#,
        &oldest_acceptable_id
    )
    .execute(&*db.0)
    .await?;
    Ok(())
}

/// Returns username and scopes for a session ID
///
/// # Errors
/// This function returns an error if accessing the database fails
#[instrument]
pub async fn fetch_session_info(
    db: &Database,
    session_id: u128,
) -> Result<Option<(String, HashSet<Scope>)>> {
    let session_id = session_id.to_be_bytes();
    let Some(username_record) = query!(
        r#"
        SELECT "user".username FROM "user"
            INNER JOIN "sessions"
                ON "sessions".user_id = "user".id
            WHERE "sessions".id = $1
    "#,
        &session_id
    )
    .fetch_optional(&*db.0)
    .await?
    else {
        return Ok(None);
    };

    let mut scopes = HashSet::with_capacity(4);

    let mut scopes_records = query!(
        "SELECT scope FROM session_scopes WHERE session_id = $1",
        &session_id
    )
    .fetch(&*db.0);

    while let Some(scope_record) = scopes_records.next().await {
        scopes.insert(Scope::from_i64(scope_record?.scope)?);
    }

    Ok(Some((username_record.username, scopes)))
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
