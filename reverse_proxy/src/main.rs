mod domain;
mod infrastructure;

use common::infrastructure::settings::Settings;
use infrastructure::server;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let settings = Settings::new();

    let app = server::make_app(settings.clone());

    let listener =
        tokio::net::TcpListener::bind(format!("0.0.0.0:{}", settings.reverse_proxy_port))
            .await
            .unwrap_or_else(|err| panic!("{}", err));

    tracing::info!(
        "Application is running on http://0.0.0.0:{}",
        settings.reverse_proxy_port
    );
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
