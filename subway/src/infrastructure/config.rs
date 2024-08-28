use dotenv::dotenv;
use once_cell::sync::Lazy;
use std::env;

pub static SETTINGS: Lazy<Settings> = Lazy::new(|| Settings::new());

pub enum EnvironmentVariable {
    ServerPort,
    DbConnectionUrl,
}

impl EnvironmentVariable {
    fn as_str(&self) -> &str {
        match self {
            EnvironmentVariable::ServerPort => "SERVER_PORT",
            EnvironmentVariable::DbConnectionUrl => "DB_CONNECTION_URL",
        }
    }

    pub fn get_value(&self) -> Result<String, env::VarError> {
        env::var(self.as_str())
    }
}

#[derive(Clone)]
pub struct Settings {
    pub server_port: u16,
    pub db_connection_url: String,
}

impl Settings {
    pub fn new() -> Settings {
        dotenv().ok();
        let server_port: u16 = EnvironmentVariable::ServerPort
            .get_value()
            .ok()
            .and_then(|value| value.parse::<u16>().ok())
            .unwrap_or_else(|| {
                tracing::info!("SERVER_PORT is not set or invalid, defaulting to 3000");
                3000
            });

        let db_connection_url: String = EnvironmentVariable::DbConnectionUrl
            .get_value()
            .unwrap_or_else(|err| {
                tracing::error!("DB_CONNECTION_URL must be set");
                panic!("{}", err)
            });

        Settings {
            server_port,
            db_connection_url,
        }
    }
}
