use std::sync::Arc;

use chir_rs_common::queue::QueueAction;
use chrono::Utc;
use eyre::Result;
use futures::StreamExt;
use sqlx::query;
use std::fmt::Write;

use crate::{queue, Global};

/// Updates the robots.txt file
///
/// # Errors
/// This function returns an error if fetching the robots.txt data fails
pub async fn update_robots(global: &Arc<Global>) -> Result<()> {
    let mut robots = query!("SELECT user_agent, robot_path, allow FROM robots").fetch(&global.db.0);
    let mut robots_txt = String::new();

    while let Some(robots_record) = robots.next().await {
        let robots_record = robots_record?;
        writeln!(&mut robots_txt, "User-Agent: {}", robots_record.user_agent)?;
        if robots_record.allow {
            writeln!(&mut robots_txt, "Allow: {}", robots_record.robot_path)?;
        } else {
            writeln!(&mut robots_txt, "Deny: {}", robots_record.robot_path)?;
        }

        robots_txt.push('\n');
    }

    robots_txt.push_str("Sitemap: https://lotte.chir.rs/sitemap.xml\n");
    let mut txn = global.db.0.begin().await?;
    let ca_id = queue::queue(
        QueueAction::UploadCA(robots_txt.into_bytes()),
        &mut txn,
        Utc::now(),
        0,
        Vec::new(),
    )
    .await?;
    queue::queue(
        QueueAction::RaccreateFile("/robots.txt".to_string(), "text/plain".to_string()),
        &mut txn,
        Utc::now(),
        0,
        vec![ca_id],
    )
    .await?;
    txn.commit().await?;
    Ok(())
}
