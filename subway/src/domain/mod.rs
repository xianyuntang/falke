use axum::Router;

mod app;

pub fn create_router() -> Router {
    Router::new().merge(app::create_route())
}
