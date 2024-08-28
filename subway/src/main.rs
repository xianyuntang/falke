mod domain;
mod infrastructure;

use infrastructure::{config, db, logging, server};
use sea_orm::DatabaseConnection;

#[tokio::main]
async fn main() {
    logging::init_tracing();

    let config = config::Config::new();
    let db: DatabaseConnection = db::connect(config.db_connection_url).await;

    let app = server::make_app(db);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.server_port))
        .await
        .unwrap_or_else(|err| panic!("{}", err));

    tracing::info!(
        "Application is running on http://0.0.0.0:{}",
        config.server_port
    );
    axum::serve(listener, app).await.unwrap();
}
