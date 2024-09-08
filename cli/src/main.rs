mod infrastructure;
mod services;

use crate::infrastructure::settings::Settings;
use crate::services::api_client::ApiService;
use clap::{Parser, Subcommand};
use std::process;
use std::sync::Arc;
use tokio::signal;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[clap(short, long, default_value = "https://pysubway.com")]
    server: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Http {
        endpoint: String,

        #[clap(short, long)]
        subdomain: Option<String>,
    },
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let cli = Cli::parse();
    let settings = Settings::new(&cli.server).await;

    let mut api_service = ApiService::new(settings);

    if api_service.health_check().await.is_err() {
        tracing::error!("Cannot connect to server {}", &cli.server);
        panic!()
    };

    match &cli.command {
        Commands::Http {
            endpoint,
            subdomain,
        } => {
            if api_service.sign_in().await.is_err() {
                tracing::error!("Sign in failed.");
                panic!()
            }

            if api_service.acquire_proxy(subdomain).await.is_err() {
                tracing::error!("Acquire proxy failed.");
                panic!()
            }

            let api_service = Arc::new(api_service);
            {
                let api_service_arc = Arc::clone(&api_service);
                tokio::spawn(async move {
                    signal::ctrl_c().await.unwrap();
                    println!("Received Ctrl+C, cleaning up...");

                    api_service_arc.release_proxy().await.unwrap();

                    process::exit(0);
                });
                let api_service_arc = Arc::clone(&api_service);

                if let Err(err) = api_service_arc.start_proxy(endpoint).await {
                    tracing::error!("Start proxy failed. {err:#?}");
                    panic!()
                };
            }
        }
    }
}
