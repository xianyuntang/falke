use axum::Router;
use subway::AppState;

mod app;

pub fn create_router() -> Router<AppState> {
    Router::new().merge(app::create_route())
}
