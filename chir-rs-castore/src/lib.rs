//! Content addressed store for chir.rs

use std::sync::Arc;

use aws_config::{AppName, Region, SdkConfig};
use aws_sdk_s3::{config::Credentials, Client};
use chir_rs_config::ChirRs;
use eyre::{Context as _, Result};
use tokio::fs::read_to_string;

/// Loads the AWS SDK config from the configuration file
async fn get_aws_config(config: &Arc<ChirRs>) -> Result<SdkConfig> {
    let access_key_id = read_to_string(&config.s3.access_key_id_file).await?;
    let secret_access_key = read_to_string(&config.s3.secret_access_key_file).await?;

    Ok(aws_config::from_env()
        .region(Region::new(config.s3.region.clone()))
        .endpoint_url(&config.s3.endpoint)
        .credentials_provider(Credentials::new(
            access_key_id,
            secret_access_key,
            None,
            None,
            "chir.rs configuration file",
        ))
        .app_name(AppName::new("chir-rs").context("Valid app name")?)
        .load()
        .await)
}

/// Content Addressed Data Store
#[derive(Clone, Debug)]
pub struct CaStore {
    /// Inner client
    client: Arc<Client>,
}

impl CaStore {
    /// Creates a new CA Store client
    ///
    /// # Errors
    ///
    /// This function returns an error if the access or secret access key cannot be read.
    pub async fn new(config: &Arc<ChirRs>) -> Result<Self> {
        let sdk_config = get_aws_config(config).await?;
        Ok(Self {
            client: Arc::new(Client::new(&sdk_config)),
        })
    }
}
