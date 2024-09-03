use axum::body::Bytes;
use axum::extract::Host;
use axum::http::HeaderMap;
use common::dto::proxy::ReqwestResponse;
use common::infrastructure::error::ApiError;
use reqwest::Method;
use url::Url;

pub async fn handler(
    method: Method,
    host: Host,
    path: String,
    headers: HeaderMap,
    body: Bytes,
    api_endpoint: String,
    api: bool,
) -> Result<ReqwestResponse, ApiError> {
    let client = reqwest::Client::new();

    let url = Url::parse(&format!(
        "http://{}/{}",
        api_endpoint,
        if api {
            path.to_string()
        } else {
            let host = host.0.to_string();
            let tunnel_id = host.split('.').next().unwrap();
            format!("api/tunnels/{}/proxy/{}", tunnel_id, path)
        }
    ))?;

    tracing::info!("Proxy HTTP request {method} to {}", url.as_str());

    let response = client
        .request(method, url.as_str())
        .headers(headers)
        .body(body)
        .send()
        .await?;

    Ok(ReqwestResponse::new(response))
}
