use axum::Router;
use std::sync::Arc;
use subway::AppState;

mod app;

pub fn create_router() -> Router<Arc<AppState>> {
    Router::new().merge(app::create_route())
}
