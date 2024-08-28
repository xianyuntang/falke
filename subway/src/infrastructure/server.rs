use crate::domain;
use axum::Router;
use sea_orm::DatabaseConnection;
use subway::AppState;

pub fn make_app(db: DatabaseConnection) -> Router {
    let shared_state = AppState { db };
    let app = Router::new()
        .nest("/api", domain::create_router())
        .with_state(shared_state);
    app
}
