//! Content addressed store for chir.rs

use std::sync::Arc;

use aws_config::{AppName, Region, SdkConfig};
use aws_sdk_s3::{
    config::Credentials,
    primitives::{ByteStream, SdkBody},
    Client,
};
use blake3::{Hash, Hasher};
use bytes::Bytes;
use chir_rs_common::lexicographic_base64;
use educe::Educe;
use eyre::{Context as _, Result};
use rand::Rng;
use stretto::AsyncCache;
use tokio::fs::read_to_string;
use tokio::time::Duration;
use tracing::{error, info, instrument};

use crate::{
    config::ChirRs,
    db::{file::File, Database},
    queue::QueueActionResult,
    Global,
};

/// Loads the AWS SDK config from the configuration file
async fn get_aws_config(config: &ChirRs) -> Result<SdkConfig> {
    let access_key_id = read_to_string(&config.s3.access_key_id_file).await?;
    let secret_access_key = read_to_string(&config.s3.secret_access_key_file).await?;

    Ok(aws_config::from_env()
        .region(Region::new(config.s3.region.clone()))
        .endpoint_url(&config.s3.endpoint)
        .credentials_provider(Credentials::new(
            access_key_id.trim(),
            secret_access_key.trim(),
            None,
            None,
            "chir.rs configuration file",
        ))
        .app_name(AppName::new("chir-rs").context("Valid app name")?)
        .load()
        .await)
}

/// Content Addressed Data Store
#[derive(Clone, Educe)]
#[educe(Debug)]
pub struct CaStore {
    /// Inner client
    #[educe(Debug(ignore))]
    client: Client,
    /// Bucket
    bucket: String,
    /// CA Value Cache
    #[educe(Debug(ignore))]
    cache: AsyncCache<Hash, Bytes>,
}

impl CaStore {
    /// Creates a new CA Store client
    ///
    /// # Errors
    ///
    /// This function returns an error if the access or secret access key cannot be read.
    pub async fn new(config: &ChirRs) -> Result<Self> {
        let sdk_config = get_aws_config(config).await?;
        Ok(Self {
            client: Client::new(&sdk_config),
            bucket: config.s3.bucket.clone(),
            cache: AsyncCache::new(
                (config.cache_max_size / 1_000)
                    .try_into()
                    .context("Cache size too large")?,
                config
                    .cache_max_size
                    .try_into()
                    .context("Value too large")?,
                tokio::spawn,
            )?,
        })
    }

    /// Uploads a file to the CA store backend and returns its hash
    ///
    /// # Errors
    ///
    /// This function returns an error if reading the source stream fails, uploading the source stream fails, or moving the file to its correct content-addressed position fails.
    #[instrument(skip(data))]
    pub async fn upload(&self, data: Vec<u8>) -> Result<Hash> {
        let mut hasher = Hasher::new();
        hasher.update_rayon(&data);
        let hash = hasher.finalize();
        let target_fname = lexicographic_base64::encode(hash.as_bytes());

        self.client
            .put_object()
            .bucket(&*self.bucket)
            .key(&target_fname)
            .body(ByteStream::from(data))
            .send()
            .await
            .context("Uploading file")?;

        Ok(hash)
    }

    /// Deletes a file from the CA store backend with its hash
    ///
    /// # Errors
    ///
    /// This function fails if deleting the object fails
    #[instrument]
    pub async fn delete_object(&self, hash: Hash) -> Result<()> {
        let key = lexicographic_base64::encode(hash.as_bytes());
        self.cache.remove(&hash).await;
        self.client
            .delete_object()
            .bucket(&*self.bucket)
            .key(&key)
            .send()
            .await
            .with_context(|| format!("Deleting content-addressed file {key}"))?;

        Ok(())
    }

