use axum::extract::ws;
use axum::extract::ws::WebSocket;
use futures_util::stream::StreamExt;
use futures_util::SinkExt;
use tokio_tungstenite::{connect_async, tungstenite};
use url::Url;

pub async fn handler(api_endpoint: String, server_ws_stream: WebSocket, path: String) {
    let url = Url::parse(&format!("ws://{api_endpoint}/{path}"))
        .unwrap()
        .to_string();

    tracing::info!("Proxy WebSocket request to {}", url.as_str());

    let (mut server_sender, mut server_receiver) = server_ws_stream.split();

    let (client_ws_stream, _) = connect_async(&url).await.unwrap();
    let (mut client_sender, mut client_receiver) = client_ws_stream.split();

    let client_to_server = async {
        while let Some(Ok(message)) = client_receiver.next().await {
            if message.is_text() {
                if server_sender
                    .send(ws::Message::Text(message.to_string()))
                    .await
                    .is_err()
                {
                    break;
                }
            }
        }
    };

    let server_to_client = async {
        while let Some(Ok(message)) = server_receiver.next().await {
            if client_sender
                .send(tungstenite::Message::Text(message.into_text().unwrap()))
                .await
                .is_err()
            {
                break;
            }
        }
    };

    tokio::select! {
        _ = client_to_server =>(),
        _ = server_to_client =>(),
    }
}
