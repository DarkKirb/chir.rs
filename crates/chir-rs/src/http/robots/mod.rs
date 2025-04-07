//! robots.txt management

use axum::extract::State;
use chir_rs_common::http_api::{
    auth::Scope, axum::bincode::Bincode, errors::APIError, robots::RobotsRule,
};
use chrono::Utc;
use eyre::Context;
use sqlx::query;

use crate::queue::QueueAction;

use super::{auth::req_auth::auth_header::AuthHeader, AppState};

/// Creates a robots.txt entry
///
/// # Errors
///
/// This function returns an error if the request fails.
pub async fn create_entry(
    State(state): State<AppState>,
    session: AuthHeader,
    Bincode(mut req): Bincode<RobotsRule>,
) -> Result<Bincode<RobotsRule>, APIError> {
    session.assert_scope(Scope::Robots)?;
    let mut txn = state
        .global
        .db
        .0
        .begin()
        .await
        .context("Starting transaction")?;
    req.id = query!(
        "INSERT INTO robots (user_agent, robot_path, allow) VALUES ($1, $2, $3) RETURNING robot_id",
        req.user_agent,
        req.path,
        req.allow
    )
    .fetch_one(&mut *txn)
    .await
    .context("Inserting new robots entry")?
    .robot_id;

    QueueAction::UpdateRobots
        .queue(&mut txn, Utc::now(), 0, Vec::new())
        .await?;

    txn.commit().await.context("Completing transaction")?;

    Ok(Bincode(req))
}
