mod domain;
mod infrastructure;

use infrastructure::{db, server, settings::Settings};
use sea_orm::DatabaseConnection;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let settings = Settings::new();
    let db: DatabaseConnection = db::connect(&settings.database_url).await;

    let app = server::make_app(settings.clone(), db);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", settings.server_port))
        .await
        .unwrap_or_else(|err| panic!("{}", err));

    tracing::info!(
        "Application is running on http://0.0.0.0:{}",
        settings.server_port
    );
    axum::serve(listener, app).await.unwrap();
}
