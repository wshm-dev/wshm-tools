//! `wshm migrate` — SQLite → PostgreSQL migration.
//!
//! The PostgreSQL backend and the SQLite-to-Postgres migration pipeline
//! are wshm-pro features. The OSS binary keeps the CLI surface so scripts
//! that wrap it don't error with "unknown command", but the command
//! exits with an error pointing the operator at wshm-pro.

use anyhow::Result;

use crate::cli::MigrateArgs;

pub async fn run(_args: &MigrateArgs, _cli: &crate::cli::Cli) -> Result<()> {
    anyhow::bail!(
        "`wshm migrate` (SQLite → PostgreSQL) is a wshm-pro feature.\n\
         OSS uses SQLite only. Upgrade to wshm-pro for PostgreSQL backend \
         and migration tooling."
    )
}
