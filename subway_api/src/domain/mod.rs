use axum::Router;

mod app;
mod auth;

pub fn create_router() -> Router {
    Router::new()
        .merge(app::create_route())
        .merge(auth::create_route())
}
