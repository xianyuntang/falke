use crate::infrastructure::server::AppState;
use axum::Router;

mod app;
mod auth;
mod tunnels;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .merge(app::create_route())
        .merge(auth::create_route())
        .merge(tunnels::create_route())
}
