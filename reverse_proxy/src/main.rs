mod domain;
mod infrastructure;
use axum::extract::Request;
use axum::ServiceExt;
use axum_server::tls_rustls::RustlsConfig;
use common::infrastructure::settings::Settings;
use infrastructure::server;
use std::net::SocketAddr;
use std::path::PathBuf;
use tower_http::normalize_path::NormalizePathLayer;
use tower_layer::Layer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    rustls::crypto::ring::default_provider()
        .install_default()
        .unwrap();

    let settings = Settings::new();

    let app = server::make_app(settings.clone());
    let app = NormalizePathLayer::trim_trailing_slash().layer(app);

    let addr = SocketAddr::from(([0, 0, 0, 0], settings.reverse_proxy_port));

    if let Some(cert_path) = settings.reverse_proxy_cert_path {
        tracing::info!(
            "Application is running on https://0.0.0.0:{}",
            settings.reverse_proxy_port
        );
        let rustls_config = RustlsConfig::from_pem_file(
            PathBuf::from(&cert_path).join("cert.pem"),
            PathBuf::from(&cert_path).join("key.pem"),
        )
        .await
        .unwrap();
        axum_server::bind_rustls(addr, rustls_config)
            .serve(ServiceExt::<Request>::into_make_service(app))
            .await
            .unwrap();
    } else {
        tracing::info!(
            "Application is running on http://0.0.0.0:{}",
            settings.reverse_proxy_port
        );
        axum_server::bind(addr)
            .serve(ServiceExt::<Request>::into_make_service(app))
            .await
            .unwrap();
    }
}
