use crate::domain;
use crate::infrastructure::settings::Settings;
use axum::{Extension, Router};
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub settings: Settings,
}

pub fn make_app(settings: Settings, db: DatabaseConnection) -> Router {
    let shared_state = AppState { db, settings };
    let app = Router::new()
        .nest("/api", domain::create_router())
        .with_state(shared_state);

    app
}
