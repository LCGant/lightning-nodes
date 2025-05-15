use anyhow::Context;
use lightning_nodes::{config::Settings, startup};
use tracing_subscriber::{fmt, EnvFilter};

/// Entry point: initializes structured logging, loads settings, 
/// and hands off control to the startup module.
///
/// # Errors
///
/// Returns an error if configuration cannot be loaded or if startup fails.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    fmt()
        // use RUST_LOG to control verbosity at runtime
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let settings = Settings::from_env()
        .context("failed to load configuration")?;

    // invoke the orchestrator that sets up the server and background task
    startup::run(settings).await
}
