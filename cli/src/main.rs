mod services;

use crate::services::api::ApiService;
use crate::services::settings::Settings;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[clap(short, long, default_value = "pysubway.com")]
    server: String,

    #[clap(short, long, default_value = "true")]
    use_ssl: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Http {
        local_port: u16,

        #[clap(short, long, default_value = "localhost")]
        local_host: String,
    },
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let cli = Cli::parse();
    let settings = Settings::new().await;

    let mut api_service = ApiService::new(settings, &cli.server, cli.use_ssl);

    match api_service.health_check().await {
        Ok(response) => response,
        Err(..) => {
            tracing::error!("Cannot connect to server {}", &cli.server);
            panic!()
        }
    };

    match &cli.command {
        Commands::Http {
            local_host,
            local_port,
        } => {
            match api_service.acquire_proxy().await {
                Ok(response) => response,
                Err(err) => {
                    tracing::error!("{:#?}", err.to_string());
                    panic!()
                }
            };

            match api_service.start_proxy(local_host, local_port).await {
                Ok(response) => response,
                Err(err) => {
                    tracing::error!("{:#?}", err.to_string());
                    panic!()
                }
            };
        }
    }
}
