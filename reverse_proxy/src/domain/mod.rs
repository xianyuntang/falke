use crate::infrastructure::server::AppState;
use axum::Router;

mod proxy;

pub fn create_router() -> Router<AppState> {
    Router::new().merge(proxy::create_route())
}
