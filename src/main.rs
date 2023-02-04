use std::{net::SocketAddr, sync::Arc};

use axum::{
    routing::{delete, get, patch, post},
    Extension, Router,
};
use handlebars::Handlebars;
use rust_embed::RustEmbed;
use sqlx::sqlite::SqlitePoolOptions;

mod api_data;
mod controller;
mod schema;
#[cfg(test)]
mod test_utils;
mod users;

#[derive(RustEmbed)]
#[folder = "templates"]
struct Assets;

pub fn make_handlebars() -> Handlebars<'static> {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_embed_templates::<Assets>()
        .expect("register embedded templates");
    handlebars
}

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
            patch(controller::categories::update_category),
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

    let auth_routes = Router::new()
        .route("/login", get(controller::auth::login_page))
        .route("/login", post(controller::auth::login_submit));

    let app = Router::new()
        .nest("/api/v1", api_routes)
        .nest("/auth", auth_routes)
        .with_state(pool)
        .layer(Extension(Arc::new(make_handlebars())));
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
