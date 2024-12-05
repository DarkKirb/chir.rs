use std::{collections::HashSet, future::Future, path::Path, pin::Pin};

use chir_rs_http_api::{
    auth::{LoginRequest, PasetoToken, Scope},
    errors::APIError,
};
use clap::{arg, Parser, Subcommand};
use eyre::{eyre, Context as _, OptionExt as _, Result};
use mime_guess::{Mime, MimeGuess};
use reqwest::Body;
use tokio::join;
use tracing::instrument;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "http://localhost:5621/")]
    url: String,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Login {
        #[arg(short, long)]
        username: String,
        #[arg(short, long)]
        password: String,
    },
    Upload {
        #[arg(short, long)]
        source: String,
        #[arg(short, long)]
        dest: String,
    },
    UploadDir {
        #[arg(short, long)]
        source: String,
        #[arg(short, long)]
        dest: String,
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
        println!("{:?}", response);
    }
    Ok(())
}

#[instrument(skip(source))]
async fn upload(url: String, source: impl AsRef<Path>, dest: String) -> Result<()> {
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
    if !res.status().is_success() {
        let response = res.bytes().await?;
        let response: APIError =
            bincode::decode_from_slice(&response, bincode::config::standard())?.0;
        Err(response).with_context(|| format!("Uploading to {dest}"))?;
    }
    Ok(())
}

#[instrument(skip(source))]
async fn upload_dir(url: String, source: impl AsRef<Path>, dest: String) -> Result<()> {
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
            let sub_fut: Pin<Box<dyn Future<Output = Result<()>>>> =
                Box::pin(upload_dir(url.clone(), ent.path(), tgt));
            sub_fut.await?;
            continue;
        }
        if file_name_str == "index.html" {
            if dest.is_empty() {
                upload(url.clone(), ent.path(), "".to_string()).await?;
            } else {
                upload(url.clone(), ent.path(), format!("{dest}/")).await?;
            }
        }
        upload(url.clone(), ent.path(), tgt).await?;
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
    }

    Ok(())
}
