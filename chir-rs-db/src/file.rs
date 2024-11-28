//! File related APIs

use bincode::{Decode, Encode};
use blake3::Hash;
use eyre::Context as _;
use eyre::Result;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, prelude::FromRow, query_as};
use sqlx::{query, Row as _};
use tracing::instrument;

use crate::Database;

/// File record
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct File {
    /// ID of the file record
    id: u64,
    /// Path this file is mounted at
    pub file_path: String,
    /// MIME type of file
    pub mime: String,
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
        self.mime.encode(encoder)?;
        self.b3hash.as_bytes().encode(encoder)
    }
}

impl Decode for File {
    fn decode<D: bincode::de::Decoder>(
        decoder: &mut D,
    ) -> std::result::Result<Self, bincode::error::DecodeError> {
        let id = u64::decode(decoder)?;
        let file_path = String::decode(decoder)?;
        let mime = String::decode(decoder)?;
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
        let mime: String = row.try_get("mime")?;
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
    pub async fn get_by_path_mime(db: &Database, path: &str, mime: &str) -> Result<Option<Self>> {
        query_as(r#"SELECT * FROM file_map WHERE "path" = $1 AND "mime" = $2"#)
            .bind(path)
            .bind(mime)
            .fetch_optional(&*db.0)
            .await
            .with_context(|| format!("Loading file path {path} with mime type {mime}"))
    }

    /// Attempts to load any files by path.
    ///
    /// # Errors
    /// This function returns an error if a database error occurs while loading.
    #[instrument(skip(db))]
    pub async fn get_by_path(db: &Database, path: &str) -> Result<Vec<Self>> {
        query_as(r#"SELECT * FROM file_map WHERE "path" = $1"#)
            .bind(path)
            .fetch_all(&*db.0)
            .await
            .with_context(|| format!("Loading files with path {path}"))
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
    /// This function returns an error if a database error occurs when writing, or there is a conflict
    #[instrument(skip(db))]
    pub async fn new(db: &Database, path: &str, mime: &str, hash: &Hash) -> Result<()> {
        query_as(
            r#"INSERT INTO file_map ("file_path", "mime", "b3hash") VALUES ($1, $2, $3) RETURNING *"#,
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
    /// THis function returns an error if updating the entry in the database fails
    #[instrument(skip(db))]
    pub async fn update(&self, db: &Database) -> Result<()> {
        let id: i64 = self.id.try_into()?;
        let b3hash = self.b3hash.as_bytes().as_slice();
        query!(
            r#"UPDATE file_map SET "file_path" = $1, "mime" = $2, "b3hash" = $3 WHERE "id" = $4"#,
            self.file_path,
            self.mime,
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
}
