use crate::domain::proxies::socket_manager::SOCKET_MANAGER;
use axum::extract::ws::{Message, WebSocket};
use common::dto::proxy::ProxyResponse;
use entity::entities::proxy;
use futures_util::{SinkExt, StreamExt};
use sea_orm::{DatabaseConnection, EntityTrait};
use tokio::sync::Mutex;

pub async fn handler(db: DatabaseConnection, socket: WebSocket, proxy_id: String) {
    let (mut sender, mut receiver) = socket.split();

    let proxy = proxy::Entity::find_by_id(&proxy_id).one(&db).await.unwrap();

    if proxy.is_none() {
        sender
            .send(Message::Text("Unauthorized".to_string()))
            .await
            .unwrap();
        sender.close().await.unwrap();
        return;
    }

    SOCKET_MANAGER
        .senders
        .insert(proxy_id.clone(), Mutex::new(sender));

    tokio::spawn(async move {
        while let Some(Ok(message)) = receiver.next().await {
            let proxy_response: ProxyResponse =
                serde_json::from_str(&message.to_text().unwrap()).unwrap();
            SOCKET_MANAGER
                .proxy_responses
                .insert(proxy_response.id.clone(), proxy_response);

            SOCKET_MANAGER.notify.notify_one();
        }
    });
}
