//! Gemini server implementation for chir.rs

use core::str;
use std::sync::Arc;

use bytes::BytesMut;
use eyre::{bail, Result};
use rustls::pki_types::{pem::PemObject, CertificateDer, PrivateKeyDer};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};
use tokio_rustls::TlsAcceptor;
use tracing::{error, info, instrument};
use url::Url;

use crate::{db::file::File, Global};

#[instrument(skip(stream))]
async fn handle_request<W>(req: Url, mut stream: W, global: Arc<Global>) -> Result<()>
where
    W: AsyncWriteExt + Unpin + Send,
{
    if req.host_str() != Some(&global.cfg.gemini.host) {
        stream.write_all(b"53\r\n").await?;
        return Ok(());
    }

    let mut chosen_file = File::get_by_path_mime(&global.db, req.path(), "text/gemini").await?;
    if chosen_file.is_none() {
        chosen_file = File::get_by_path(&global.db, req.path())
            .await?
            .first()
            .cloned();
    }

    let Some(chosen_file) = chosen_file else {
        stream.write_all(b"51\r\n").await?;
        return Ok(());
    };

    stream.write_all(b"20 ").await?;
    stream
        .write_all(chosen_file.mime.as_ref().as_bytes())
        .await?;
    stream.write_all(b"\r\n").await?;

    let (_, body) = global
        .castore
        .download_bytestream(chosen_file.b3hash)
        .await?;
    let mut body = body.into_async_read();

    let mut buf = BytesMut::with_capacity(16 * 1024 * 1024);
    loop {
        buf.clear();
        body.read_buf(&mut buf).await?;
        if buf.is_empty() {
            break;
        }
        stream.write_all(&buf).await?;
    }

    Ok(())
}

/// Parses a gemini request
///
/// # Errors
/// This function returns an error if the request is invalid.
fn parse_request(request: &[u8]) -> Result<Url> {
    if &request[request.len() - 2..] != b"\r\n" {
        bail!("Invalid request: {request:?} (missing CRLF)");
    }
    let url = str::from_utf8(&request[..request.len() - 2])?;
    let url = Url::parse(url)?;
    if !url.has_host() {
        bail!("Host required for request: {request:?}");
    }
    if url.scheme() != "gemini" {
        bail!("gemini scheme required for request: {request:?}");
    }
    Ok(url)
}

/// entrypoint for the gemini server
///
/// # Errors
///
/// This function returns an error if starting the gemini server fails
pub async fn main(global: Arc<Global>) -> Result<()> {
    let certs = CertificateDer::pem_file_iter(&global.cfg.gemini.certificate)?
        .collect::<Result<Vec<_>, _>>()?;
    let key = PrivateKeyDer::from_pem_file(&global.cfg.gemini.private_key)?;
    let config = rustls::ServerConfig::builder_with_provider(Arc::new(
        rustls::crypto::aws_lc_rs::default_provider(),
    ))
    .with_safe_default_protocol_versions()?
    .with_no_client_auth()
    .with_single_cert(certs, key)?;
    let acceptor = TlsAcceptor::from(Arc::new(config));
    let listener = TcpListener::bind(&*global.cfg.gemini.listen).await?;
    info!("Starting Gemini server on {:?}", global.cfg.gemini.listen);
    loop {
        let (stream, _peer_addr) = listener.accept().await?;
        let acceptor = acceptor.clone();
        let global2 = Arc::clone(&global);
        let fut = async move {
            let mut stream = acceptor.accept(stream).await?;
            let mut request = BytesMut::with_capacity(4096);
            stream.read_buf(&mut request).await?;
            let req = match parse_request(&request) {
                Ok(u) => u,
                Err(e) => {
                    info!("{e:?}");
                    stream.write_all(b"59\r\n").await?;
                    stream.shutdown().await?;
                    return Ok::<_, eyre::Report>(());
                }
            };

            handle_request(req, &mut stream, global2).await?;

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
