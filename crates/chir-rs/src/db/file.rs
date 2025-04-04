//! File related APIs

use std::{fmt::Formatter, sync::LazyLock};

use bincode::{error::DecodeError, Decode, Encode};
use blake3::Hash;
use chrono::{DateTime, Duration, Utc};
use dashmap::DashMap;
use eyre::Context as _;
use eyre::Result;
use mime::Mime;
use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};
use sqlx::{postgres::PgRow, prelude::FromRow, query_as};
use sqlx::{query, Row as _};
use tracing::{error, instrument};

use super::Database;

/// Value for the file association cache
#[derive(Debug, Clone)]
struct CacheValue {
    /// The file that is cached
    file: File,
    /// The last time the association has been verified
    last_checked: DateTime<Utc>,
}

/// Value for the file-mime association cache
#[derive(Debug, Clone)]
struct CacheValueVec {
    /// The files that are cached
    file: Vec<File>,
    /// The last time the association has been verified
    last_checked: DateTime<Utc>,
}

/// Serializes a mime type to string
fn serialize_mime<S: Serializer>(mime: &Mime, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_str(mime.as_ref())
}

/// Deserializes a mime type from string
fn deserialize_mime<'de, D: Deserializer<'de>>(d: D) -> Result<Mime, D::Error> {
    /// Helper struct for parsing
    struct JsonStringVisitor;

    impl Visitor<'_> for JsonStringVisitor {
        type Value = Mime;
        fn expecting(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
            formatter.write_str("a string containing a mime type")
        }
        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            v.parse().map_err(E::custom)
        }
    }

    d.deserialize_any(JsonStringVisitor)
}

/// File record
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct File {
    /// ID of the file record
    id: u64,
    /// Path this file is mounted at
    pub file_path: String,
    /// MIME type of file
    #[serde(serialize_with = "serialize_mime")]
    #[serde(deserialize_with = "deserialize_mime")]
    pub mime: Mime,
    /// blake3 hash of the file to serve
    pub b3hash: Hash,
}

impl Encode for File {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> std::result::Result<(), bincode::error::EncodeError> {
        self.id.encode(encoder)?;
        self.file_path.encode(encoder)?;
        self.mime.as_ref().encode(encoder)?;
        self.b3hash.as_bytes().encode(encoder)
    }
}

impl Decode<()> for File {
    fn decode<D: bincode::de::Decoder>(decoder: &mut D) -> std::result::Result<Self, DecodeError> {
        let id = u64::decode(decoder)?;
        let file_path = String::decode(decoder)?;
        let mime = match String::decode(decoder)?.parse() {
            Ok(v) => v,
            Err(e) => {
                return Err(DecodeError::Io {
                    inner: std::io::Error::other(e),
                    additional: 0,
                });
            }
        };
        let b3hash = <[u8; 32]>::decode(decoder)?;
        Ok(Self {
            id,
            file_path,
            mime,
            b3hash: Hash::from_bytes(b3hash),
        })
    }
}

impl<'r> FromRow<'r, PgRow> for File {
    fn from_row(row: &'r PgRow) -> std::result::Result<Self, sqlx::Error> {
        let id = u64::try_from(row.try_get::<i64, _>("id")?).unwrap_or_default();
        let file_path: String = row.try_get("file_path")?;
        let mime: Mime = match row.try_get::<&str, _>("mime")?.parse() {
            Ok(v) => v,
            Err(e) => return Err(sqlx::Error::Decode(Box::new(e))),
        };
        let b3hash: Vec<u8> = row.try_get("b3hash")?;

        if b3hash.len() != 32 {
            return Err(sqlx::Error::ColumnDecode {
                index: "b3hash".to_string(),
                source: Box::new(std::io::Error::other("invalid b3 hash len")),
            });
        }

        let mut b3hash_arr = [0u8; 32];
        b3hash_arr.copy_from_slice(&b3hash);

        Ok(Self {
            id,
            file_path,
            mime,
            b3hash: Hash::from_bytes(b3hash_arr),
        })
    }
}

