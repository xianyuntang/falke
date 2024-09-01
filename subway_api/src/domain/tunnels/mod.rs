use crate::infrastructure::error::ApiError;
use crate::infrastructure::response::JsonResponse;
use crate::infrastructure::server::AppState;
use axum::body::Bytes;
use axum::extract::ws::WebSocketUpgrade;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, Method, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::{any, delete, get, post};
use axum::Router;
use common::converter::json::json_string_to_header_map;
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
            .route("/:tunnel_id/proxy", any(proxy))
            .route("/:tunnel_id/proxy/*upgrade", any(proxy)),
    )
}

#[derive(Deserialize)]
struct WsParams {
    tunnel_id: String,
}

#[derive(Deserialize)]
struct ProxyParams {
    tunnel_id: String,
    upgrade: Option<String>,
}

async fn acquire_tunnel(State(state): State<AppState>) -> Result<impl IntoResponse, ApiError> {
    let response = handlers::acquire_tunnel::handler(state.db).await?;

    Ok(JsonResponse(response))
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    Path(WsParams { tunnel_id }): Path<WsParams>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handlers::socket::handler(socket, tunnel_id))
}

async fn proxy(
    Path(ProxyParams {
        tunnel_id,
        mut upgrade,
    }): Path<ProxyParams>,
    State(state): State<AppState>,
    method: Method,
    headers: HeaderMap,
    body: Bytes,
) -> Result<impl IntoResponse, ApiError> {
    if let None = upgrade {
        upgrade = Option::from("".to_string())
    }
    let response =
        handlers::proxy::handler(state.db, tunnel_id, upgrade.unwrap(), method, headers, body)
            .await?;

    let status_code = StatusCode::from_u16(response.status_code)?;
    let headers = json_string_to_header_map(response.headers)?;
    let body = response.body;

    let mut response = Response::builder().status(status_code);

    for (header_name, header_value) in headers.iter() {
        response = response.header(header_name.to_string(), header_value.to_str().unwrap());
    }

    Ok(response.body(body).unwrap())
}