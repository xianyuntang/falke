use crate::domain;
use crate::infrastructure::settings::Settings;
use axum::{Extension, Router};
use sea_orm::DatabaseConnection;

pub fn make_app(settings: Settings, db: DatabaseConnection) -> Router {
    let app = Router::new()
        .nest("/api", domain::create_router())
        .layer(Extension(settings))
        .layer(Extension(db));

    app
}
