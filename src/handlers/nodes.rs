use actix_web::{get, web, HttpResponse};
use crate::repository::{SqliteNodeRepo, NodeRepository};
use tracing::instrument;

/// Handler for `GET /nodes` endpoint.
///
/// Fetches all stored nodes and returns them as JSON.
///
/// # Errors
///
/// Returns HTTP 500 if the repository call fails.
#[get("/nodes")]
#[instrument(skip(repo), fields(endpoint = "/nodes"))]
async fn list_nodes(repo: web::Data<SqliteNodeRepo>) -> HttpResponse {
    match repo.list().await {
        Ok(nodes) => HttpResponse::Ok().json(nodes),
        Err(e) => {
            tracing::error!(error = %e, "failed to list nodes");
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// Configures HTTP routes for node-related handlers.
///
/// Adds the `list_nodes` service and shares the repository instance via app data.
pub fn init_routes(cfg: &mut web::ServiceConfig, repo: SqliteNodeRepo) {
    cfg
        .app_data(web::Data::new(repo))
        .service(list_nodes);
}
