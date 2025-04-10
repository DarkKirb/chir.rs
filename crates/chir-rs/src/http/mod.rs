//! HTTP server implementation for chir-rs

use std::{fmt::Debug, sync::Arc};

use axum::{
    extract::{MatchedPath, Request, State},
    http::StatusCode,
    routing::{delete, get, post},
    Router,
};
use axum_prometheus::PrometheusMetricLayer;
use b64_ct::FromBase64;
use chir_rs_common::http_api::{axum::bincode::Bincode, readiness::ReadyState};
use eyre::{bail, eyre, Context, Result};
use rusty_paseto::core::{Key, Local, PasetoSymmetricKey, V4};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::{error, info, info_span};

use crate::Global;

pub mod auth;
pub mod ca_server;
pub mod robots;

/// Application state
#[derive(Clone)]
pub struct AppState {
    /// Global state
    pub global: Arc<Global>,
    /// PASETO private key
    pub paseto_key: Arc<PasetoSymmetricKey<V4, Local>>,
}

impl Debug for AppState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppState").finish_non_exhaustive()
    }
}

/// Entrypoint for the HTTP server component
///
/// # Errors
/// This function returns an error if the startup of the server fails.
///
/// Errors it encounters during runtime should be automatically handled.
pub async fn main(global: Arc<Global>) -> Result<()> {
    let paseto_symmetric_key =
        tokio::fs::read_to_string(global.cfg.paseto_secret_key_file.clone()).await?;
    let paseto_symmetric_key = paseto_symmetric_key
        .from_base64()
        .map_err(|e| eyre!("{e:?}"))?;
    if paseto_symmetric_key.len() != 32 {
        bail!("Invalid symmetric key size");
    }
    let paseto_symmetric_key = PasetoSymmetricKey::from(Key::from(paseto_symmetric_key.as_slice()));

    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();
    let app = Router::new()
        // Routes here
        .route(
            "/.api/readyz",
            get(|State(state): State<AppState>| async move {
                match state.global.db.ping().await {
                    Ok(()) => (StatusCode::OK, Bincode(ReadyState::Ready)),
                    Err(e) => {
                        error!("Database is not responding: {e:?}");
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Bincode(ReadyState::NotReady),
                        )
                    }
                }
            }),
        )
        .route(
            "/.api/metrics",
            get(|| async move { metric_handle.render() }),
        )
        .route(
            "/.api/auth/login",
            post(auth::password_login::login).delete(auth::password_login::logout),
        )
        .route(
            "/.api/robots",
            post(robots::create_entry).get(robots::list_entries),
        )
        .route(
            "/.api/robots/{rule_id}",
            get(robots::get_entry).delete(robots::delete_entry),
        )
        .fallback(get(ca_server::serve_files).post(ca_server::create_files))
        .with_state(AppState {
            global: Arc::clone(&global),
            paseto_key: Arc::new(paseto_symmetric_key),
        })
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                let matched_path = request
                    .extensions()
                    .get::<MatchedPath>()
                    .map(MatchedPath::as_str);

                info_span!(
                    "http_request",
                    method = ?request.method(),
                    matched_path,
                )
            }),
        )
        .layer(prometheus_layer)
        .layer(sentry_tower::NewSentryLayer::<Request>::new_from_top())
        .layer(sentry_tower::SentryHttpLayer::with_transaction());
    let listener = TcpListener::bind(&*global.cfg.http.listen)
        .await
        .with_context(|| format!("Binding to TCP {:?}", global.cfg.http.listen))?;
    info!("Starting HTTP server on {:?}", global.cfg.http.listen);
    axum::serve(listener, app)
        .await
        .context("Starting Axum Server")?;
    Ok(())
}
