use crate::domain::tunnels::socket_manager::SOCKET_MANAGER;
use axum::extract::ws::WebSocket;
use common::dto::tunnel::TunnelResponse;
use futures_util::StreamExt;
use tokio::sync::Mutex;

pub async fn handler(socket: WebSocket, tunnel_id: String) {
    let (sender, mut receiver) = socket.split();

    SOCKET_MANAGER
        .senders
        .insert(tunnel_id.clone(), Mutex::new(sender));

    tokio::spawn(async move {
        while let Some(Ok(message)) = receiver.next().await {
            let tunnel_response: TunnelResponse =
                serde_json::from_str(&message.to_text().unwrap()).unwrap();
            SOCKET_MANAGER
                .tunnel_responses
                .insert(tunnel_response.id.clone(), tunnel_response);

            SOCKET_MANAGER.notify.notify_one();
        }
    });
}
