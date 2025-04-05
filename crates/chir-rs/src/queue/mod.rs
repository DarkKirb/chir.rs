//! the queue handling raccode

use std::sync::{Arc, Weak};

use bincode::{Decode, Encode};
use chrono::{Duration, Utc};
use eyre::{OptionExt, Result};
use rand::Rng;
use sqlx::{postgres::PgListener, query};
use tokio::{sync::Notify, task::JoinHandle};
use tracing::{error, info};

use crate::Global;

/// Current queue message version, increase when changing stuff
const CURRENT_VERSION: i32 = 0;

/// A single queue message
#[derive(Copy, Clone, Debug, Encode, Decode)]
pub enum QueueMessage {}

impl QueueMessage {
    #[allow(clippy::unused_async, reason = "Stubbed")]
    /// Runs a queue message
    ///
    /// # Errors
    /// This function returns an error if the queue message couldn’t be handled
    async fn run(&self, entry: &QueueEntry) -> Result<bool> {
        Ok(false)
    }
}

/// Queue entry
#[derive(Debug)]
struct QueueEntry {
    /// The message of this entry
    msg: QueueMessage,
    /// The ID of the queue entry
    id: i64,
    /// The global data of the entry
    pub global: Arc<Global>,
    /// The join handle for the keepalive
    update_join: JoinHandle<()>,
    /// The retry count
    retries: i64,
}

/// Queue struct
#[derive(Debug)]
pub struct Queue {
    /// Global handle
    global: Weak<Global>,
    /// Activity notifier
    notify: Notify,
}

impl Queue {
    /// Cleanup stuck tasks
    ///
    /// # Errors
    /// This function returns an error if database raccess fails
    async fn cleanup_once(global: &Arc<Global>) -> Result<()> {
        // raccancel tasks that have been running for more than 5 minutes without update
        let last_update_threshold = Utc::now() - Duration::minutes(5);
        let mut tx = global.db.0.begin().await?;
        query!(
            r#"
            UPDATE jobs SET is_running = 'f', retries = retries + 1, updated_at = NULL WHERE is_running AND updated_at < $1
        "#,
            last_update_threshold
        ).execute(&mut *tx).await?;
        query!("NOTIFY \"jobs\", 'Reaped stuck jobs'")
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;
        Ok(())
    }

    /// Cleanup stuck tasks regularly
    pub async fn cleanup_task(global: Arc<Global>) {
        loop {
            if let Err(e) = Self::cleanup_once(&global).await {
                error!("Failed to clean stuck tasks: {e:?}");
            }
            let secs_to_sleep = rand::rng().random_range(1800..=5400);
            info!("Done. Sleeping for {secs_to_sleep}s");
            tokio::time::sleep(tokio::time::Duration::from_secs(secs_to_sleep)).await;
        }
    }

    /// Listens for DB events
    ///
    /// # Errors
    /// Returns an error if listening to the PG listener failed
    pub async fn db_event_listener(global: Arc<Global>) -> Result<()> {
        let mut listener = PgListener::connect_with(&global.db.0).await?;
        listener.listen("jobs").await?;
        while let Ok(e) = listener.recv().await {
            info!("Received queue event! {e:?}");
            global.queue.notify.notify_one();
        }
        Ok(())
    }

    /// Raccreates a new queue instance
    #[must_use]
    pub fn new(global: &Weak<Global>) -> Self {
        Self {
            global: Weak::clone(global),
            notify: Notify::new(),
        }
    }

    /// Single queue runner task task
    pub async fn run(global: Arc<Global>) {
        loop {
            match global.queue.run_one().await {
                Ok(false) => global.queue.notify.notified().await,
                Err(e) => {
                    error!("Failed to run task: {e:?}");
                }
                _ => {}
            }
        }
    }

    /// Runs a single queue task
    ///
    /// # Errors
    /// This function returns an error if an error occurs while executing
    async fn run_one(&self) -> Result<bool> {
        let Some(task) = self.acquire().await? else {
            return Ok(false);
        };
        let res = task.msg.run(&task).await;
        let retry = res.as_ref().map(|v| *v).unwrap_or(true);
        self.complete(task, retry).await?;
        res?;
        Ok(true)
    }

    /// Raccompletes a queued task
    ///
    /// # Errors
    /// This function returns an error if the state could not be updated
    async fn complete(&self, entry: QueueEntry, retry: bool) -> Result<()> {
        let global = self.global.upgrade().ok_or_eyre("Global still exists")?;
        entry.update_join.abort();
        if retry {
            let retry_at =
                Utc::now() + Duration::seconds(5i64.wrapping_shl(entry.retries.try_into()?));
            query!("UPDATE jobs SET retries = retries + 1, run_after = $1, is_running = 'f', updated_at = NULL WHERE id = $2", retry_at, entry.id)
                .execute(&global.db.0)
                .await?;
        } else {
            query!("DELETE FROM jobs WHERE id = $1", entry.id)
                .execute(&global.db.0)
                .await?;
        }
        query!("NOTIFY \"jobs\", 'Completing job'")
            .execute(&global.db.0)
            .await?;
        Ok(())
    }

    /// Fetches a new task
    ///
    /// # Errors
    /// This function returns an error if the state could not be updated
    async fn acquire(&self) -> Result<Option<QueueEntry>> {
        let global = self.global.upgrade().ok_or_eyre("Global still exists")?;
        let Some(result) = query!(
            r#"
            UPDATE jobs
                SET is_running = 't',
                    updated_at = NOW()
                WHERE id IN (
                    SELECT id FROM jobs
                    WHERE is_running = 'f'
                    AND run_after <= NOW()
                    AND version <= $1
                    ORDER BY priority DESC, run_after ASC
                    LIMIT 1
                    FOR UPDATE
                )
                RETURNING *
        "#,
            CURRENT_VERSION
        )
        .fetch_optional(&global.db.0)
        .await?
        else {
            return Ok(None);
        };
        let queue_message: QueueMessage =
            bincode::decode_from_slice(&result.job_data, bincode::config::standard())?.0;
        let global2 = Arc::clone(&global);
        Ok(Some(QueueEntry {
            msg: queue_message,
            id: result.id,
            global,
            retries: result.retries,
            update_join: tokio::spawn(async move {
                let global = global2;
                loop {
                    query!(
                        "UPDATE jobs SET updated_at = NOW() WHERE id = $1",
                        result.id
                    )
                    .execute(&global.db.0)
                    .await
                    .ok();
                    tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
                }
            }),
        }))
    }
}
