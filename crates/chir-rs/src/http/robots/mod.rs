//! robots.txt management

use axum::extract::{Path, State};
use chir_rs_common::http_api::{
    auth::Scope, axum::bincode::Bincode, errors::APIError, robots::RobotsRule,
};
use chrono::Utc;
use eyre::Context;
use futures::StreamExt;
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

pub async fn list_entries(
    State(state): State<AppState>,
) -> Result<Bincode<Vec<RobotsRule>>, APIError> {
    let mut rules = Vec::new();

    let mut results = query!("SELECT * FROM robots").fetch(&state.global.db.0);

    while let Some(v) = results.next().await {
        let v = v.context("Fetching robots entry")?;
        rules.push(RobotsRule {
            id: v.robot_id,
            user_agent: v.user_agent,
            path: v.robot_path,
            allow: v.allow,
        });
    }

    Ok(Bincode(rules))
}

pub async fn get_entry(
    State(state): State<AppState>,
    Path(rule_id): Path<i32>,
) -> Result<Bincode<RobotsRule>, APIError> {
    let result = query!("SELECT * FROM robots WHERE robot_id = $1", rule_id)
        .fetch_optional(&state.global.db.0)
        .await
        .context("Fetching DB entry")?;

    if let Some(result) = result {
        Ok(Bincode(RobotsRule {
            id: result.robot_id,
            user_agent: result.user_agent,
            path: result.robot_path,
            allow: result.allow,
        }))
    } else {
        Err(APIError::MissingRobotsRule(rule_id))
    }
}

pub async fn delete_entry(
    State(state): State<AppState>,
    session: AuthHeader,
    Path(rule_id): Path<i32>,
) -> Result<(), APIError> {
    session.assert_scope(Scope::Robots)?;
    let mut txn = state
        .global
        .db
        .0
        .begin()
        .await
        .context("Starting transaction")?;
    query!("DELETE FROM robots WHERE robot_id = $1", rule_id)
        .execute(&mut *txn)
        .await
        .context("Deleting robots entry")?;

    QueueAction::UpdateRobots
        .queue(&mut txn, Utc::now(), 0, Vec::new())
        .await?;

    txn.commit().await.context("Completing transaction")?;

    Ok(())
}
