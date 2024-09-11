mod handlers;

use crate::infrastructure::server::AppState;
use axum::body::Bytes;
use axum::extract::{Host, OriginalUri, State, WebSocketUpgrade};
use axum::http::HeaderMap;
use axum::response::{IntoResponse, Response};
use axum::routing::any;
use axum::Router;
use common::infrastructure::error::ApiError;
use reqwest::Method;

pub(crate) fn create_route() -> Router<AppState> {
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
    let mut path = uri.path().to_string();
    if let Some(query) = uri.query() {
        path += "?";
        path += query;
    }
    let to_api = headers
        .get("x-falke-api")
        .map(|value| value == "yes")
        .unwrap_or_else(|| false);

    let api_endpoint = format!("{}:{}", settings.api_host, settings.api_port);

    let response: Response;
    if let Some(ws) = ws {
        response = ws.on_upgrade(move |socket| {
            handlers::ws::handler(socket, host, path, api_endpoint, to_api)
        })
    } else {
        response = handlers::http::handler(method, host, path, headers, body, api_endpoint, to_api)
            .await?;
    }

    Ok(response)
}
