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
use http::{HeaderName, HeaderValue};
use reqwest::header::HeaderMap;
use reqwest::redirect::Policy;
use reqwest::{Body, Client, Method};
use std::io::Write;
use std::str::FromStr;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use tungstenite::client::IntoClientRequest;
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
        let headers = vec![("x-subway-api".to_string(), "yes".to_string())];
        let proxy_client = Self::build_client(None);
        let client = Self::build_client(Some(headers));

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
        let url = self.build_url("http", "/api/ping");
        self.client.get(url).send().await?;
        Ok(())
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

        let url = self.build_url("http", "/api/proxies");

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

    pub async fn release_proxy(&self) -> Result<()> {
        let url = self.build_url(
            "http",
            &format!("/api/proxies/{}", self.proxy_id.clone().unwrap()),
        );

        let access_token = self.settings.read_token(self.server.clone()).await;

        self.client
            .post(url)
            .header("authorization", access_token)
            .send()
            .await?;

        Ok(())
    }

    pub async fn start_proxy(&self, endpoint: &str) -> Result<()> {
        let proxy_id = self
            .proxy_id
            .clone()
            .ok_or(anyhow::Error::msg("No proxy id found"))?;

        let url = self
            .build_url("ws", &format!("/api/proxies/{}/ws", proxy_id))
            .to_string()
            .parse::<http::Uri>()?;

        let mut request = url.into_client_request()?;

        request.headers_mut().insert("x-subway-api", "yes".parse()?);

        let (ws_stream, _) = connect_async(request).await?;

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
            "{} {}: {} ",
            &res.status_code,
            method.as_str(),
            &proxy_request.path
        );

        Ok(res)
    }

    async fn validate_token(&self, access_token: String) -> Result<bool> {
        let url = self.build_url("http", "/api/auth/validate-token");
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
        let url = self.build_url("http", "/api/auth/sign-in");
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

    fn build_client(headers: Option<Vec<(String, String)>>) -> Client {
        let mut builder = Client::builder().redirect(Policy::none());
        if let Some(headers) = headers {
            let mut header_map = HeaderMap::new();
            for (key, value) in headers {
                header_map.insert(
                    HeaderName::try_from(key).unwrap(),
                    HeaderValue::try_from(value).unwrap(),
                );
            }
            builder = builder.default_headers(header_map);
        }
        builder.build().unwrap()
    }

    fn build_url(&self, protocol: &str, endpoint: &str) -> Url {
        if self.secure {
            Url::parse(&format!("{protocol}s://{}{endpoint}", self.server)).unwrap()
        } else {
            Url::parse(&format!("{protocol}://{}{endpoint}", self.server)).unwrap()
        }
    }

    fn build_local_url(&self, endpoint: &str, path: &str) -> Url {
        if !endpoint.starts_with("http") {
            Url::parse(&format!("http://{endpoint}{path}")).unwrap()
        } else {
            Url::parse(&format!("{endpoint}{path}")).unwrap()
        }
    }
}
