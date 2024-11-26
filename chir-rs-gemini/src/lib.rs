//! Gemini server implementation for chir.rs

use std::sync::Arc;

use bytes::BytesMut;
use chir_rs_config::ChirRs;
use chir_rs_db::Database;
use eyre::Result;
use rustls::pki_types::{pem::PemObject, CertificateDer, PrivateKeyDer};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};
use tokio_rustls::TlsAcceptor;
use tracing::{error, info};

/// entrypoint for the gemini server
///
/// # Errors
///
/// This function returns an error if starting the gemini server fails
pub async fn main(cfg: Arc<ChirRs>, _: Database) -> Result<()> {
    let certs =
        CertificateDer::pem_file_iter(&cfg.gemini.certificate)?.collect::<Result<Vec<_>, _>>()?;
    let key = PrivateKeyDer::from_pem_file(&cfg.gemini.private_key)?;
    let config = rustls::ServerConfig::builder_with_provider(Arc::new(
        rustls::crypto::aws_lc_rs::default_provider(),
    ))
    .with_safe_default_protocol_versions()?
    .with_no_client_auth()
    .with_single_cert(certs, key)?;
    let acceptor = TlsAcceptor::from(Arc::new(config));
    let listener = TcpListener::bind(&*cfg.gemini.listen).await?;
    info!("Starting Gemini server on {:?}", cfg.gemini.listen);
    loop {
        let (stream, _peer_addr) = listener.accept().await?;
        let acceptor = acceptor.clone();
        let fut = async move {
            let mut stream = acceptor.accept(stream).await?;
            let mut request = BytesMut::with_capacity(4096);
            stream.read_buf(&mut request).await?;
            println!("{request:?}");
            stream.write_all(b"51\r\n").await?;
            stream.shutdown().await?;
            Ok::<_, eyre::Report>(())
        };
        tokio::spawn(async move {
            if let Err(err) = fut.await {
                error!("Failed to handle request: {err:?}");
            }
        });
    }
}
