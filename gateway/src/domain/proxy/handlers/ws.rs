use axum::extract::ws::WebSocket;
use axum::extract::{ws, Host};
use futures_util::stream::StreamExt;
use futures_util::SinkExt;
use tokio_tungstenite::{connect_async, tungstenite};
use url::Url;

pub async fn handler(
    server_ws_stream: WebSocket,
    host: Host,
    path: String,
    api_endpoint: String,
    to_api: bool,
) {
    let url = Url::parse(&format!(
        "ws://{}{}",
        api_endpoint,
        if to_api {
            path.to_string()
        } else {
            let host = host.0.to_string();
            let proxy_id = host.split('.').next().unwrap();
            format!("/api/proxies/{}/transport{}", proxy_id, path)
        }
    ))
    .unwrap()
    .to_string();

    tracing::info!("Proxy WebSocket request to {}", url.as_str());

    tokio::spawn(async move {
        let (mut server_sender, mut server_receiver) = server_ws_stream.split();

        let (client_ws_stream, _) = connect_async(&url).await.unwrap();
        let (mut client_sender, mut client_receiver) = client_ws_stream.split();

        let client_to_server = async {
            while let Some(Ok(message)) = client_receiver.next().await {
                if let Err(err) = server_sender
                    .send(ws::Message::from(message.into_data()))
                    .await
                {
                    tracing::error!("{err:#?}");
                    break;
                }
            }
        };

        let server_to_client = async {
            while let Some(Ok(message)) = server_receiver.next().await {
                if let Err(err) = client_sender
                    .send(tungstenite::Message::from(message.into_data()))
                    .await
                {
                    tracing::error!("{err:#?}");
                    break;
                }
            }
        };

        tokio::select! {
            _ = client_to_server =>(),
            _ = server_to_client =>(),
        }
    });
}
