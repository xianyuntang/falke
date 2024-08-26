mod config;

use axum::Router;
use config::Config;
use dotenv::dotenv;
use sea_orm::{Database, DatabaseConnection};

fn make_app() -> Router {
    let app = Router::new();
    app
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    dotenv().ok();

    let app = make_app();

    let config = Config::new();

    let _db: DatabaseConnection = Database::connect(config.db_connection_url)
        .await
        .expect("Failed to connect to database");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
