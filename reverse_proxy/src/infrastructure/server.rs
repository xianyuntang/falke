use crate::domain;
use axum::Router;
use common::infrastructure::settings::Settings;

#[derive(Clone)]
pub struct AppState {
    pub settings: Settings,
}

pub fn make_app(settings: Settings) -> Router {
    let shared_state = AppState { settings };
    let app = Router::new()
        .nest("/", domain::create_router())
        .with_state(shared_state);

    app
}
