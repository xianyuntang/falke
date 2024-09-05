mod domain;
mod infrastructure;

use axum::extract::Request;
use axum::ServiceExt;
use common::infrastructure::settings::Settings;
use infrastructure::server;
use tower_http::normalize_path::NormalizePathLayer;
use tower_layer::Layer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let settings = Settings::new();

    let app = server::make_app(settings.clone());

    let app = NormalizePathLayer::trim_trailing_slash().layer(app);

    let listener =
        tokio::net::TcpListener::bind(format!("0.0.0.0:{}", settings.reverse_proxy_port))
            .await
            .unwrap_or_else(|err| panic!("{}", err));

    tracing::info!(
        "Application is running on http://0.0.0.0:{}",
        settings.reverse_proxy_port
    );
    axum::serve(listener, ServiceExt::<Request>::into_make_service(app))
        .await
        .unwrap();
}
