mod http;
mod ws;

use crate::infrastructure::server::AppState;
use axum::body::{Body, Bytes};
use axum::extract::{Host, Path, State, WebSocketUpgrade};
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
    ws: Option<WebSocketUpgrade>,
    State(AppState { settings }): State<AppState>,
    Path(WsParams { path }): Path<WsParams>,
    method: Method,
    host: Host,
    headers: HeaderMap,
    body: Bytes,
) -> Result<impl IntoResponse, ApiError> {
    let path = path.unwrap_or_else(|| "".parse().unwrap());

    let api_endpoint = format!("{}:{}", settings.api_host, settings.api_port);

    if let Some(ws) = ws {
        Ok(ws.on_upgrade(move |socket| ws::handler(api_endpoint, socket, path)))
    } else {
        let response;
        if let Some(reverse_proxy_api) = headers.get("x-subway-api") {
            if reverse_proxy_api.to_str().unwrap() == "yes" {
                response =
                    http::handler(method, host, path, headers, body, api_endpoint, true).await?;
            } else {
                response =
                    http::handler(method, host, path, headers, body, api_endpoint, false).await?;
            }
        } else {
            response =
                http::handler(method, host, path, headers, body, api_endpoint, false).await?;
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
}
