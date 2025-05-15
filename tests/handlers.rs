#![allow(clippy::unwrap_used, clippy::expect_used)]

use actix_web::{test, App, http::StatusCode};
use sqlx::SqlitePool;
use lightning_nodes::{handlers::init_routes, repository::SqliteNodeRepo};

/// Verifies that `/nodes` returns 500 if the `nodes` table is missing.
#[actix_web::test]
async fn nodes_endpoint_should_return_500_when_table_missing() {
    // Use an in-memory SQLite without initializing the schema
    let pool = SqlitePool::connect(":memory:").await.unwrap();
    let repo = SqliteNodeRepo::new(pool);

    let app = test::init_service(
        App::new().configure(|cfg| init_routes(cfg, repo.clone()))
    ).await;

    let req = test::TestRequest::get().uri("/nodes").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

/// Verifies that `/nodes` returns 200 and an empty JSON list when the table is present but has no data.
#[actix_web::test]
async fn nodes_endpoint_should_return_200_and_empty_list_when_no_data() {
    // Initialize an in-memory SQLite with the `nodes` table
    let pool = lightning_nodes::db::init_pool("sqlite::memory:").await.unwrap();
    let repo = SqliteNodeRepo::new(pool);

    let app = test::init_service(
        App::new().configure(|cfg| init_routes(cfg, repo.clone()))
    ).await;

    let req = test::TestRequest::get().uri("/nodes").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
    let body = test::read_body(resp).await;
    assert_eq!(body, "[]");
}
