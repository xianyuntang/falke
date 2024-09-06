mod http;
mod ws;

use crate::infrastructure::server::AppState;
use axum::body::Bytes;
use axum::extract::{Host, OriginalUri, State, WebSocketUpgrade};
use axum::http::HeaderMap;
use axum::response::{IntoResponse, Response};
use axum::routing::any;
use axum::Router;
use common::infrastructure::error::ApiError;
use reqwest::Method;

pub fn create_route() -> Router<AppState> {
    Router::new()
        .route("/*path", any(proxy))
        .route("/", any(proxy))
}

async fn proxy(
    ws: Option<WebSocketUpgrade>,
    State(AppState { settings }): State<AppState>,
    OriginalUri(uri): OriginalUri,
    method: Method,
    host: Host,
    headers: HeaderMap,
    body: Bytes,
) -> Result<impl IntoResponse, ApiError> {
    let path = uri.to_string();

    let api_endpoint = format!("{}:{}", settings.api_host, settings.api_port);

    if let Some(ws) = ws {
        Ok(ws.on_upgrade(move |socket| ws::handler(api_endpoint, socket, path)))
    } else {
        let response: Response;
        if let Some(reverse_proxy_api) = headers.get("x-subway-api") {
            // To api endpoint
            if reverse_proxy_api.to_str()? == "yes" {
                response =
                    http::handler(method, host, path, headers, body, api_endpoint, true).await?
            }
            // To proxy endpoint
            else {
                response =
                    http::handler(method, host, path, headers, body, api_endpoint, false).await?
            }
        } else {
            // To proxy endpoint
            response = http::handler(method, host, path, headers, body, api_endpoint, false).await?
        }

        Ok(response)
    }
}
