//! HTTP server implementation for chir-rs

use std::{fmt::Debug, sync::Arc};

use axum::{
    extract::{MatchedPath, Request, State},
    http::StatusCode,
    routing::{get, post},
    Router,
};
use axum_prometheus::PrometheusMetricLayer;
use b64_ct::FromBase64;
use chir_rs_castore::CaStore;
use chir_rs_config::ChirRs;
use chir_rs_db::Database;
use chir_rs_http_api::{axum::bincode::Bincode, readiness::ReadyState};
use eyre::{bail, eyre, Context, Result};
use rusty_paseto::core::{Key, Local, PasetoSymmetricKey, V4};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::{error, info, info_span};

pub mod auth;
pub mod ca_server;

/// Application state
#[derive(Clone)]
pub struct AppState {
    /// Database handle
    pub db: Database,
    /// CA store handle
    pub ca: CaStore,
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
pub async fn main(cfg: Arc<ChirRs>, db: Database, castore: CaStore) -> Result<()> {
    let paseto_symmetric_key =
        tokio::fs::read_to_string(cfg.paseto_secret_key_file.clone()).await?;
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
                match state.db.ping().await {
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
        .route("/.api/auth/login", post(auth::password_login::login))
        .fallback(get(ca_server::serve_files))
        .with_state(AppState {
            db,
            ca: castore,
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
    let listener = TcpListener::bind(&*cfg.http.listen)
        .await
        .with_context(|| format!("Binding to TCP {:?}", cfg.http.listen))?;
    info!("Starting HTTP server on {:?}", cfg.http.listen);
    axum::serve(listener, app)
        .await
        .context("Starting Axum Server")?;
    Ok(())
}
