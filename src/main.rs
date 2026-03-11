use anyhow::Result;
use clap::Parser;
use tracing_subscriber::EnvFilter;

mod ai;
mod cli;
mod config;
mod db;
mod github;
mod pipelines;

use cli::{Cli, Command};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();

    match &cli.command {
        Some(Command::Sync) => {
            let config = config::Config::load(&cli)?;
            let db = db::Database::open(&config)?;
            let gh = github::Client::new(&config)?;
            github::sync::full_sync(&gh, &db).await?;
            println!("Sync complete.");
        }
        Some(Command::Triage(args)) => {
            let config = config::Config::load(&cli)?;
            let db = db::Database::open(&config)?;
            let gh = github::Client::new(&config)?;

            if !cli.offline {
                github::sync::incremental_sync(&gh, &db, "issues").await?;
            }

            pipelines::triage::run(&config, &db, &gh, args).await?;
        }
        Some(Command::Pr(args)) => {
            let config = config::Config::load(&cli)?;
            let db = db::Database::open(&config)?;
            let gh = github::Client::new(&config)?;

            if !cli.offline {
                github::sync::incremental_sync(&gh, &db, "pulls").await?;
            }

            pipelines::pr_analysis::run(&config, &db, &gh, args).await?;
        }
        Some(Command::Queue(args)) => {
            let config = config::Config::load(&cli)?;
            let db = db::Database::open(&config)?;
            let gh = github::Client::new(&config)?;

            if !cli.offline {
                github::sync::incremental_sync(&gh, &db, "pulls").await?;
            }

            pipelines::merge_queue::run(&config, &db, &gh, args).await?;
        }
        Some(Command::Conflicts(args)) => {
            let config = config::Config::load(&cli)?;
            let db = db::Database::open(&config)?;
            let gh = github::Client::new(&config)?;

            if !cli.offline {
                github::sync::incremental_sync(&gh, &db, "pulls").await?;
            }

            pipelines::conflict_resolution::run(&config, &db, &gh, args).await?;
        }
        Some(Command::Run(args)) => {
            let config = config::Config::load(&cli)?;
            let db = db::Database::open(&config)?;
            let gh = github::Client::new(&config)?;

            if !cli.offline {
                github::sync::full_sync(&gh, &db).await?;
            }

            let triage_args = cli::TriageArgs {
                issue: None,
                apply: args.apply,
            };
            pipelines::triage::run(&config, &db, &gh, &triage_args).await?;

            let pr_args = cli::PrArgs {
                pr: None,
                apply: args.apply,
            };
            pipelines::pr_analysis::run(&config, &db, &gh, &pr_args).await?;

            let queue_args = cli::QueueArgs { apply: args.apply };
            pipelines::merge_queue::run(&config, &db, &gh, &queue_args).await?;

            let conflict_args = cli::ConflictArgs { apply: args.apply };
            pipelines::conflict_resolution::run(&config, &db, &gh, &conflict_args).await?;

            println!("Full cycle complete.");
        }
        Some(Command::Config(config_cmd)) => match config_cmd {
            cli::ConfigCommand::Init => {
                config::Config::init_template()?;
                println!("Created .wshm/config.toml template.");
            }
        },
        None => {
            let config = config::Config::load(&cli)?;
            let db = db::Database::open(&config)?;
            pipelines::status::show(&db)?;
        }
    }

    Ok(())
}
