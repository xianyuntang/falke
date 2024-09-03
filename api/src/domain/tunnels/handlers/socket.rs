use crate::domain::tunnels::socket_manager::SOCKET_MANAGER;
use axum::extract::ws::{Message, WebSocket};
use common::dto::proxy::ProxyResponse;
use entity::entities::tunnel;
use futures_util::{SinkExt, StreamExt};
use sea_orm::{DatabaseConnection, EntityTrait};
use tokio::sync::Mutex;

pub async fn handler(db: DatabaseConnection, socket: WebSocket, tunnel_id: String) {
    let (mut sender, mut receiver) = socket.split();

    let tunnel = tunnel::Entity::find_by_id(&tunnel_id)
        .one(&db)
        .await
        .unwrap();

    if tunnel.is_none() {
        sender
            .send(Message::Text("Unauthorized".to_string()))
            .await
            .unwrap();
        sender.close().await.unwrap();
        return;
    }

    SOCKET_MANAGER
        .senders
        .insert(tunnel_id.clone(), Mutex::new(sender));

    tokio::spawn(async move {
        while let Some(Ok(message)) = receiver.next().await {
            let tunnel_response: ProxyResponse =
                serde_json::from_str(&message.to_text().unwrap()).unwrap();
            SOCKET_MANAGER
                .tunnel_responses
                .insert(tunnel_response.id.clone(), tunnel_response);

            SOCKET_MANAGER.notify.notify_one();
        }
    });
}
