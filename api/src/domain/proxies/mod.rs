use crate::domain::auth::jwt_validator::verify;
use crate::infrastructure::server::AppState;
use crate::infrastructure::templates::HtmlTemplate;
use askama::Template;
use axum::body::Bytes;
use axum::extract::ws::WebSocketUpgrade;
use axum::extract::{Json, OriginalUri, Path, State};
use axum::http::{HeaderMap, Method};
use axum::response::IntoResponse;
use axum::routing::{any, delete, get, post};
use axum::Router;
use common::dto::proxy::AcquireProxyRequestDto;
use common::infrastructure::error::ApiError;
use common::infrastructure::response::JsonResponse;
use regex::Regex;
use serde::Deserialize;
use serde_json::json;

mod handlers;
mod socket_manager;

pub(crate) fn create_route() -> Router<AppState> {
    Router::new().nest(
        "/proxies",
        Router::new()
            .route("/", post(acquire_proxy))
            .route("/:proxy_id/ws", get(ws_handler))
            .route("/:proxy_id", delete(release_proxy))
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
}

#[derive(Deserialize)]
struct ReleaseProxyParams {
    proxy_id: String,
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

async fn release_proxy(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(ReleaseProxyParams { proxy_id }): Path<ReleaseProxyParams>,
) -> Result<impl IntoResponse, ApiError> {
    let authorization = headers
        .get("authorization")
        .ok_or_else(|| ApiError::UnauthorizedError)?;

    let user_id = verify(&state.settings.api_secret, &authorization.to_str().unwrap())?;

    handlers::release_proxy::handler(state.db, &proxy_id, &user_id).await?;

    Ok(JsonResponse(json!({"message":"ok"})))
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(AppState { db, .. }): State<AppState>,
    Path(WsParams { proxy_id }): Path<WsParams>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handlers::socket::handler(db, socket, proxy_id))
}

#[derive(Template)]
#[template(path = "404.html")]
pub struct NotfoundTemplate {}

async fn proxy(
    State(state): State<AppState>,
    Path(ProxyParams { proxy_id }): Path<ProxyParams>,
    OriginalUri(uri): OriginalUri,
    method: Method,
    headers: HeaderMap,
    body: Bytes,
) -> Result<impl IntoResponse, ApiError> {
    let re = Regex::new(r"/api/proxies/.+?/transport(.+)")?;
    let url = uri.to_string();

    let path = re
        .captures(&url)
        .and_then(|captures| captures.get(1))
        .map(|m| m.as_str())
        .unwrap_or_else(|| "/")
        .to_string();

    match handlers::proxy::handler(state.db, proxy_id, path, method, headers, body).await {
        Ok(response) => Ok(response.into_response()),
        Err(_) => Ok(HtmlTemplate(NotfoundTemplate {}).into_response()),
    }
}
