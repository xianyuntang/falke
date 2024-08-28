use sea_orm::{Database, DatabaseConnection};

pub async fn connect(url: String) -> DatabaseConnection {
    Database::connect(url)
        .await
        .unwrap_or_else(|err| panic!("{}", err))
}
