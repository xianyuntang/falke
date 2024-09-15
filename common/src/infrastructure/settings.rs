use dotenv::dotenv;
use std::env;

pub enum EnvironmentVariable {
    DatabaseUrl,
    ApiPort,
    ApiSecret,
    ApiHost,
    ReverseProxyPort,
    ReverseProxyHost,
    ReverseProxyCertPath,
}

impl EnvironmentVariable {
    fn as_str(&self) -> &str {
        match self {
            EnvironmentVariable::ApiPort => "API_PORT",
            EnvironmentVariable::ApiSecret => "API_SECRET",
            EnvironmentVariable::ApiHost => "API_HOST",
            EnvironmentVariable::ReverseProxyHost => "REVERSE_PROXY_HOST",
            EnvironmentVariable::DatabaseUrl => "DATABASE_URL",
            EnvironmentVariable::ReverseProxyPort => "REVERSE_PROXY_PORT",
            EnvironmentVariable::ReverseProxyCertPath => "REVERSE_PROXY_CERT_PATH",
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
    pub api_host: String,
    pub reverse_proxy_port: u16,
    pub reverse_proxy_host: String,
    pub reverse_proxy_cert_path: Option<String>,
    pub database_url: String,
}

impl Settings {
    pub fn new() -> Settings {
        dotenv().ok();
        let api_host: String = EnvironmentVariable::ApiHost
            .get_value()
            .unwrap_or_else(|err| {
                tracing::error!("API_HOST must be set");
                panic!("{}", err)
            });

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

        let reverse_proxy_host: String = EnvironmentVariable::ReverseProxyHost
            .get_value()
            .unwrap_or_else(|err| {
                tracing::error!("REVERSE_PROXY_HOST must be set");
                panic!("{}", err)
            });

        let reverse_proxy_cert_path: Option<String> =
            EnvironmentVariable::ReverseProxyCertPath.get_value().ok();

        let database_url: String =
            EnvironmentVariable::DatabaseUrl
                .get_value()
                .unwrap_or_else(|err| {
                    tracing::error!("DB_CONNECTION_URL must be set");
                    panic!("{}", err)
                });

        Settings {
            api_port,
            api_secret,
            api_host,
            reverse_proxy_port,
            reverse_proxy_host,
            reverse_proxy_cert_path,
            database_url,
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::new()
    }
}
