//! Content addressed store for chir.rs

use std::sync::Arc;

use aws_config::{AppName, Region, SdkConfig};
use aws_sdk_s3::{
    config::Credentials,
    primitives::{ByteStream, SdkBody},
    types::{CompletedMultipartUpload, CompletedPart},
    Client,
};
use blake3::{Hash, Hasher};
use bytes::{Bytes, BytesMut};
use chir_rs_config::ChirRs;
use chir_rs_misc::{id_generator, lexicographic_base64};
use educe::Educe;
use eyre::{Context as _, Result};
use stretto::{AsyncCache, AsyncCacheBuilder};
use tokio::{
    fs::read_to_string,
    io::{AsyncRead, AsyncReadExt},
    sync::Mutex,
    task::spawn_blocking,
    try_join,
};
use tracing::{debug, info, instrument};

/// Loads the AWS SDK config from the configuration file
async fn get_aws_config(config: &Arc<ChirRs>) -> Result<SdkConfig> {
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
    client: Arc<Client>,
    /// Bucket
    bucket: Arc<str>,
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
    pub async fn new(config: &Arc<ChirRs>) -> Result<Self> {
        let sdk_config = get_aws_config(config).await?;
        Ok(Self {
            client: Arc::new(Client::new(&sdk_config)),
            bucket: Arc::from(config.s3.bucket.as_ref()),
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
    #[instrument(skip(reader))]
    async fn upload_inner<R>(&self, reader: R, id: u128) -> Result<Hash>
    where
        R: AsyncRead + AsyncReadExt + Send,
    {
        let mut reader = Box::pin(reader);
        let string_id = lexicographic_base64::encode(id.to_be_bytes());

        info!("Starting multipart upload {id}");
        let source_fname = format!("temp/{string_id}");
        let multipart_result = self
            .client
            .create_multipart_upload()
            .bucket(&*self.bucket)
            .key(&source_fname)
            .send()
            .await
            .with_context(|| format!("Creating multipart request for Request ID{id}"))?;

        let mut buf = BytesMut::with_capacity(16 * 1024 * 1024); // 16MiB byte buffer for the file
        let hasher = Arc::new(Mutex::new(Hasher::new()));

        let mut i = 1;
        let mut completed_multipart_upload_builder = CompletedMultipartUpload::builder();

        loop {
            buf.clear();
            reader.read_buf(&mut buf).await.context("Reading chunk")?;
            if buf.is_empty() {
                break;
            }

            debug!("Uploading part {i} for multipart upload {id}");

            let buf2 = buf.clone();
            let hasher = Arc::clone(&hasher);
            let hasher_job = spawn_blocking(move || {
                hasher.blocking_lock().update_rayon(&buf2);
            });

            let part_upload_fut = self
                .client
                .upload_part()
                .bucket(&*self.bucket)
                .key(&source_fname)
                .set_upload_id(multipart_result.upload_id.clone())
                .body(ByteStream::from(buf.to_vec()))
                .part_number(i)
                .send();

            let ((), part_upload_result) = try_join!(
                async { hasher_job.await.context("Awaiting hasher job") },
                async { part_upload_fut.await.context("Awaiting uploader job") }
            )
            .context("Awaiting job for chunk")?;
            completed_multipart_upload_builder = completed_multipart_upload_builder.parts(
                CompletedPart::builder()
                    .e_tag(part_upload_result.e_tag.unwrap_or_default())
                    .part_number(i)
                    .build(),
            );
            i += 1;
        }

        debug!("Finalizing Multipart Upload {id}");

        let hash = hasher.lock().await.finalize();
        self.client
            .complete_multipart_upload()
            .bucket(&*self.bucket)
            .key(&source_fname)
            .multipart_upload(completed_multipart_upload_builder.build())
            .set_upload_id(multipart_result.upload_id)
            .send()
            .await
            .context("Completing multipart upload")?;

        let target_fname = lexicographic_base64::encode(hash.as_bytes());

        self.client
            .copy_object()
            .bucket(&*self.bucket)
            .copy_source(format!("{}/{source_fname}", self.bucket))
            .key(target_fname)
            .send()
            .await
            .context("Renaming temporary file")?;

        self.client
            .delete_object()
            .bucket(&*self.bucket)
            .key(source_fname)
            .send()
            .await
            .context("Deleting temporary file")?;

        Ok(hash)
    }

    /// Uploads a file to the CA store backend and returns its hash
    ///
    /// # Errors
    ///
    /// This function returns an error if reading the source stream fails, uploading the source stream fails, or moving the file to its correct content-addressed position fails.
    pub async fn upload<R>(&self, reader: R) -> Result<Hash>
    where
        R: AsyncRead + AsyncReadExt + Send,
    {
        let id = id_generator::generate();

        self.upload_inner(reader, id).await
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
}
