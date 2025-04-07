//! Main entrypoint for the chir-rs web server

use core::str::FromStr;
use std::sync::Arc;

use castore::CaStore;
use config::ChirRs;
use db::Database;
use eyre::{Context, Result};
use queue::Queue;
// implicitly used
use sentry_eyre as _;
use tokio::signal;
use tracing::error;
use tracing_error::ErrorLayer;
use tracing_subscriber::{
    fmt::format::JsonFields, layer::SubscriberExt as _, util::SubscriberInitExt as _, Layer,
};

pub mod castore;
pub mod config;
pub mod db;
pub mod gemini;
pub mod http;
pub mod queue;
pub mod robots;

/// Global raccomponents
#[derive(Debug)]
pub struct Global {
    /// Racconfiguration file
    pub cfg: ChirRs,
    /// Database
    pub db: Database,
    /// Raccontent-addressed store
    pub castore: CaStore,
    /// Queue
    pub queue: Queue,
}

/// Initializes logging for the application
fn init_logging(cfg: &ChirRs) -> Result<()> {
    let log_filter = tracing_subscriber::EnvFilter::from_str(&cfg.logging.log_level)
        .with_context(|| format!("Setting log filter to {}", cfg.logging.log_level))?;

    match cfg.logging.log_style {
        config::LogFormat::Full => {
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
        config::LogFormat::Compact => {
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
        config::LogFormat::Pretty => {
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
        config::LogFormat::Json => {
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
            let db = db::open_database(&cfg.database.path).await?;
            let castore = CaStore::new(&cfg).await?;
            let global = Arc::new_cyclic(|global| Global {
                cfg,
                db,
                castore,
                queue: Queue::new(global),
            });
            let global2 = Arc::clone(&global);
            let global3 = Arc::clone(&global);
            let global4 = Arc::clone(&global);
            let mut jobs = vec![
                tokio::spawn(db::session::expire_sessions_job(Arc::clone(&global))),
                tokio::spawn(CaStore::clean_task(Arc::clone(&global))),
                tokio::spawn(Queue::cleanup_task(Arc::clone(&global))),
                tokio::spawn(async move {
                    if let Err(e) = Queue::db_event_listener(global3).await {
                        error!("Failed to register event listener: {e:?}");
                    }
                }),
                tokio::spawn(async move {
                    if let Err(e) = http::main(global2).await {
                        error!("Failing to start HTTP Server: {e:?}");
                    }
                }),
                tokio::spawn(async move {
                    if let Err(e) = gemini::main(global4).await {
                        error!("Failing to start Gemini Server: {e:?}");
                    }
                }),
            ];

            for _ in 0..std::thread::available_parallelism()
                .map(usize::from)
                .unwrap_or(4)
            {
                jobs.push(tokio::spawn(Queue::run(Arc::clone(&global))));
            }

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
