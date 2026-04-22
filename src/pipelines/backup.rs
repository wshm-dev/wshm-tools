use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

/// Returns the default path for the wshm state database: ~/.wshm/state.db
pub fn default_state_db_path() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".wshm")
        .join("state.db")
}

/// Path of a timestamped backup: ~/.wshm/backups/state-<timestamp>.db
fn default_backup_path() -> PathBuf {
    let ts = chrono::Utc::now().format("%Y%m%dT%H%M%S");
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".wshm")
        .join("backups")
        .join(format!("state-{ts}.db"))
}

pub fn run_backup(output: Option<&Path>) -> Result<PathBuf> {
    // Look for state.db in .wshm/ (local repo) first, then fall back to ~/.wshm/state.db
    let src = if Path::new(".wshm/state.db").exists() {
        PathBuf::from(".wshm/state.db")
    } else {
        default_state_db_path()
    };

    if !src.exists() {
        anyhow::bail!(
            "No state.db found at {:?} or ~/.wshm/state.db. Run `wshm sync` first.",
            src
        );
    }

    let dest = output
        .map(PathBuf::from)
        .unwrap_or_else(default_backup_path);

    if let Some(parent) = dest.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // Use VACUUM INTO for an atomic, consistent SQLite backup (includes WAL checkpoint)
    let conn = rusqlite::Connection::open(&src)
        .with_context(|| format!("Cannot open {:?}", src))?;
    conn.execute_batch(&format!("VACUUM INTO '{}'", dest.display()))
        .with_context(|| format!("VACUUM INTO failed for {:?}", dest))?;

    println!("Backup saved to: {}", dest.display());
    Ok(dest)
}

pub fn run_restore(from: &Path) -> Result<()> {
    // Destination: local .wshm/state.db if we are inside a repo, else ~/.wshm/state.db
    let dest = if Path::new(".wshm").exists() {
        PathBuf::from(".wshm/state.db")
    } else {
        default_state_db_path()
    };

    if !from.exists() {
        anyhow::bail!("Backup file not found: {:?}", from);
    }

    // Validate that the backup is a valid SQLite database
    let conn = rusqlite::Connection::open(from)
        .with_context(|| format!("Cannot open backup file {:?}", from))?;
    conn.execute_batch("PRAGMA integrity_check")
        .with_context(|| format!("Integrity check failed for {:?}", from))?;
    drop(conn);

    // Safety backup of the existing database before overwriting
    if dest.exists() {
        let bak = dest.with_extension("db.pre-restore");
        std::fs::copy(&dest, &bak)
            .with_context(|| format!("Failed to create safety backup at {:?}", bak))?;
        println!("Existing DB backed up to: {}", bak.display());
    }

    std::fs::copy(from, &dest)
        .with_context(|| format!("Failed to restore {:?} -> {:?}", from, dest))?;

    println!("Restored from: {}", from.display());
    Ok(())
}
