use crate::domain::proxies::socket_manager::SOCKET_MANAGER;
use anyhow::Result;
use axum::body::Bytes;
use axum::extract::ws::Message;
use axum::http::{HeaderMap, Method};
use common::dto::proxy::{ProxyRequest, ProxyResponse};
use common::infrastructure::error::ApiError;
use entity::entities::proxy;
use futures_util::SinkExt;
use nanoid::nanoid;
use sea_orm::{DatabaseConnection, EntityTrait};

pub async fn handler(
    db: DatabaseConnection,
    proxy_id: String,
    path: String,
    method: Method,
    headers: HeaderMap,
    body: Bytes,
) -> Result<ProxyResponse, ApiError> {
    let proxy = proxy::Entity::find_by_id(&proxy_id).one(&db).await?;

    if proxy.is_none() {
        return Err(ApiError::NotFoundError);
    }

    let sender = SOCKET_MANAGER.senders.get(&proxy_id);

    if let Some(sender_ref) = sender {
        tracing::info!("Redirect request {} to path {}", method.as_str(), &path);

        let id = nanoid!();
        let sender_mutex = sender_ref.value();
        let mut sender = sender_mutex.lock().await;
        let proxy_request = ProxyRequest::new(&id, &proxy_id, method, headers, &path, body);

        sender
            .send(Message::Text(serde_json::to_string(&proxy_request)?))
            .await?;

        loop {
            SOCKET_MANAGER.notify.notified().await;
            if let Some(proxy_response) = SOCKET_MANAGER.proxy_responses.remove(&id) {
                return Ok(proxy_response.1);
            }
        }
    }

    Err(ApiError::NotFoundError)
}
