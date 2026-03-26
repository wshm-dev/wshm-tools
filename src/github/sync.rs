use anyhow::Result;
use chrono::Utc;
use tracing::info;

use crate::db::Database;
use crate::github::Client;

pub async fn full_sync(gh: &Client, db: &Database) -> Result<()> {
    info!("Starting full sync...");
    sync_issues(gh, db, None).await?;
    sync_pulls(gh, db).await?;
    info!("Full sync complete.");
    Ok(())
}

pub async fn incremental_sync(gh: &Client, db: &Database, table: &str) -> Result<()> {
    let entry = db.get_sync_entry(table)?;

    let since = entry.as_ref().map(|e| {
        // Check if last sync was within the interval
        if let Ok(last) = e.last_synced_at.parse::<chrono::DateTime<Utc>>() {
            let elapsed = Utc::now().signed_duration_since(last);
            if elapsed.num_minutes() < 5 {
                tracing::debug!("Skipping sync for {table}: last sync was {elapsed} ago");
                return None;
            }
            Some(e.last_synced_at.clone())
        } else {
            None
        }
    });

    // If inner Option is None (within interval), skip sync
    if let Some(None) = since {
        return Ok(());
    }

    let since_str = since.flatten();

    match table {
        "issues" => sync_issues(gh, db, since_str.as_deref()).await?,
        "pulls" => sync_pulls(gh, db).await?,
        _ => anyhow::bail!("Unknown sync table: {table}"),
    }

    Ok(())
}

/// Force sync issues now (bypass throttle). Used when we know there's a new event.
pub async fn sync_issues_now(gh: &Client, db: &Database) -> Result<()> {
    sync_issues(gh, db, None).await
}

/// Force sync pulls now (bypass throttle). Used when we know there's a new event.
pub async fn sync_pulls_now(gh: &Client, db: &Database) -> Result<()> {
    sync_pulls(gh, db).await
}

async fn sync_issues(gh: &Client, db: &Database, since: Option<&str>) -> Result<()> {
    info!("Syncing issues...");
    // Fetch ALL states (open+closed) so closed issues get updated in local DB
    let issues = gh.fetch_all_issues(since).await?;
    let count = issues.len();

    db.batch_upsert_issues(&issues)?;

    let now = Utc::now().to_rfc3339();
    db.update_sync_entry("issues", &now, None)?;

    info!("Synced {count} issues.");
    Ok(())
}

async fn sync_pulls(gh: &Client, db: &Database) -> Result<()> {
    info!("Syncing pull requests...");
    let mut pulls = gh.fetch_pulls().await?;
    let count = pulls.len();

    // Fetch mergeable status concurrently (GitHub only returns it on single-PR endpoint)
    let needs_mergeable: Vec<(usize, u64)> = pulls
        .iter()
        .enumerate()
        .filter(|(_, pr)| pr.mergeable.is_none())
        .map(|(i, pr)| (i, pr.number))
        .collect();

    if !needs_mergeable.is_empty() {
        info!("Fetching mergeable status for {} PRs (concurrent)...", needs_mergeable.len());
        let results: Vec<(usize, Result<Option<bool>>)> =
            futures::future::join_all(needs_mergeable.iter().map(|&(idx, number)| async move {
                (idx, gh.fetch_pr_mergeable(number).await)
            }))
            .await;

        for (idx, result) in results {
            match result {
                Ok(m) => pulls[idx].mergeable = m,
                Err(e) => tracing::warn!("Failed to fetch mergeable for PR #{}: {e}", pulls[idx].number),
            }
        }
    }

    db.batch_upsert_pulls(&pulls)?;

    let now = Utc::now().to_rfc3339();
    db.update_sync_entry("pulls", &now, None)?;

    info!("Synced {count} pull requests.");
    Ok(())
}
