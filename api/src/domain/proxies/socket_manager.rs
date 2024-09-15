use axum::extract::ws::{Message, WebSocket};
use common::dto::proxy::ProxyResponse;
use dashmap::DashMap;
use futures_util::stream::SplitSink;
use std::sync::{Arc, LazyLock};
use tokio::sync::{Mutex, Notify};

pub static SOCKET_MANAGER: LazyLock<SocketManager> = LazyLock::new(SocketManager::new);

pub struct SocketManager {
    pub senders: Arc<DashMap<String, Mutex<SplitSink<WebSocket, Message>>>>,
    pub proxy_responses: Arc<DashMap<String, ProxyResponse>>,
    pub notify: Arc<Notify>,
}

impl SocketManager {
    fn new() -> Self {
        Self {
            senders: Arc::new(DashMap::new()),
            proxy_responses: Arc::new(DashMap::new()),
            notify: Arc::new(Notify::new()),
        }
    }
}
