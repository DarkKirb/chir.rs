//! Main entrypoint for the chir-rs web server

use core::str::FromStr;
use std::sync::Arc;

use chir_rs_config::ChirRs;
use eyre::{Context, Result};
// implicitly used
use sentry_eyre as _;
use tokio::signal;
use tracing::error;
use tracing_error::ErrorLayer;
use tracing_subscriber::{
    fmt::format::JsonFields, layer::SubscriberExt as _, util::SubscriberInitExt as _, Layer,
};

/// Initializes logging for the application
fn init_logging(cfg: &ChirRs) -> Result<()> {
    let log_filter = tracing_subscriber::EnvFilter::from_str(&cfg.logging.log_level)
        .with_context(|| format!("Setting log filter to {}", cfg.logging.log_level))?;

    match cfg.logging.log_style {
        chir_rs_config::LogFormat::Full => {
            let log_format = tracing_subscriber::fmt::format();
            tracing_subscriber::registry()
                .with(ErrorLayer::default())
                .with(sentry_tracing::layer())
                .with(
                    tracing_subscriber::fmt::layer()
                        .event_format(log_format)
                        .with_filter(log_filter),
                )
                .init();
        }
        chir_rs_config::LogFormat::Compact => {
            let log_format = tracing_subscriber::fmt::format().compact();
            tracing_subscriber::registry()
                .with(ErrorLayer::default())
                .with(sentry_tracing::layer())
                .with(
                    tracing_subscriber::fmt::layer()
                        .event_format(log_format)
                        .with_filter(log_filter),
                )
                .init();
        }
        chir_rs_config::LogFormat::Pretty => {
            let log_format = tracing_subscriber::fmt::format().pretty();
            tracing_subscriber::registry()
                .with(ErrorLayer::default())
                .with(sentry_tracing::layer())
                .with(
                    tracing_subscriber::fmt::layer()
                        .event_format(log_format)
                        .with_filter(log_filter),
                )
                .init();
        }
        chir_rs_config::LogFormat::Json => {
            let log_format = tracing_subscriber::fmt::format().json();
            tracing_subscriber::registry()
                .with(ErrorLayer::default())
                .with(sentry_tracing::layer())
                .with(
                    tracing_subscriber::fmt::layer()
                        .event_format(log_format)
                        .fmt_fields(JsonFields::new())
                        .with_filter(log_filter),
                )
                .init();
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    color_eyre::install().ok();
    dotenvy::dotenv().ok();

    // NO THREADS BEFORE THIS POINT

    let cfg = ChirRs::read_from_env().context("Reading chir.rs configuration")?;

    let _guard = sentry::init(sentry::ClientOptions {
        dsn: cfg.logging.sentry_dsn.clone(),
        release: sentry::release_name!(),
        traces_sample_rate: 0.1,
        attach_stacktrace: true,
        debug: cfg!(debug_assertions),
        ..Default::default()
    });

    init_logging(&cfg)?;

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .context("Building thread pool for tokio")?
        .block_on(async move {
            let cfg = Arc::new(cfg);
            let db = chir_rs_db::open_database(&cfg.database.path).await?;
            let castore = chir_rs_castore::CaStore::new(&cfg).await?;
            let cfg1 = Arc::clone(&cfg);
            let cfg2 = Arc::clone(&cfg);
            let db1 = db.clone();
            let db2 = db.clone();
            let castore1 = castore.clone();
            let castore2 = castore.clone();
            let jobs = [
                tokio::spawn(chir_rs_db::session::expire_sessions_job(db.clone())),
                tokio::spawn(async move {
                    if let Err(e) = chir_rs_http::main(cfg1, db1, castore1).await {
                        error!("Failing to start HTTP Server: {e:?}");
                    }
                }),
                tokio::spawn(async move {
                    if let Err(e) = chir_rs_gemini::main(cfg2, db2, castore2).await {
                        error!("Failing to start Gemini Server: {e:?}");
                    }
                }),
            ];

            signal::ctrl_c()
                .await
                .context("Trying to register ctrl+c handler")?;

            for job in jobs {
                job.abort();
                if let Err(e) = job.await {
                    if e.is_panic() {
                        error!("Failed running job: {e:?}");
                    }
                }
            }
            Ok::<_, eyre::Report>(())
        })
        .context("Running chir.rs")
}
