mod domain;
mod infrastructure;
use crate::infrastructure::config::Config;
use axum::Router;
use sea_orm::{Database, DatabaseConnection};
use tracing::error;

fn init_tracing() {
    tracing_subscriber::fmt::init();
    std::panic::set_hook(Box::new(|panic_info| {
        if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            error!("panic occurred: {s}");
        } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            error!("panic occurred: {s}");
        } else {
            error!("panic occurred with unknown payload");
        }
    }));
}

#[tokio::main]
async fn main() {
    init_tracing();

    let config = Config::new();

    let app = Router::new().nest("/api", domain::create_router());

    let _db: DatabaseConnection = Database::connect(config.db_connection_url)
        .await
        .unwrap_or_else(|err| panic!("{}", err));

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.server_port))
        .await
        .unwrap_or_else(|err| panic!("{}", err));

    tracing::info!(
        "Application is running on http://0.0.0.0:{}",
        config.server_port
    );
    axum::serve(listener, app).await.unwrap();
}
