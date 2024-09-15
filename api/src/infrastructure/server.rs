use crate::domain;
use axum::Router;
use common::infrastructure::settings::Settings;
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub settings: Settings,
}

pub fn make_app(settings: Settings, db: DatabaseConnection) -> Router {
    let shared_state = AppState { db, settings };
    Router::new()
        .nest("/api", domain::create_router())
        .with_state(shared_state)
}
