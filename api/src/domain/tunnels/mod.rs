use crate::domain::auth::jwt_validator::verify;
use crate::infrastructure::server::AppState;
use axum::body::Bytes;
use axum::extract::ws::WebSocketUpgrade;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, Method};
use axum::response::IntoResponse;
use axum::routing::{any, delete, get, post};
use axum::Router;
use common::infrastructure::error::ApiError;
use common::infrastructure::response::JsonResponse;
use serde::Deserialize;

pub mod handlers;
mod socket_manager;

pub fn create_route() -> Router<AppState> {
    Router::new().nest(
        "/tunnels",
        Router::new()
            .route("/", post(acquire_tunnel))
            .route("/:tunnel_id/ws", get(ws_handler))
            .route("/:tunnel_id", delete(|| async { "Hello World" }))
            .route("/:tunnel_id/proxy/", any(proxy))
            .route("/:tunnel_id/proxy/*path", any(proxy)),
    )
}

#[derive(Deserialize)]
struct WsParams {
    tunnel_id: String,
}

#[derive(Deserialize)]
struct ProxyParams {
    tunnel_id: String,
    path: Option<String>,
}

async fn acquire_tunnel(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, ApiError> {
    let authorization = headers.get("authorization");

    if authorization.is_none() {
        return Err(ApiError::UnauthorizedError);
    }

    let user_id = verify(
        &state.settings.api_secret,
        &authorization.unwrap().to_str().unwrap(),
    )?;

    let response = handlers::acquire_tunnel::handler(state.db, state.settings, &user_id).await?;

    Ok(JsonResponse(response))
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(AppState { db, .. }): State<AppState>,
    Path(WsParams { tunnel_id }): Path<WsParams>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handlers::socket::handler(db, socket, tunnel_id))
}

async fn proxy(
    Path(ProxyParams {
        tunnel_id,
        mut path,
    }): Path<ProxyParams>,
    State(state): State<AppState>,
    method: Method,
    headers: HeaderMap,
    body: Bytes,
) -> Result<impl IntoResponse, ApiError> {
    if let None = path {
        path = Option::from("".to_string())
    }
    let response =
        handlers::proxy::handler(state.db, tunnel_id, path.unwrap(), method, headers, body)
            .await?
            .into_response();

    Ok(response)
}
