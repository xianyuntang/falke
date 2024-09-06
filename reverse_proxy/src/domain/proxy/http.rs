use anyhow::Result;
use axum::body::Bytes;
use axum::extract::Host;
use axum::http::HeaderMap;
use axum::response::Response;
use common::dto::proxy::{IntoResponseAsync, ReqwestResponse};
use reqwest::redirect::Policy;
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
) -> Result<Response> {
    let client = reqwest::Client::builder()
        .redirect(Policy::none())
        .build()?;

    let url = Url::parse(&format!(
        "http://{}{}",
        api_endpoint,
        if api {
            path.to_string()
        } else {
            let host = host.0.to_string();
            let proxy_id = host.split('.').next().unwrap();
            format!("/api/proxies/{}/transport{}", proxy_id, path)
        }
    ))?;

    let response = client
        .request(method.clone(), url.clone())
        .headers(headers)
        .body(body)
        .send()
        .await?;

    let response = ReqwestResponse::new(response).into_response().await?;

    tracing::info!("{} {method} {}", &response.status(), url.as_str());

    Ok(response)
}
