use std::{net::SocketAddr, process::ExitCode, sync::Arc};

use axum::{
    routing::{delete, get, patch, post},
    Extension, Router,
};
use clap::{Parser, Subcommand};
use handlebars::Handlebars;
use rust_embed::RustEmbed;
use schema::install_schema;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

mod api_data;
mod auth;
mod controller;
mod schema;
mod seeds;
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

fn routes() -> Router {
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

    Router::new()
        .nest("/api/v1", api_routes)
        .nest("/auth", auth_routes)
}

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[arg(long, default_value = "development.sqlite")]
    database: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Start {
        #[arg(long, default_value = "8080")]
        port: u16,
    },
    User {
        #[command(subcommand)]
        command: UserCommands,
    },
    Db {
        #[command(subcommand)]
        command: DbCommands,
    },
}

#[derive(Subcommand)]
enum UserCommands {
    Add { username: String, password: String },
    SetPassword { username: String, password: String },
}

#[derive(Subcommand)]
enum DbCommands {
    Init,
    Seed,
}

async fn create_pool(database: &str) -> SqlitePool {
    SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database)
        .await
        .expect("couldn't connect to database")
}

async fn start_server(port: u16, pool: SqlitePool) {
    let app = routes()
        .layer(Extension(pool))
        .layer(Extension(Arc::new(make_handlebars())));

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[tokio::main]
async fn main() -> ExitCode {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Start { port } => {
            start_server(port, create_pool(&cli.database).await).await;
            ExitCode::SUCCESS
        }
        Commands::User { command } => match command {
            UserCommands::Add { username, password } => {
                let pool = create_pool(&cli.database).await;
                match users::add_user(&pool, &username, password).await {
                    Err(e) => {
                        eprintln!("Error creating user: {}", e);
                        ExitCode::FAILURE
                    }
                    Ok(user_id) => {
                        println!("{}", user_id);
                        ExitCode::SUCCESS
                    }
                }
            }
            UserCommands::SetPassword { username, password } => {
                let pool = create_pool(&cli.database).await;
                match users::user_by_name(&pool, &username).await {
                    Ok(Some(id)) => match users::set_password(&pool, id, password).await {
                        Ok(()) => ExitCode::SUCCESS,
                        Err(e) => {
                            eprintln!("Error updating password: {}", e);
                            ExitCode::FAILURE
                        }
                    },
                    Ok(None) => {
                        eprintln!("User not found");
                        ExitCode::FAILURE
                    }
                    Err(e) => {
                        eprintln!("Error getting user when updating password: {}", e);
                        ExitCode::FAILURE
                    }
                }
            }
        },
        Commands::Db { command } => match command {
            DbCommands::Init => {
                let pool = create_pool(&cli.database).await;
                match install_schema(&pool).await {
                    Ok(()) => ExitCode::SUCCESS,
                    Err(e) => {
                        eprintln!("Error initializing database schema: {}", e);
                        ExitCode::FAILURE
                    }
                }
            }
            DbCommands::Seed => {
                let pool = create_pool(&cli.database).await;
                match seeds::install(&pool).await {
                    Ok(()) => {
                        eprintln!("Created users user and user1 with password 123");
                        ExitCode::SUCCESS
                    }
                    Err(e) => {
                        eprintln!("Error installing database seeds: {:?}", e);
                        ExitCode::FAILURE
                    }
                }
            }
        },
    }
}
