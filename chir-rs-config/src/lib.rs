//! Configuration file support

use std::{
    io::IsTerminal,
    net::SocketAddr,
    path::{Path, PathBuf},
};

use eyre::{self, Context, Result};
use sentry_core::types::Dsn;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Returns the default logging level.
///
/// This is set to `debug` on debug builds and `warn` on release builds.
#[must_use]
fn default_log_level() -> String {
    #[cfg(debug_assertions)]
    {
        "debug".to_string()
    }
    #[cfg(not(debug_assertions))]
    {
        "warn".to_string()
    }
}

/// The Logging format to use
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogFormat {
    /// Full human-readable logging output
    Full,
    /// Compact single-line logging format
    Compact,
    /// Pretty and sparse logging output, intended for development.
    Pretty,
    /// JSON output
    Json,
}

impl Default for LogFormat {
    fn default() -> Self {
        if std::io::stdout().is_terminal() {
            #[cfg(debug_assertions)]
            {
                Self::Pretty
            }
            #[cfg(not(debug_assertions))]
            {
                Self::Full
            }
        } else {
            Self::Json
        }
    }
}

/// Configuration for monitoring and logging
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct LoggingConfig {
    /// Sentry DSN
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub sentry_dsn: Option<Dsn>,
    /// Log Level to output to stdout.
    /// By default, this is `"debug"` in debug builds and `"warn"` in release builds.
    #[serde(default = "default_log_level")]
    pub log_level: String,
    /// Logging style used by tracing.
    ///
    /// There are three default values:
    ///
    /// - If the log output is not a tty (for example systemd-journald or a file), the log output will be [`LogFormat::Json`]
    /// - If the log output is a tty AND the program runs in debug mode, the log output will be [`LogFormat::Pretty`]
    /// - If the log output is a tty AND the program runs in release mode, the log output will be [`LogFormat::Full`]
    #[serde(default)]
    pub log_style: LogFormat,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            sentry_dsn: None,
            log_level: default_log_level(),
            log_style: LogFormat::default(),
        }
    }
}

/// Deserializer for either one socket address or multiple.
fn deserialize_socket_addrs<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<SocketAddr>, D::Error> {
    /// Internal representation of socket addresses
    ///
    /// It’s either a single one, or a list of them
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum OneOrVec {
        /// Just a single socket address
        One(SocketAddr),
        /// List of socket addresses
        Vec(Vec<SocketAddr>),
    }
    let one_or_vec: OneOrVec = OneOrVec::deserialize(d)?;

    match one_or_vec {
        OneOrVec::One(socket_addr) => Ok(vec![socket_addr]),
        OneOrVec::Vec(vec) => Ok(vec),
    }
}

fn serialize_socket_addrs<S: Serializer>(
    socket_addrs: &Vec<SocketAddr>,
    s: S,
) -> Result<S::Ok, S::Error> {
    if socket_addrs.len() == 1 {
        socket_addrs[0].serialize(s)
    } else {
        socket_addrs.serialize(s)
    }
}

/// Returns the default socket addresses to use for http
#[expect(clippy::expect_used, reason = "Expect on hard-coded string literal")]
fn default_http_socket_addrs() -> Vec<SocketAddr> {
    vec!["[::1]:5621"
        .parse()
        .expect("Hard coded string literal, should never happen")]
}

/// Returns the default socket addresses to use for gemini
#[expect(clippy::expect_used, reason = "Expect on hard-coded string literal")]
fn default_gemini_socket_addrs() -> Vec<SocketAddr> {
    vec!["[::]:1965"
        .parse()
        .expect("Hard coded string literal, should never happen")]
}

/// Configuration used for axum’s HTTP config
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Http {
    /// Which IP addresses and ports to bind to
    ///
    /// The default is `[::1]:5621`.
    #[serde(serialize_with = "serialize_socket_addrs")]
    #[serde(deserialize_with = "deserialize_socket_addrs")]
    #[serde(default = "default_http_socket_addrs")]
    pub listen: Vec<SocketAddr>,
}

impl Default for Http {
    fn default() -> Self {
        Self {
            listen: default_http_socket_addrs(),
        }
    }
}

/// Default gemini hostname
fn default_gemini_host() -> String {
    "lotte.chir.rs".to_string()
}

/// Configuration used for the gemini server
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Gemini {
    /// Which IP addresses and ports to bind to
    ///
    /// The default is `[::]:1965`.
    #[serde(serialize_with = "serialize_socket_addrs")]
    #[serde(deserialize_with = "deserialize_socket_addrs")]
    #[serde(default = "default_gemini_socket_addrs")]
    pub listen: Vec<SocketAddr>,
    /// Host name to run under
    #[serde(default = "default_gemini_host")]
    pub host: String,
    /// Path to the private key
    pub private_key: PathBuf,
    /// Path to the certificate
    pub certificate: PathBuf,
}

/// Database configuration file
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Database {
    /// Path to the database
    pub path: String,
}

/// S3 configuration
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct S3Config {
    /// S3 endpoint url
    pub endpoint: String,
    /// S3 Region name
    pub region: String,
    /// File containing the access key id
    pub access_key_id_file: PathBuf,
    /// File containing the secret access key
    pub secret_access_key_file: PathBuf,
    /// Bucket name
    pub bucket: String,
}

/// Root configuration file
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ChirRs {
    /// Logging and monitoring related settings
    #[serde(default)]
    pub logging: LoggingConfig,
    /// HTTP Configuration
    #[serde(default)]
    pub http: Http,
    /// Gemini Configuration
    pub gemini: Gemini,
    /// Database Configuration
    pub database: Database,
    /// S3 configuration
    pub s3: S3Config,
}

impl ChirRs {
    /// Reads chir.rs configuration from file
    ///
    /// # Errors
    /// This function returns an error if the path cannot be read, doesn’t contain valid UTF-8 text, or TOML in the expected configuration format.
    pub fn read(fname: impl AsRef<Path>) -> Result<Self> {
        let config = std::fs::read_to_string(fname.as_ref())
            .with_context(|| format!("Reading config file {:?}", fname.as_ref()))?;
        toml::de::from_str(&config)
            .with_context(|| format!("Deserializing config file {:?}", fname.as_ref()))
    }

    /// Reads chir.rs configuration from a file pointed to by the `CHIR_RS_CONFIG` environment variable
    ///
    /// # Errors
    /// Like [`ChirRs::read`], but it also returns an error if the value of `CHIR_RS_CONFIG` is missing or invalid.
    pub fn read_from_env() -> Result<Self> {
        let fname = std::env::var("CHIR_RS_CONFIG")
            .context("Reading CHIR_RS_CONFIG environment variable")?;
        Self::read(fname)
    }
}
