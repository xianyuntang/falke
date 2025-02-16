use dotenv::dotenv;
use std::env;

pub enum EnvironmentVariable {
    DatabaseUrl,
    ApiPort,
    ApiSecret,
    ApiHost,
    GatewayPort,
    GatewayHost,
    GatewayCertPath,
}

impl EnvironmentVariable {
    fn as_str(&self) -> &str {
        match self {
            EnvironmentVariable::ApiPort => "API_PORT",
            EnvironmentVariable::ApiSecret => "API_SECRET",
            EnvironmentVariable::ApiHost => "API_HOST",
            EnvironmentVariable::GatewayHost => "GATEWAY_HOST",
            EnvironmentVariable::DatabaseUrl => "DATABASE_URL",
            EnvironmentVariable::GatewayPort => "GATEWAY_PORT",
            EnvironmentVariable::GatewayCertPath => "GATEWAY_CERT_PATH",
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
    pub gateway_port: u16,
    pub gateway_host: String,
    pub gateway_cert_path: Option<String>,
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

        let gateway_port: u16 = EnvironmentVariable::GatewayPort
            .get_value()
            .ok()
            .and_then(|value| value.parse::<u16>().ok())
            .unwrap_or_else(|| {
                tracing::info!("GATEWAY_PORT is not set or invalid, defaulting to 3000");
                3000
            });

        let gateway_host: String = EnvironmentVariable::GatewayHost
            .get_value()
            .unwrap_or_else(|err| {
                tracing::error!("GATEWAY_HOST must be set");
                panic!("{}", err)
            });

        let gateway_cert_path: Option<String> =
            EnvironmentVariable::GatewayCertPath.get_value().ok();

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
            gateway_port,
            gateway_host,
            gateway_cert_path,
            database_url,
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::new()
    }
}
