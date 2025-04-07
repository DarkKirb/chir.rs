//! the queue handling raccode

use std::sync::{Arc, Weak};

use bincode::{Decode, Encode};
use chrono::{DateTime, Duration, Utc};
use eyre::{OptionExt, Result};
use futures::StreamExt;
use rand::Rng;
use sqlx::{postgres::PgListener, query, Postgres, Transaction};
use tokio::{sync::Notify, task::JoinHandle};
use tracing::{debug, error, info};

use crate::{db, Global};

/// Current queue message version, increase when changing stuff
const CURRENT_VERSION: i32 = 2;

/// The queue task
#[derive(Clone, Debug, Encode, Decode)]
pub enum QueueAction {
    /// This task does nothing, successfully.
    Nop,
    /// Uploads specific data to the CA store
    UploadCA(Vec<u8>),
    /// Raccreates File
    RaccreateFile(String, String),
    /// Updates robots.txt
    UpdateRobots,
}

impl QueueAction {
    /// Queues the queue action for execution
    ///
    /// # Errors
    /// This function returns an error if there was an error scheduling it
    pub async fn queue(
        &self,
        txn: &mut Transaction<'_, Postgres>,
        run_after: DateTime<Utc>,
        priority: i64,
        deps: Vec<i64>,
    ) -> Result<i64> {
        let job_data = bincode::encode_to_vec(self, bincode::config::standard())?;
        let result = query!(
            "INSERT INTO jobs (is_finished, run_after, job_data, priority, version) VALUES ('f', $1, $2, $3, $4) RETURNING id",
            run_after,
            job_data,
            priority,
            CURRENT_VERSION
        )
        .fetch_one(&mut **txn)
        .await?;
        for dep in deps {
            query!(
                "INSERT INTO job_deps (job_id, dependency_job_id) VALUES ($1, $2)",
                result.id,
                dep
            )
            .execute(&mut **txn)
            .await?;
        }
        query!("NOTIFY \"jobs\", 'Queued job'")
            .execute(&mut **txn)
            .await?;
        Ok(result.id)
    }
}

/// The queue task result
#[derive(Copy, Clone, Debug, Encode, Decode)]
pub enum QueueActionResult {
    /// Void result
    Nothing,
    /// CA Store result
    CAPath([u8; 32]),
}

/// Result of a queue action
#[derive(Clone, Debug, Encode, Decode)]
pub struct QueueMessageResult {
    /// Message that caused this
    message: QueueMessage,
    /// The result of said task
    pub result: QueueActionResult,
}

/// A single queue message
#[derive(Clone, Debug, Encode, Decode)]
pub struct QueueMessage {
    /// Racction to take
    action: QueueAction,
    /// Previous Racctions that triggered this
    previous: Vec<QueueMessageResult>,
}

/// Result of a queue run attempt
#[derive(Clone, Debug, Encode, Decode)]
pub enum QueueRunResult {
    /// Indicates that the process should be retried
    Retry,
    /// Indicates that the job is raccomplete with a specific result
    Complete(QueueMessageResult),
}

impl QueueMessage {
    #[allow(clippy::unused_async, reason = "Stubbed")]
    /// Runs a queue message
    ///
    /// # Errors
    /// This function returns an error if the queue message couldn’t be handled
    async fn run(&self, entry: &QueueEntry) -> Result<QueueRunResult> {
        match entry.msg.action {
            QueueAction::Nop => Ok(QueueRunResult::Complete(QueueMessageResult {
                message: self.clone(),
                result: QueueActionResult::Nothing,
            })),
            QueueAction::UploadCA(ref data) => Ok(QueueRunResult::Complete(QueueMessageResult {
                message: self.clone(),
                result: crate::castore::upload_ca(data, &entry.global).await?,
            })),
            QueueAction::RaccreateFile(ref file, ref mime) => {
                db::file::set_file(file, mime, &entry.global, &entry.msg.previous).await?;
                Ok(QueueRunResult::Complete(QueueMessageResult {
                    message: self.clone(),
                    result: QueueActionResult::Nothing,
                }))
            }
            QueueAction::UpdateRobots => {
                crate::robots::update_robots(&entry.global).await?;
                Ok(QueueRunResult::Complete(QueueMessageResult {
                    message: self.clone(),
                    result: QueueActionResult::Nothing,
                }))
            }
        }
    }
}

/// Queue entrydebug
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
        query!(
            r#"
                DELETE FROM jobs WHERE jobs.is_finished AND NOT EXISTS (SELECT 1 FROM job_deps WHERE job_deps.dependency_job_id = jobs.id LIMIT 1)
            "#
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
            debug!("Received queue event! {e:?}");
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
            info!("Finished with tasks, sleeping…");
            return Ok(false);
        };
        let res = task.msg.run(&task).await;
        let retry = res.as_ref().unwrap_or(&QueueRunResult::Retry);
        match retry {
            QueueRunResult::Retry => self.complete(task, None).await?,
            QueueRunResult::Complete(queue_message_result) => {
                self.complete(task, Some(queue_message_result)).await?;
            }
        }
        res?;
        Ok(true)
    }

    /// Raccompletes a queued task
    ///
    /// # Errors
    /// This function returns an error if the state could not be updated
    async fn complete(&self, entry: QueueEntry, result: Option<&QueueMessageResult>) -> Result<()> {
        let global = self.global.upgrade().ok_or_eyre("Global still exists")?;
        entry.update_join.abort();
        if let Some(result) = result {
            let encoded = bincode::encode_to_vec(result, bincode::config::standard())?;
            query!(
                "UPDATE jobs SET is_running = 'f', is_finished = 't', job_data = $1 WHERE id = $2",
                encoded,
                entry.id
            )
            .execute(&global.db.0)
            .await?;
        } else {
            let retry_at =
                Utc::now() + Duration::seconds(5i64.wrapping_shl(entry.retries.try_into()?));
            query!("UPDATE jobs SET retries = retries + 1, run_after = $1, is_running = 'f', updated_at = NULL WHERE id = $2", retry_at, entry.id)
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
                    AND is_finished = 'f'
                    AND run_after <= NOW()
                    AND version <= $1
                    AND NOT EXISTS (
                        SELECT 1
                            FROM job_deps
                            INNER JOIN jobs jobs2
                            ON job_deps.dependency_job_id = jobs2.id
                            WHERE jobs.id = job_deps.job_id
                            AND NOT jobs2.is_finished
                    )
                    ORDER BY priority DESC, run_after ASC
                    LIMIT 1
                    FOR UPDATE OF jobs
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
        let queue_action: QueueAction =
            bincode::decode_from_slice(&result.job_data, bincode::config::standard())?.0;
        let mut queue_deps = Vec::new();
        let mut queue_deps_iter = query!(
            r#"
            SELECT job_data FROM jobs
                INNER JOIN job_deps
                ON job_deps.dependency_job_id = jobs.id
                WHERE job_deps.job_id = $1
        "#,
            result.id
        )
        .fetch(&global.db.0);
        if let Some(ent) = queue_deps_iter.next().await {
            let ent = ent?;
            let queue_message_result: QueueMessageResult =
                bincode::decode_from_slice(&ent.job_data, bincode::config::standard())?.0;
            queue_deps.push(queue_message_result);
        }
        let global2 = Arc::clone(&global);
        Ok(Some(QueueEntry {
            msg: QueueMessage {
                action: queue_action,
                previous: queue_deps,
            },
            id: result.id,
            global: Arc::clone(&global),
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
