use dotenv::dotenv;
use std::env;

pub enum EnvironmentVariable {
    DatabaseUrl,
    ApiPort,
    ApiSecret,
    ReverseProxyPort,
    ReverseProxyToken,
    ApiUrl,
    ExternalUrl,
}

impl EnvironmentVariable {
    fn as_str(&self) -> &str {
        match self {
            EnvironmentVariable::ApiPort => "API_PORT",
            EnvironmentVariable::ApiSecret => "API_SECRET",
            EnvironmentVariable::ExternalUrl => "EXTERNAL_URL",
            EnvironmentVariable::DatabaseUrl => "DATABASE_URL",
            EnvironmentVariable::ReverseProxyPort => "REVERSE_PROXY_PORT",
            EnvironmentVariable::ReverseProxyToken => "REVERSE_PROXY_TOKEN",
            EnvironmentVariable::ApiUrl => "API_URL",
        }
    }

    pub fn get_value(&self) -> Result<String, env::VarError> {
        env::var(self.as_str())
    }
}

#[derive(Clone)]
pub struct Settings {
    pub api_port: u16,
    pub api_secret: String,
    pub api_url: String,
    pub reverse_proxy_port: u16,
    pub reverse_proxy_token: String,
    pub database_url: String,
    pub external_url: String,
}

impl Settings {
    pub fn new() -> Settings {
        dotenv().ok();
        let api_port: u16 = EnvironmentVariable::ApiPort
            .get_value()
            .ok()
            .and_then(|value| value.parse::<u16>().ok())
            .unwrap_or_else(|| {
                tracing::info!("API_PORT is not set or invalid, defaulting to 3000");
                3000
            });

        let api_secret: String = EnvironmentVariable::ApiSecret
            .get_value()
            .unwrap_or_else(|err| {
                tracing::error!("API_SECRET must be set");
                panic!("{}", err)
            });

        let reverse_proxy_port: u16 = EnvironmentVariable::ReverseProxyPort
            .get_value()
            .ok()
            .and_then(|value| value.parse::<u16>().ok())
            .unwrap_or_else(|| {
                tracing::info!("REVERSE_PROXY_PORT is not set or invalid, defaulting to 3000");
                3000
            });

        let reverse_proxy_token: String = EnvironmentVariable::ReverseProxyToken
            .get_value()
            .unwrap_or_else(|err| {
                tracing::error!("REVERSE_PROXY_TOKEN must be set");
                panic!("{}", err)
            });

        let database_url: String =
            EnvironmentVariable::DatabaseUrl
                .get_value()
                .unwrap_or_else(|err| {
                    tracing::error!("DB_CONNECTION_URL must be set");
                    panic!("{}", err)
                });

        let external_url: String =
            EnvironmentVariable::ExternalUrl
                .get_value()
                .unwrap_or_else(|err| {
                    tracing::error!("EXTERNAL_URL must be set");
                    panic!("{}", err)
                });

        let api_url: String = EnvironmentVariable::ApiUrl
            .get_value()
            .unwrap_or_else(|err| {
                tracing::error!("API_URL must be set");
                panic!("{}", err)
            });

        Settings {
            api_port,
            api_secret,
            api_url,
            reverse_proxy_port,
            reverse_proxy_token,
            external_url,
            database_url,
        }
    }
}
