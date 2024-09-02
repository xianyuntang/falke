use axum::body::Bytes;
use axum::extract::Host;
use axum::http::HeaderMap;
use common::infrastructure::error::ApiError;
use reqwest::{Method, Response};
use url::Url;

pub async fn handler(
    method: Method,
    host: Host,
    path: String,
    headers: HeaderMap,
    body: Bytes,
    api_endpoint: String,
    api: bool,
) -> Result<Response, ApiError> {
    let client = reqwest::Client::new();

    let url = match api {
        true => Url::parse(&format!("http://{api_endpoint}/{path}"))?,
        false => {
            let host = host.0.to_string();
            let tunnel_id = host.split('.').next().unwrap();
            Url::parse(&format!(
                "http://{api_endpoint}/api/tunnels/{tunnel_id}/proxy/{path}"
            ))?
        }
    };
    tracing::info!("Proxy HTTP request {method} to {}", url.as_str());

    let response = client
        .request(method, url.as_str())
        .headers(headers)
        .body(body)
        .send()
        .await?;

    Ok(response)
}
