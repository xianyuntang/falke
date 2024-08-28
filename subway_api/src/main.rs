mod domain;
mod infrastructure;

use crate::infrastructure::config::SETTINGS;
use infrastructure::{db, logging, server};
use sea_orm::DatabaseConnection;

#[tokio::main]
async fn main() {
    logging::init_tracing();

    let db: DatabaseConnection = db::connect(&SETTINGS.database_url).await;

    let app = server::make_app(db);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", SETTINGS.server_port))
        .await
        .unwrap_or_else(|err| panic!("{}", err));

    tracing::info!(
        "Application is running on http://0.0.0.0:{}",
        SETTINGS.server_port
    );
    axum::serve(listener, app).await.unwrap();
}
