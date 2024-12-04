use std::collections::HashSet;

use chir_rs_http_api::{
    auth::{LoginRequest, PasetoToken, Scope},
    errors::APIError,
};
use clap::{arg, Parser, Subcommand};
use eyre::Result;

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

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install().ok();
    tracing_subscriber::fmt::init();
    let matches = Args::parse();

    match matches.command {
        Command::Login { username, password } => login(matches.url, username, password).await?,
    }

    Ok(())
}
