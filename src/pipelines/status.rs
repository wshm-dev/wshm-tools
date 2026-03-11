use anyhow::Result;

use crate::db::Database;

pub fn show(db: &Database) -> Result<()> {
    let open_issues = db.get_open_issues()?;
    let untriaged = db.get_untriaged_issues()?;
    let open_pulls = db.get_open_pulls()?;
    let unanalyzed = db.get_unanalyzed_pulls()?;

    println!("wshm — status");
    println!("─────────────────────────");
    println!(
        "Issues:  {} open ({} untriaged)",
        open_issues.len(),
        untriaged.len()
    );
    println!(
        "PRs:     {} open ({} unanalyzed)",
        open_pulls.len(),
        unanalyzed.len()
    );

    let conflicts: usize = open_pulls
        .iter()
        .filter(|p| p.mergeable == Some(false))
        .count();
    if conflicts > 0 {
        println!("Conflicts: {conflicts}");
    }

    if let Ok(Some(sync_entry)) = db.get_sync_entry("issues") {
        println!("Last sync: {}", sync_entry.last_synced_at);
    } else {
        println!("Last sync: never (run `wshm sync`)");
    }

    Ok(())
}
