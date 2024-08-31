use crate::infrastructure::error::ApiError;
use serde_json::{json, Value};
use tokio::io;
use tokio::net::TcpListener;

pub async fn handler() -> Result<Value, ApiError> {
    let control_listener = create_listener().await?;
    let proxy_listener = create_listener().await?;
    let port = control_listener.local_addr()?.port();

    tokio::spawn(async move { listen(control_listener).await });
    tokio::spawn(async move { listen(proxy_listener).await });

    Ok(json!({"control_port":port}))
}

async fn create_listener() -> io::Result<TcpListener> {
    TcpListener::bind(("0.0.0.0", 1234)).await
}

async fn listen(listener: TcpListener) -> Result<(), ApiError> {
    loop {
        if let Ok((_stream, ..)) = listener.accept().await {}
    }
}

// async fn handle_connection() -> Result<(), ApiError> {}