impl File {
    /// Attempts to load a file by path and mime type
    ///
    /// # Errors
    /// This function returns an error if a database error occurs while loading.
    #[instrument(skip(db))]
    async fn get_by_path_mime_inner(db: &Database, path: &str, mime: &str) -> Result<Option<Self>> {
        query_as(r#"SELECT * FROM file_map WHERE "file_path" = $1 AND "mime" = $2"#)
            .bind(path)
            .bind(mime)
            .fetch_optional(&*db.0)
            .await
            .with_context(|| format!("Loading file path {path} with mime type {mime}"))
    }

    /// Attempts to load a file by path and mime type
    ///
    /// # Errors
    /// This function returns an error if a database error occurs while loading.
    #[instrument(skip(db))]
    pub async fn get_by_path_mime(db: &Database, path: &str, mime: &str) -> Result<Option<Self>> {
        static CACHE: LazyLock<DashMap<(String, String), CacheValue>> = LazyLock::new(DashMap::new);
        let path = path.to_string();
        let mime = mime.to_string();
        if let Some(value) = CACHE.get(&(path.clone(), mime.clone())) {
            let file = value.file.clone();
            if value.last_checked < (Utc::now() - Duration::minutes(1)) {
                let db2 = db.clone();
                let path = path.clone();
                let mime = mime.clone();
                tokio::spawn(async move {
                    match Self::get_by_path_mime_inner(&db2, &path, &mime).await {
                        Err(e) => {
                            error!("Failed to update path info for {path}: {e:?}");
                        }
                        Ok(None) => {
                            CACHE.remove(&(path, mime));
                        }
                        Ok(Some(file)) => {
                            CACHE.insert(
                                (path, mime),
                                CacheValue {
                                    file,
                                    last_checked: Utc::now(),
                                },
                            );
                        }
                    }
                });
            }
            Ok(Some(file))
        } else {
            let file = Self::get_by_path_mime_inner(db, &path, &mime).await?;
            if let Some(ref file) = file {
                CACHE.insert(
                    (path, mime),
                    CacheValue {
                        file: file.clone(),
                        last_checked: Utc::now(),
                    },
                );
            }
            Ok(file)
        }
    }

    /// Attempts to load any files by path.
    ///
    /// # Errors
    /// This function returns an error if a database error occurs while loading.
    #[instrument(skip(db))]
    async fn get_by_path_inner(db: &Database, path: &str) -> Result<Vec<Self>> {
        query_as(r#"SELECT * FROM file_map WHERE "file_path" = $1"#)
            .bind(path)
            .fetch_all(&*db.0)
            .await
            .with_context(|| format!("Loading files with path {path}"))
    }

    /// Attempts to load any files by path.
    ///
    /// # Errors
    /// This function returns an error if a database error occurs while loading.
    #[instrument(skip(db))]
    pub async fn get_by_path(db: &Database, path: &str) -> Result<Vec<Self>> {
        static CACHE: LazyLock<DashMap<String, CacheValueVec>> = LazyLock::new(DashMap::new);
        if let Some(value) = CACHE.get(path) {
            let file = value.file.clone();
            if value.last_checked < (Utc::now() - Duration::minutes(1)) {
                let db2 = db.clone();
                let path = path.to_string();
                tokio::spawn(async move {
                    match Self::get_by_path_inner(&db2, &path).await {
                        Err(e) => {
                            error!("Failed to update path info for {path}: {e:?}");
                        }
                        Ok(v) if v.is_empty() => {
                            CACHE.remove(&path);
                        }
                        Ok(file) => {
                            CACHE.insert(
                                path,
                                CacheValueVec {
                                    file,
                                    last_checked: Utc::now(),
                                },
                            );
                        }
                    }
                });
            }
            Ok(file)
        } else {
            let file = Self::get_by_path_inner(db, path).await?;
            if !file.is_empty() {
                CACHE.insert(
                    path.to_string(),
                    CacheValueVec {
                        file: file.clone(),
                        last_checked: Utc::now(),
                    },
                );
            }
            Ok(file)
        }
    }

    /// Returns a paginated view into the file table
    ///
    /// # Errors
    /// This function returns an error if a database error occurs while loading.
    #[instrument(skip(db))]
    pub async fn list(db: &Database, after: i64, limit: usize) -> Result<Vec<Self>> {
        let limit: i64 = limit.min(100).try_into().unwrap_or(100); // reasonable limit for pagination size
        query_as(r#"SELECT * FROM file_map WHERE "id" > $1 LIMIT $2"#)
            .bind(after)
            .bind(limit)
            .fetch_all(&*db.0)
            .await
            .with_context(|| format!("Loading up to {limit} files after id {after}"))
    }

    /// Creates a new file
    ///
    /// # Errors
    /// This function returns an error if a database error occurs when writing
    #[instrument(skip(db))]
    pub async fn new(db: &Database, path: &str, mime: &str, hash: &Hash) -> Result<()> {
        query_as(
            r#"INSERT INTO file_map ("file_path", "mime", "b3hash") VALUES ($1, $2, $3) ON CONFLICT ("file_path", "mime") DO UPDATE SET "b3hash" = $3 RETURNING *"#,
        )
        .bind(path)
        .bind(mime)
        .bind(hash.as_bytes().as_slice())
        .fetch_one(&*db.0)
        .await
        .with_context(|| format!("Inserting new file {path} with mime type {mime}"))
    }

    /// Deletes a file record from the database. This does not perform any actual file deletion.
    ///
    /// # Errors
    /// This function returns an error if removing the entry from the database fails.
    #[instrument(skip(db))]
    pub async fn delete(self, db: &Database) -> Result<()> {
        let id: i64 = self.id.try_into()?;
        query!(r#"DELETE FROM file_map WHERE "id" = $1"#, id)
            .execute(&*db.0)
            .await
            .with_context(|| {
                format!(
                    "Deleting file {} with mime type {}",
                    self.file_path, self.mime
                )
            })?;
        Ok(())
    }

    /// Returns the immutable ID of the object
    #[must_use]
    pub const fn id(&self) -> u64 {
        self.id
    }

    /// Updates the file with new information
    ///
    /// # Errors
    /// This function returns an error if updating the entry in the database fails
    #[instrument(skip(db))]
    pub async fn update(&self, db: &Database) -> Result<()> {
        let id: i64 = self.id.try_into()?;
        let b3hash = self.b3hash.as_bytes().as_slice();
        query!(
            r#"UPDATE file_map SET "file_path" = $1, "mime" = $2, "b3hash" = $3 WHERE "id" = $4"#,
            self.file_path,
            self.mime.as_ref(),
            b3hash,
            id
        )
        .execute(&*db.0)
        .await
        .with_context(|| {
            format!(
                "Deleting file {} with mime type {}",
                self.file_path, self.mime
            )
        })?;
        Ok(())
    }

    /// Checks if a particular hash is in use
    ///
    /// # Errors
    /// This function returns an error if updating the entry in the database fails
    #[instrument(skip(db))]
    pub async fn is_used(db: &Database, hash: Hash) -> Result<bool> {
        let count = query!(
            r#"SELECT COUNT(*) as amount FROM file_map WHERE "b3hash" = $1"#,
            hash.as_bytes()
        )
        .fetch_one(&*db.0)
        .await
        .with_context(|| format!("Checking if {hash:?} is still used."))?;

        Ok(count.amount.unwrap_or_default() != 0)
    }
}
