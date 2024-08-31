use crate::infrastructure::server::AppState;
use axum::Router;

mod app;
mod auth;
mod tunnel;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .merge(app::create_route())
        .merge(auth::create_route())
        .merge(tunnel::create_route())
}
