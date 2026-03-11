use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "wshm", about = "Your repo's wish is my command.", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,

    /// Skip GitHub sync, use cached data only
    #[arg(long, global = true)]
    pub offline: bool,

    /// Detailed output
    #[arg(long, short, global = true)]
    pub verbose: bool,

    /// JSON output for scripting
    #[arg(long, global = true)]
    pub json: bool,

    /// Override detected repo (owner/repo)
    #[arg(long, global = true)]
    pub repo: Option<String>,
}

#[derive(Subcommand)]
pub enum Command {
    /// Force full sync from GitHub
    Sync,

    /// Classify issues
    Triage(TriageArgs),

    /// Analyze pull requests
    Pr(PrArgs),

    /// Show ranked merge queue
    Queue(QueueArgs),

    /// Detect and resolve conflicts
    Conflicts(ConflictArgs),

    /// Full cycle: sync + triage + analyze + queue + conflicts
    Run(RunArgs),

    /// Configuration management
    #[command(subcommand)]
    Config(ConfigCommand),
}

#[derive(clap::Args)]
pub struct TriageArgs {
    /// Triage single issue by number
    #[arg(long)]
    pub issue: Option<u64>,

    /// Actually perform actions (dry-run by default)
    #[arg(long)]
    pub apply: bool,
}

#[derive(clap::Args)]
pub struct PrArgs {
    /// Analyze single PR by number
    #[arg(long)]
    pub pr: Option<u64>,

    /// Actually perform actions (dry-run by default)
    #[arg(long)]
    pub apply: bool,
}

#[derive(clap::Args)]
pub struct QueueArgs {
    /// Merge top PR if above threshold
    #[arg(long)]
    pub apply: bool,
}

#[derive(clap::Args)]
pub struct ConflictArgs {
    /// Attempt conflict resolution
    #[arg(long)]
    pub apply: bool,
}

#[derive(clap::Args)]
pub struct RunArgs {
    /// Actually perform actions (dry-run by default)
    #[arg(long)]
    pub apply: bool,
}

#[derive(Subcommand)]
pub enum ConfigCommand {
    /// Create .wshm/config.toml template
    Init,
}
