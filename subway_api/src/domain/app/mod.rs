use axum::routing::{get, Router};
use subway_api::AppState;

mod health_check;

pub fn create_route() -> Router<AppState> {
    Router::new().route("/ping", get(health_check::handler))
}