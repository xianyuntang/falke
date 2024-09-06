use crate::domain::auth::jwt_validator::verify;
use crate::infrastructure::server::AppState;
use axum::body::Bytes;
use axum::extract::ws::WebSocketUpgrade;
use axum::extract::{Json, Path, Query, State};
use axum::http::{HeaderMap, Method};
use axum::response::IntoResponse;
use axum::routing::{any, delete, get, post};
use axum::Router;
use common::dto::proxy::AcquireProxyRequestDto;
use common::infrastructure::error::ApiError;
use common::infrastructure::response::JsonResponse;
use serde::Deserialize;
use std::collections::HashMap;

mod handlers;
mod socket_manager;

pub fn create_route() -> Router<AppState> {
    Router::new().nest(
        "/proxies",
        Router::new()
            .route("/", post(acquire_proxy))
            .route("/:proxy_id/ws", get(ws_handler))
            .route("/:proxy_id", delete(|| async { "Hello World" }))
            .route("/:proxy_id/transport", any(proxy))
            .route("/:proxy_id/transport/*path", any(proxy)),
    )
}

#[derive(Deserialize)]
struct WsParams {
    proxy_id: String,
}

#[derive(Deserialize)]
struct ProxyParams {
    proxy_id: String,
    path: Option<String>,
}

async fn acquire_proxy(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(dto): Json<AcquireProxyRequestDto>,
) -> Result<impl IntoResponse, ApiError> {
    let authorization = headers
        .get("authorization")
        .ok_or_else(|| ApiError::UnauthorizedError)?;

    let user_id = verify(&state.settings.api_secret, &authorization.to_str().unwrap())?;

    let response =
        handlers::acquire_proxy::handler(state.db, state.settings, dto, &user_id).await?;

    Ok(JsonResponse(response))
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(AppState { db, .. }): State<AppState>,
    Path(WsParams { proxy_id }): Path<WsParams>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handlers::socket::handler(db, socket, proxy_id))
}

async fn proxy(
    State(state): State<AppState>,
    Path(ProxyParams { proxy_id, path }): Path<ProxyParams>,
    Query(params): Query<HashMap<String, String>>,
    method: Method,
    headers: HeaderMap,
    body: Bytes,
) -> Result<impl IntoResponse, ApiError> {
    let mut path = path.unwrap_or_else(|| "".to_string());

    if !params.is_empty() {
        path += "?";

        for (key, value) in params.iter() {
            path += &format!("{key}={value}&")
        }

        path.pop();
    };

    let response = handlers::proxy::handler(state.db, proxy_id, path, method, headers, body)
        .await?
        .into_response();

    Ok(response)
}
