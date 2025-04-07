//! Standalone API racclient for chir-rs

use std::{collections::HashSet, path::Path};

use chir_rs_common::http_api::{
    auth::{LoginRequest, PasetoToken, Scope},
    errors::APIError,
    robots::RobotsRule,
};
use clap::{arg, Parser, Subcommand};
use eyre::{eyre, Context as _, Result};
use mime_guess::MimeGuess;
use reqwest::Body;
use tracing::{info, instrument};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
/// Arguments for the chir.rs api racclient
struct Args {
    /// The base URL for chir.rs
    #[arg(short, long, default_value = "https://lotte.chir.rs/")]
    url: String,
    /// The raccommand to use
    #[command(subcommand)]
    command: Command,
}

/// Raccommand selection
#[derive(Subcommand, Debug)]
enum Command {
    /// Login raccommand
    Login {
        /// Username
        #[arg(short, long)]
        username: String,
        /// Password
        #[arg(short, long)]
        password: String,
    },
    /// Upload a single file
    Upload {
        #[arg(short, long)]
        /// Local source of the file
        source: String,
        /// Destination of the file
        #[arg(short, long)]
        ///
        dest: String,
    },
    /// Upload directory
    UploadDir {
        /// Local source of the directory
        #[arg(short, long)]
        source: String,
        /// Destination to upload it to
        #[arg(short, long)]
        dest: String,
    },
    /// Add robots.txt line
    AddRobots {
        /// Bot useragent name
        #[arg(short, long)]
        bot: String,
        /// Path to manage
        #[arg(short, long)]
        path: String,
        /// Whether to allow or deny
        #[arg(short, long, action)]
        allow: bool,
    },
}

async fn login(url: String, username: String, password: String) -> Result<()> {
    let request = LoginRequest {
        username,
        password,
        scopes: HashSet::from([Scope::Full]),
    };
    let request = bincode::encode_to_vec(request, bincode::config::standard())?;
    let client = reqwest::Client::new();
    let res = client
        .post(format!("{url}.api/auth/login"))
        .header("Content-Type", "application/x+bincode")
        .header("Accept", "application/x+bincode")
        .body(request)
        .send()
        .await?;

    let status = res.status();

    let response = res.bytes().await?;

    if status.is_success() {
        let response: PasetoToken =
            bincode::decode_from_slice(&response, bincode::config::standard())?.0;
        println!("{}", response.to_paseto());
    } else {
        let response: APIError =
            bincode::decode_from_slice(&response, bincode::config::standard())?.0;
        println!("{response:?}");
    }
    Ok(())
}

#[instrument(skip(source))]
async fn upload(url: String, source: impl AsRef<Path> + Send + Sync, dest: String) -> Result<()> {
    let client = reqwest::Client::new();
    let token = std::env::var("CHIR_RS_TOKEN")?;
    let file = tokio::fs::File::open(&source).await?;
    let res = client
        .post(format!("{url}{dest}"))
        .header(
            "Content-Type",
            MimeGuess::from_path(&source)
                .first()
                .unwrap_or(mime::APPLICATION_OCTET_STREAM)
                .to_string(),
        )
        .header("Authorization", format!("Bearer {token}"))
        .body(Body::from(file))
        .send()
        .await?;
    info!("Finished uploading {dest}");
    if !res.status().is_success() {
        let response = res.bytes().await?;
        let response: APIError =
            bincode::decode_from_slice(&response, bincode::config::standard())?.0;
        Err(response).with_context(|| format!("Uploading to {dest}"))?;
    }
    Ok(())
}

#[instrument(skip(source))]
async fn upload_dir(
    url: String,
    source: impl AsRef<Path> + Send + Sync,
    dest: String,
) -> Result<()> {
    let mut dir = tokio::fs::read_dir(source).await?;
    while let Some(ent) = dir.next_entry().await? {
        let file_type = ent.file_type().await?;
        let file_name_str = ent
            .file_name()
            .into_string()
            .map_err(|_| eyre!("Invalid file name encountered"))?;
        let tgt = if dest.is_empty() {
            file_name_str.clone()
        } else {
            format!("{dest}/{file_name_str}")
        };
        if file_type.is_dir() {
            Box::pin(upload_dir(url.clone(), ent.path(), tgt)).await?;
            continue;
        }
        if file_name_str == "index.html" {
            if !dest.is_empty() {
                upload(url.clone(), ent.path(), format!("{dest}/")).await?;
            }
            upload(url.clone(), ent.path(), dest.clone()).await?;
        }
        upload(url.clone(), ent.path(), tgt).await?;
    }
    Ok(())
}

#[instrument]
async fn add_robots(url: String, bot: String, path: String, allow: bool) -> Result<()> {
    let client = reqwest::Client::new();
    let token = std::env::var("CHIR_RS_TOKEN")?;
    let request = RobotsRule {
        id: 0,
        user_agent: bot,
        path,
        allow,
    };
    let request = bincode::encode_to_vec(request, bincode::config::standard())?;
    let res = client
        .post(format!("{url}.api/robots"))
        .header("Content-Type", "application/x+bincode")
        .header("Accept", "application/x+bincode")
        .header("Authorization", format!("Bearer {token}"))
        .body(request)
        .send()
        .await?;
    let status = res.status();

    let response = res.bytes().await?;

    if status.is_success() {
        let response: RobotsRule =
            bincode::decode_from_slice(&response, bincode::config::standard())?.0;
        println!("{response:?}");
    } else {
        let response: APIError =
            bincode::decode_from_slice(&response, bincode::config::standard())?.0;
        println!("{response:?}");
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install().ok();
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();
    let matches = Args::parse();

    match matches.command {
        Command::Login { username, password } => login(matches.url, username, password).await?,
        Command::Upload { source, dest } => upload(matches.url, source, dest).await?,
        Command::UploadDir { source, dest } => upload_dir(matches.url, source, dest).await?,
        Command::AddRobots { bot, path, allow } => {
            add_robots(matches.url, bot, path, allow).await?;
        }
    }

    Ok(())
}
