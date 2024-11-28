//! HTTP server implementation for chir-rs

use std::sync::Arc;

use axum::{
    extract::{MatchedPath, Request, State},
    http::{Response, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use axum_prometheus::PrometheusMetricLayer;
use chir_rs_castore::CaStore;
use chir_rs_config::ChirRs;
use chir_rs_db::{file::File, Database};
use chir_rs_http_api::{axum::bincode::Bincode, readiness::ReadyState};
use eyre::{Context, Result};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::{error, info, info_span};

/// Application state
#[derive(Clone, Debug)]
pub struct AppState {
    /// Database handle
    pub db: Database,
    /// CA store handle
    pub ca: CaStore,
}

async fn root_handler(State(state): State<AppState>) -> impl IntoResponse {
    let res = state.ca.upload(b"Hewwo!".as_slice()).await.unwrap();

    info!("{res:?}");

    let res = state.ca.download(res).await.unwrap();

    Response::builder().body(res).unwrap()
}

/// Entrypoint for the HTTP server component
///
/// # Errors
/// This function returns an error if the startup of the server fails.
///
/// Errors it encounters during runtime should be automatically handled.
pub async fn main(cfg: Arc<ChirRs>, db: Database, castore: CaStore) -> Result<()> {
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
        .route("/", get(root_handler))
        .with_state(AppState { db, ca: castore })
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
