use axum::routing::{get, Router};
use std::sync::Arc;
use subway::AppState;

mod health_check;

pub fn create_route() -> Router<Arc<AppState>> {
    Router::new().route("/ping", get(health_check::handler()))
}
