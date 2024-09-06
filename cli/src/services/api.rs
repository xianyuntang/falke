use crate::services::settings::Settings;
use anyhow::Result;
use common::converter::json::json_string_to_header_map;
use common::dto::auth::{
    SignInRequestDto, SignInResponseDto, ValidateTokenRequestDto, ValidateTokenResponseDto,
};
use common::dto::proxy::{
    AcquireProxyRequestDto, AcquireProxyResponseDto, IntoProxyResponseAsync, ProxyRequest,
    ProxyResponse, ReqwestResponse,
};
use futures_util::{SinkExt, StreamExt};
use reqwest::header::HeaderMap;
use reqwest::redirect::Policy;
use reqwest::{Body, Client, Method};
use std::io::Write;
use std::str::FromStr;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;

pub struct ApiService {
    pub settings: Settings,
    pub proxy_id: Option<String>,
    pub proxy_client: Client,
    pub client: Client,
    pub server: String,
    pub secure: bool,
}

impl ApiService {
    pub fn new(settings: Settings, server: &str, secure: bool) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert("x-subway-api", "yes".parse().unwrap());
        let proxy_client = Client::builder().redirect(Policy::none()).build().unwrap();

        let client = Client::builder().default_headers(headers).build().unwrap();

        Self {
            settings,
            proxy_client,
            client,
            server: server.to_string(),
            proxy_id: None,
            secure,
        }
    }

    pub async fn health_check(&self) -> Result<()> {
        let url = self.build_url("/api/ping", "http");

        match self.client.get(url).send().await {
            Ok(..) => Ok(()),
            Err(err) => {
                tracing::error!("Health check failed. {}", err);
                panic!()
            }
        }
    }

    pub async fn acquire_proxy(&mut self, subdomain: &Option<String>) -> Result<()> {
        let mut access_token = self.settings.read_token(self.server.clone()).await;
        let is_valid = self.validate_token(access_token.clone()).await?;

        if !is_valid {
            tracing::warn!("Token is invalid, please sign in again.");
            print!("Enter your email: ");
            std::io::stdout().flush()?;
            let mut email = String::new();
            std::io::stdin().read_line(&mut email)?;
            let email = email.trim();
            let password = rpassword::prompt_password("Enter your password: ")?;
            access_token = self.sign_in(email.to_string(), password.clone()).await?;
        };

        let url = self.build_url("/api/proxies", "http");

        let dto = AcquireProxyRequestDto {
            subdomain: subdomain.clone(),
        };

        let response = self
            .client
            .post(url)
            .header("authorization", access_token)
            .json(&dto)
            .send()
            .await?;

        if response.status().is_success() {
            let response: AcquireProxyResponseDto = response.json().await?;
            self.proxy_id = Option::from(response.id);
            tracing::info!(
                "Proxy on {}://{}",
                if self.secure { "https" } else { "http" },
                response.proxy_endpoint
            );
            Ok(())
        } else {
            tracing::error!("Acquire proxy failed. {}", response.status());
            panic!()
        }
    }

    pub async fn start_proxy(&self, endpoint: &str) -> Result<()> {
        let url = self.build_url(
            &format!("/api/proxies/{}/ws", self.proxy_id.clone().unwrap()),
            "ws",
        );

        let (ws_stream, _) = connect_async(url.as_str()).await?;

        let (mut sender, mut receiver) = ws_stream.split();

        while let Some(Ok(message)) = receiver.next().await {
            match self.transport(message, endpoint).await {
                Ok(proxy_response) => {
                    sender
                        .send(Message::Text(serde_json::to_string(&proxy_response)?))
                        .await?;
                }
                Err(error) => {
                    tracing::error!("{:#?}", error);
                }
            }
        }

        Ok(())
    }

    async fn transport(&self, message: Message, endpoint: &str) -> Result<ProxyResponse> {
        let proxy_request: ProxyRequest = serde_json::from_str(&message.to_string())?;

        let request_id = proxy_request.id.clone();
        let method = Method::from_str(&proxy_request.method)?;
        let headers = json_string_to_header_map(proxy_request.headers)?;

        let body = Body::from(proxy_request.body);

        let endpoint = self.build_local_url(endpoint, &proxy_request.path);

        let response = self
            .proxy_client
            .request(method.clone(), endpoint)
            .headers(headers)
            .body(body)
            .send()
            .await?;

        let response = ReqwestResponse::new(response);

        let res = response.into_proxy_response(request_id).await?;
        tracing::info!(
            "{} {}: /{} ",
            &res.status_code,
            method.as_str(),
            &proxy_request.path
        );

        Ok(res)
    }

    async fn validate_token(&self, access_token: String) -> Result<bool> {
        let url = self.build_url("/api/auth/validate-token", "http");
        let dto = ValidateTokenRequestDto { access_token };

        let response = self.client.post(url).json(&dto).send().await?;

        if response.status().is_success() {
            let response: ValidateTokenResponseDto = response.json().await?;
            Ok(response.is_valid)
        } else {
            let status = response.status();
            let error_message = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            tracing::error!(
                "Validate token failed. Status: {}. Error: {}",
                status,
                error_message
            );
            panic!()
        }
    }

    async fn sign_in(&mut self, email: String, password: String) -> Result<String> {
        let url = self.build_url("/api/auth/sign-in", "http");
        let dto = SignInRequestDto { email, password };

        let response = self.client.post(url).json(&dto).send().await?;

        if response.status().is_success() {
            let response: SignInResponseDto = response.json().await?;
            tracing::info!("Sign in successful.");
            self.settings
                .write_token(&self.server, &response.access_token)
                .await?;
            Ok(response.access_token)
        } else {
            let status = response.status();
            let error_message = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            tracing::error!(
                "Sign in failed. Status: {}. Error: {}",
                status,
                error_message
            );
            panic!()
        }
    }

    fn build_url(&self, endpoint: &str, protocol: &str) -> Url {
        if self.secure {
            Url::parse(&format!("{}s://{}{endpoint}", protocol, self.server)).unwrap()
        } else {
            Url::parse(&format!("{}://{}{endpoint}", protocol, self.server)).unwrap()
        }
    }

    fn build_local_url(&self, endpoint: &str, path: &str) -> Url {
        if !endpoint.starts_with("http") {
            Url::parse(&format!("http://{endpoint}/{path}")).unwrap()
        } else {
            Url::parse(&format!("{endpoint}/{path}")).unwrap()
        }
    }
}
