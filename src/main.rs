use anyhow::Result;
use clap::Parser;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    rustls::crypto::ring::default_provider()
        .install_default()
        .ok();
    let log_buffer = wshm_core::daemon::log_buffer::install_global();
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,wshm_core=debug"));
    tracing_subscriber::registry()
        .with(env_filter)
        .with(tracing_subscriber::fmt::layer())
        .with(wshm_core::daemon::log_buffer::LogLayer::new(log_buffer))
        .init();
    wshm_core::telemetry::maybe_ping();
    wshm_core::login::inject_credentials();
    let cli = wshm_core::Cli::parse();
    wshm_core::run_oss(cli).await
}
