use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub async fn connect(url: &String) -> DatabaseConnection {
    let mut opt = ConnectOptions::new(url);
    opt.sqlx_logging(false)
        .sqlx_logging_level(log::LevelFilter::Debug);

    Database::connect(opt)
        .await
        .unwrap_or_else(|err| panic!("{}", err))
}