    /// Downloads a file from the CA store backend with its hash
    ///
    /// # Errors
    ///
    /// This function returns an error if loading file matadata fails
    #[instrument]
    pub async fn download_bytestream(&self, hash: Hash) -> Result<(Option<i64>, ByteStream)> {
        #[allow(
            clippy::significant_drop_in_scrutinee,
            reason = "We are cloning like 1 arc lol"
        )]
        if let Some(v) = self.cache.get(&hash).await {
            info!("{hash:?} found in cache. Returning");
            let data = v.as_ref();
            return Ok((
                Some(data.len().try_into().context("Bad api design")?),
                data.clone().into(),
            ));
        }
        let key = lexicographic_base64::encode(hash.as_bytes());
        let file = self
            .client
            .get_object()
            .bucket(&*self.bucket)
            .key(&key)
            .send()
            .await
            .with_context(|| format!("Downloading content-addressed file {key}"))?;
        let file_size_hint = file.content_length.unwrap_or(i64::MAX);
        let file_size_hint: u64 = file_size_hint.try_into().unwrap_or(u64::MAX);
        if file_size_hint < 1_000_000 {
            // Cache this
            let data = file.body.collect().await?.into_bytes();

            self.cache
                .insert(
                    hash,
                    data.clone(),
                    data.len().try_into().context("Bad api design")?,
                )
                .await;

            Ok((
                Some(data.len().try_into().context("Bad api design")?),
                data.into(),
            ))
        } else {
            Ok((file.content_length, file.body))
        }
    }

    /// Downloads a file from the CA store backend with its hash
    ///
    /// # Errors
    ///
    /// This function returns an error if loading file matadata fails
    #[instrument]
    pub async fn download(&self, hash: Hash) -> Result<(Option<i64>, SdkBody)> {
        let (length, body) = self.download_bytestream(hash).await?;
        Ok((length, body.into_inner()))
    }

    /// Get all of the files stored
    async fn get_all_files(&self) -> Result<Vec<String>> {
        let mut marker: Option<String> = None;
        let mut files = Vec::new();
        loop {
            let mut objects = self.client.list_objects().bucket(&*self.bucket);

            if let Some(marker) = &marker {
                objects = objects.marker(marker.clone());
            }

            let objects = objects.send().await?;

            marker = objects.next_marker().map(ToString::to_string);
            files.extend(
                objects
                    .contents()
                    .iter()
                    .filter_map(|o| o.key())
                    .map(ToOwned::to_owned),
            );
            if marker.is_none() {
                break;
            }
        }
        Ok(files)
    }

    /// Run a single clean cycle
    ///
    /// # Errors
    /// This function returns an error if cleaning the content addressed store fails
    async fn clean_once(&self, db: &Database) -> Result<()> {
        for file in self.get_all_files().await? {
            let should_delete = if file.contains('/') {
                true
            } else {
                let hash = lexicographic_base64::decode(&file)?;
                let mut hash2 = [0u8; 32];
                if hash.len() == 32 {
                    hash2.copy_from_slice(&hash);
                    let hash = Hash::from_bytes(hash2);
                    !File::is_used(db, hash).await.unwrap_or(true)
                } else {
                    true
                }
            };
            if should_delete {
                info!("Deleting unused file {file}");
                self.client
                    .delete_object()
                    .bucket(&*self.bucket)
                    .key(file)
                    .send()
                    .await?;
            }
        }
        Ok(())
    }

    /// Run the periodic CA store raccleanup task
    pub async fn clean_task(global: Arc<Global>) {
        info!("Starting CA clean thread");
        loop {
            info!("Deleting unused objects");
            if let Err(e) = global.castore.clean_once(&global.db).await {
                error!("Failed to delete unused files: {e:?}");
            }
            let secs_to_sleep = rand::rng().random_range(1800..=5400);
            info!("Done. Sleeping for {secs_to_sleep}s");
            tokio::time::sleep(Duration::from_secs(secs_to_sleep)).await;
        }
    }
}

/// Queue action for uploading a CA path
///
/// # Errors
/// This function returns an error if uploading fails
#[instrument(skip(global))]
pub async fn upload_ca(data: &[u8], global: &Arc<Global>) -> Result<QueueActionResult> {
    let hash = global.castore.upload(data.to_vec()).await?;
    Ok(QueueActionResult::CAPath(*hash.as_bytes()))
}
