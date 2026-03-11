use anyhow::Result;
use tracing::info;

use crate::cli::ConflictArgs;
use crate::config::Config;
use crate::db::Database;
use crate::github::Client as GhClient;

pub async fn run(
    _config: &Config,
    db: &Database,
    _gh: &GhClient,
    _args: &ConflictArgs,
) -> Result<()> {
    let pulls = db.get_open_pulls()?;

    if pulls.is_empty() {
        println!("No open PRs to check for conflicts.");
        return Ok(());
    }

    let mut conflicts = Vec::new();
    let mut clean = 0;
    let mut unknown = 0;

    for pr in &pulls {
        match pr.mergeable {
            Some(true) => clean += 1,
            Some(false) => {
                conflicts.push(pr);
            }
            None => unknown += 1,
        }
    }

    println!("Conflict Scan ({} PRs):", pulls.len());
    println!("  Clean: {clean}");
    println!("  Conflicts: {}", conflicts.len());
    println!("  Unknown: {unknown}");

    if !conflicts.is_empty() {
        println!("\nConflicting PRs:");
        for pr in &conflicts {
            println!(
                "  #{} {} ({}←{})",
                pr.number,
                pr.title,
                pr.base_ref.as_deref().unwrap_or("?"),
                pr.head_ref.as_deref().unwrap_or("?"),
            );
        }
    }

    // TODO: auto-resolve with AI in M3
    if !conflicts.is_empty() {
        info!("Auto-resolve not yet implemented. Coming in M3.");
    }

    Ok(())
}
