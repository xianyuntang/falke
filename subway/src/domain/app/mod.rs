use axum::routing::{get, Router};

mod health_check;

pub fn create_route() -> Router {
    Router::new().route("/ping", get(health_check::handler()))
}
