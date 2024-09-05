mod domain;
mod infrastructure;
use tower_layer::Layer;

use axum::extract::Request;
use axum::ServiceExt;
use common::infrastructure::db;
use common::infrastructure::settings::Settings;
use infrastructure::server;
use sea_orm::DatabaseConnection;
use tower_http::normalize_path::NormalizePathLayer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let settings = Settings::new();
    let db: DatabaseConnection = db::connect(&settings.database_url).await;

    let app = server::make_app(settings.clone(), db);
    let app = NormalizePathLayer::trim_trailing_slash().layer(app);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", settings.api_port))
        .await
        .unwrap_or_else(|err| panic!("{}", err));

    tracing::info!(
        "Application is running on http://0.0.0.0:{}",
        settings.api_port
    );
    axum::serve(listener, ServiceExt::<Request>::into_make_service(app))
        .await
        .unwrap();
}
