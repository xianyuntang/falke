use axum::Router;
use sea_orm::{Database, DatabaseConnection};

fn make_app() -> Router {
    let app = Router::new();
    app
}

#[tokio::main]
async fn main() {
    let app = make_app();
    let _db: DatabaseConnection =
        Database::connect("postgres://username:password@localhost/database")
            .await
            .expect("Failed to connect to database");

    tracing_subscriber::fmt::init();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
