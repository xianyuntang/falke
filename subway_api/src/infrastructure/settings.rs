use dotenv::dotenv;
use std::env;

pub enum EnvironmentVariable {
    ServerPort,
    DatabaseUrl,
}

impl EnvironmentVariable {
    fn as_str(&self) -> &str {
        match self {
            EnvironmentVariable::ServerPort => "SERVER_PORT",
            EnvironmentVariable::DatabaseUrl => "DATABASE_URL",
        }
    }

    pub fn get_value(&self) -> Result<String, env::VarError> {
        env::var(self.as_str())
    }
}

#[derive(Clone)]
pub struct Settings {
    pub server_port: u16,
    pub database_url: String,
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

        let database_url: String =
            EnvironmentVariable::DatabaseUrl
                .get_value()
                .unwrap_or_else(|err| {
                    tracing::error!("DB_CONNECTION_URL must be set");
                    panic!("{}", err)
                });

        Settings {
            server_port,
            database_url,
        }
    }
}
