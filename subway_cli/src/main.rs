mod services;

use crate::services::api::ApiService;
use clap::{Parser, Subcommand};
use std::collections::HashMap;
use std::error::Error;
use std::future::Future;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[clap(short, long, default_value = "http://localhost:3000")]
    server: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    SignIn {
        #[clap(short, long)]
        email: String,
    },
    Tunnel {
        local_port: u16,

        #[clap(short, long, default_value = "localhost")]
        local_host: String,
    },
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let cli = Cli::parse();
    let mut api_service = ApiService::new(cli.server);

    match api_service.health_check().await {
        Ok(response) => response,
        Err(err) => {
            tracing::error!("{:#?}", err.to_string());
            panic!()
        }
    };

    match &cli.command {
        Commands::SignIn { email } => {
            let password = rpassword::prompt_password("Enter your password: ").unwrap();
            match api_service.sign_in(email.clone(), password.clone()).await {
                Ok(response) => response,
                Err(err) => {
                    tracing::error!("{:#?}", err.to_string());
                    panic!()
                }
            };
        }
        Commands::Tunnel {
            local_host,
            local_port,
        } => {}
    }
}
