use std::net::SocketAddr;

use axum::{
    routing::{delete, get, patch, post},
    Router,
};
use sqlx::sqlite::SqlitePoolOptions;

mod controller;
mod schema;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&":memory:".to_string())
        .await
        .expect("couldn't connect to database");

    let api_routes = Router::new()
        .route("/words", get(controller::categories::list_categories))
        .route("/words", post(controller::categories::create_category))
        .route(
            "/words/:category_id",
            patch(controller::categories::edit_category),
        )
        .route(
            "/words/:category_id",
            delete(controller::categories::delete_category),
        )
        .route("/words/:category_id", get(controller::words::list_words))
        .route("/words/:category_id", post(controller::words::create_word))
        .route(
            "/words/:category_id/:word_id",
            delete(controller::words::delete_word),
        );

    let app = Router::new().nest("/api/v1", api_routes).with_state(pool);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
