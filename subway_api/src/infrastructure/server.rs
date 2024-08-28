use crate::domain;
use axum::Router;
use sea_orm::DatabaseConnection;
use subway_api::AppState;

pub fn make_app(db: DatabaseConnection) -> Router {
    let shared_state = AppState { db };
    let app = Router::new()
        .nest("/subway_api", domain::create_router())
        .with_state(shared_state);
    app
}
