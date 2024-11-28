//! Main entrypoint for the chir-rs web server

use core::str::FromStr;
use std::sync::Arc;

use chir_rs_config::ChirRs;
use eyre::{Context, Result};
// implicitly used
use sentry_eyre as _;
use tokio::try_join;
use tracing_error::ErrorLayer;
use tracing_subscriber::{
    fmt::format::JsonFields, layer::SubscriberExt as _, util::SubscriberInitExt as _, Layer,
};

fn main() -> Result<()> {
    color_eyre::install().ok();
    dotenvy::dotenv().ok();

    // NO THREADS BEFORE THIS POINT

    let cfg = ChirRs::read_from_env().context("Reading chir.rs configuration")?;

    let _guard = sentry::init(sentry::ClientOptions {
        dsn: cfg.logging.sentry_dsn.clone(),
        release: sentry::release_name!(),
        traces_sample_rate: 1.0,
        attach_stacktrace: true,
        debug: cfg!(debug_assertions),
        ..Default::default()
    });

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

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .context("Building thread pool for tokio")?
        .block_on(async move {
            let cfg = Arc::new(cfg);
            let db = chir_rs_db::open_database(&cfg.database.path).await?;
            let castore = chir_rs_castore::CaStore::new(&cfg).await?;
            try_join!(
                chir_rs_http::main(Arc::clone(&cfg), db.clone(), castore.clone()),
                chir_rs_gemini::main(Arc::clone(&cfg), db.clone(), castore.clone())
            )
            .context("Starting server components")?;
            Ok::<_, eyre::Report>(())
        })
        .context("Running chir.rs")
}
