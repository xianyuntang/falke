use common::dto::auth::{AcquireTunnelResponseDto, SignInRequestDto, SignInResponseDto};
use futures_util::{SinkExt, StreamExt};

use crate::services::credential::{read_token, write_token};
use common::converter::json::json_string_to_header_map;
use common::dto::tunnel::{TunnelRequest, TunnelResponse};
use reqwest::header::HeaderMap;
use reqwest::{Body, Client, Method};
use std::error::Error;
use std::str::FromStr;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;

pub struct ApiService {
    pub tunnel_id: Option<String>,
    pub client: Client,
    pub server: String,
    pub secure: bool,
}

impl ApiService {
    pub fn new(server: String, secure: bool) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert("x-subway-api", "yes".parse().unwrap());
        let client = Client::builder().default_headers(headers).build().unwrap();

        Self {
            client,
            server,
            secure,
            tunnel_id: None,
        }
    }

    pub async fn health_check(&self) -> Result<(), Box<dyn Error>> {
        let url = self.build_url("/api/ping", "http");

        match self.client.get(url).send().await {
            Ok(..) => Ok(()),
            Err(..) => {
                tracing::error!("Health check failed.");
                panic!()
            }
        }
    }

    pub async fn sign_in(&mut self, email: String, password: String) -> Result<(), Box<dyn Error>> {
        let url = self.build_url("/api/auth/sign-in", "http");
        let dto = SignInRequestDto { email, password };

        let response = self.client.post(url).json(&dto).send().await?;
        match response.status().is_success() {
            true => {
                let response: SignInResponseDto = response.json().await?;
                write_token(response.access_token).await?;
                Ok(())
            }
            false => {
                tracing::error!("Sign in failed.");
                panic!()
            }
        }
    }

    pub async fn acquire_tunnel(&mut self) -> Result<(), Box<dyn Error>> {
        let url = self.build_url("/api/tunnels", "http");
        let access_token = read_token().await?;
        let response = self
            .client
            .post(url)
            .header("authorization", access_token)
            .send()
            .await?;
        match response.status().is_success() {
            true => {
                let response: AcquireTunnelResponseDto = response.json().await?;
                self.tunnel_id = Option::from(response.id);
                tracing::info!("Proxy on {}", response.proxy_endpoint);
                Ok(())
            }
            false => {
                tracing::error!("Acquire tunnel failed.");
                panic!()
            }
        }
    }

    pub async fn start_proxy(
        &self,
        local_host: &str,
        local_port: &u16,
    ) -> Result<(), Box<dyn Error>> {
        let url = self.build_url(
            &format!("/api/tunnels/{}/ws", self.tunnel_id.clone().unwrap()),
            "ws",
        );

        let (ws_stream, _) = connect_async(url.as_str()).await.unwrap();

        let (mut sender, mut receiver) = ws_stream.split();

        while let Some(Ok(message)) = receiver.next().await {
            match self.transport(message, local_host, local_port).await {
                Ok(tunnel_response) => {
                    sender
                        .send(Message::Text(
                            serde_json::to_string(&tunnel_response).unwrap(),
                        ))
                        .await?;
                }
                Err(error) => {
                    tracing::error!("{error}");
                    panic!()
                }
            }
        }

        Ok(())
    }

    async fn transport(
        &self,
        message: Message,
        local_host: &str,
        local_port: &u16,
    ) -> Result<TunnelResponse, Box<dyn Error>> {
        let tunnel_request: TunnelRequest = serde_json::from_str(&message.to_string())?;

        let method = Method::from_str(&tunnel_request.method)?;
        let headers = json_string_to_header_map(tunnel_request.headers)?;

        let body = Body::from(tunnel_request.body);

        let url = Url::parse(&format!(
            "http://{local_host}:{local_port}/{}",
            tunnel_request.upgrade
        ))?;

        tracing::info!("Redirect request {} to {} ", method.as_str(), url.as_str());

        let response = self
            .client
            .request(method, url)
            .headers(headers)
            .body(body)
            .send()
            .await?;

        let tunnel_response = TunnelResponse::new(
            &tunnel_request.id.clone(),
            response.headers().clone(),
            response.status(),
            response.bytes().await?,
        );
        Ok(tunnel_response)
    }

    fn build_url(&self, endpoint: &str, protocol: &str) -> Url {
        match self.secure {
            true => Url::parse(&format!("{}s://{}{endpoint}", protocol, self.server)).unwrap(),
            false => Url::parse(&format!("{}://{}{endpoint}", protocol, self.server)).unwrap(),
        }
    }
}
