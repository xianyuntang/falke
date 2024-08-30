use common::dto::auth::{SignInRequestDto, SignInResponseDto};
use reqwest::{Client, Response};
use std::collections::HashMap;
use std::error::Error;
use std::future::Future;
use url::Url;

pub struct ApiService {
    pub client: Client,
    pub base_url: Url,
    pub access_token: Option<String>,
}

impl ApiService {
    pub fn new(server: String) -> Self {
        let base_url = Url::parse(&server).unwrap();
        let client = Client::new();

        Self {
            client,
            base_url,
            access_token: None,
        }
    }

    pub async fn health_check(&self) -> Result<(), Box<dyn Error>> {
        let url = self.base_url.join("/api/ping")?;

        match self.client.get(url).send().await {
            Ok(response) => Ok(()),
            Err(..) => {
                tracing::error!("Health check failed.");
                panic!()
            }
        }
    }

    pub async fn sign_in(&mut self, email: String, password: String) -> Result<(), Box<dyn Error>> {
        let url = self.base_url.join("/api/auth/sign-in")?;
        let dto = SignInRequestDto { email, password };

        let response = self.client.post(url).json(&dto).send().await?;

        match response.status().is_success() {
            true => {
                let response: SignInResponseDto = response.json().await?;
                self.access_token = Option::from(response.access_token);
                Ok(())
            }
            false => {
                tracing::error!("Sign in failed.");
                panic!()
            }
        }
    }
}
