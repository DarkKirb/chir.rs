//! Code for the chir.rs website

use std::env;

use anyhow::{Context, Result};
use axum::{routing::get, Router};
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tower_http::compression::CompressionLayer;
use tracing::{error, instrument};

pub mod err;
pub mod homepage;
pub mod lang;
pub mod static_file;
pub mod theming;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    #[instrument]
    async fn main() -> Result<()> {
        let app = Router::new()
            .route("/", get(homepage::homepage))
            .route("/static/:path", get(static_file::static_file))
            .route("/update_settings", get(homepage::update_settings))
            .layer(CookieManagerLayer::new())
            .layer(
                CompressionLayer::new()
                    .br(true)
                    .deflate(true)
                    .gzip(true)
                    .zstd(true),
            );
        let listener = TcpListener::bind(
            env::var("BIND_ADDR").context("Failed to read environment variable BIND_ADDR")?,
        )
        .await
        .context("failed to bind port for server")?;
        axum::serve(listener, app)
            .await
            .context("failed to start serving")?;
        Ok(())
    }

    if let Err(e) = main().await.context("Failed to run the server") {
        error!("{:?}", e);
        #[allow(clippy::exit)]
        {
            std::process::exit(1);
        }
    }
}
