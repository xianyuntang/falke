use crate::domain::tunnels::socket_manager::SOCKET_MANAGER;
use axum::body::Bytes;
use axum::extract::ws::Message;
use axum::http::{HeaderMap, Method};
use common::dto::proxy::{ProxyRequest, ProxyResponse};
use common::infrastructure::error::ApiError;
use entity::entities::tunnel;
use futures_util::SinkExt;
use nanoid::nanoid;
use sea_orm::{DatabaseConnection, EntityTrait};

pub async fn handler(
    db: DatabaseConnection,
    tunnel_id: String,
    path: String,
    method: Method,
    headers: HeaderMap,
    body: Bytes,
) -> Result<ProxyResponse, ApiError> {
    let tunnel = tunnel::Entity::find_by_id(&tunnel_id).one(&db).await?;

    if tunnel.is_none() {
        return Err(ApiError::NotFoundError);
    }

    let sender = SOCKET_MANAGER.senders.get(&tunnel_id);

    if let Some(sender_ref) = sender {
        tracing::info!("Redirect request {} to path /{}", method.as_str(), &path);

        let id = nanoid!();
        let sender_mutex = sender_ref.value();
        let mut sender = sender_mutex.lock().await;
        let tunnel_request = ProxyRequest::new(&id, &tunnel_id, method, headers, &path, body);

        sender
            .send(Message::Text(serde_json::to_string(&tunnel_request)?))
            .await?;

        loop {
            SOCKET_MANAGER.notify.notified().await;
            if let Some(tunnel_response) = SOCKET_MANAGER.tunnel_responses.remove(&id) {
                return Ok(tunnel_response.1);
            }
        }
    }

    Err(ApiError::NotFoundError)
}
