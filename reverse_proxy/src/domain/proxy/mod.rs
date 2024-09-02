mod proxy;

use crate::infrastructure::server::AppState;
use axum::body::{Body, Bytes};
use axum::extract::{Host, Path, State};
use axum::http::HeaderMap;
use axum::response::{IntoResponse, Response};
use axum::routing::any;
use axum::Router;
use common::infrastructure::error::ApiError;
use reqwest::Method;
use serde::Deserialize;

pub fn create_route() -> Router<AppState> {
    Router::new()
        .route("/*path", any(proxy))
        .route("/", any(proxy))
}

#[derive(Deserialize)]
struct WsParams {
    path: Option<String>,
}

async fn proxy(
    State(AppState { settings }): State<AppState>,
    Path(WsParams { path }): Path<WsParams>,
    method: Method,
    host: Host,
    headers: HeaderMap,
    body: Bytes,
) -> Result<impl IntoResponse, ApiError> {
    let path = path.unwrap_or_else(|| "".parse().unwrap());

    let response;
    if let Some(reverse_proxy_token) = headers.get("X-SUBWAY-TOKEN") {
        if reverse_proxy_token.to_str().unwrap() == settings.reverse_proxy_token {
            response =
                proxy::handler(method, host, path, headers, body, settings.api_url, true).await?;
        } else {
            response =
                proxy::handler(method, host, path, headers, body, settings.api_url, false).await?;
        }
    } else {
        response =
            proxy::handler(method, host, path, headers, body, settings.api_url, false).await?;
    }

    let response_status = response.status().clone();
    let response_headers = response.headers().clone();
    let response_body = Body::from(response.bytes().await?);

    let mut response = Response::builder().status(response_status);

    for (name, value) in response_headers {
        response = response.header(name.unwrap(), value.to_str().unwrap());
    }

    Ok(response.body(response_body).unwrap())
}
