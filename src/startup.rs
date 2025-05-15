use actix_web::{middleware::Logger, web, App, HttpServer};
use anyhow::Context;
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

use crate::{
    config::Settings,
    db,
    handlers,
    repository::SqliteNodeRepo,
    service::{self, ReqwestClient},
};

/// Orchestrates database setup, background import task, and HTTP server.
///
/// Shuts down gracefully on Ctrl-C or SIGTERM.
///
/// # Errors
///
/// Returns an error if database initialization or server startup fails.
pub async fn run(settings: Settings) -> anyhow::Result<()> {
    // Initialize SQLite pool with WAL mode for concurrent reads/writes.
    let pool = db::init_pool(&settings.database_url)
        .await
        .context("failed to initialize SQLite pool")?;
    let repo = SqliteNodeRepo::new(pool.clone());

    // Spawn periodic import task using a cancellation token for graceful shutdown.
    let cancel_token = CancellationToken::new();
    let import_handle = {
        let repo = repo.clone();
        let settings = settings.clone();
        let client = ReqwestClient;
        let token = cancel_token.clone();

        tokio::spawn(async move {
            tokio::select! {
                // Runs fetch-and-store until error or cancellation.
                _ = service::import_task(repo, settings, client) => {},
                _ = token.cancelled() => {}
            }
        })
    };

    // Configure and run the HTTP server with a health check endpoint.
    let mut server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())  // structured logging for all requests
            .configure(|cfg| handlers::init_routes(cfg, repo.clone()))
            .route("/healthz", web::get().to(|| async { "OK" }))
    })
    .bind(("0.0.0.0", 8080))?
    .run();

    info!("server running on http://localhost:8080");

    // Await shutdown signal or server error.
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            info!("shutdown signal received");
        }
        res = &mut server => {
            if let Err(e) = res {
                error!(?e, "server terminated unexpectedly");
            }
        }
    }

    // Signal import task to stop and await its completion.
    cancel_token.cancel();
    import_handle
        .await
        .context("failed to join import task")?;

    Ok(())
}
