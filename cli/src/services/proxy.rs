use crate::services::api_client::ApiService;
use anyhow::Result;
use common::converter::json::json_string_to_header_map;
use common::dto::proxy::{
    AcquireProxyRequestDto, AcquireProxyResponseDto, IntoProxyResponseAsync, ProxyRequest,
    ProxyResponse, ReqwestResponse,
};
use futures_util::{SinkExt, StreamExt};
use http::Method;
use http::Uri;
use reqwest::Body;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use tungstenite::client::IntoClientRequest;
use url::Url;

impl ApiService {
    pub async fn acquire_proxy(&mut self, subdomain: &Option<String>) -> Result<()> {
        let access_token = self.settings.read_token().await;

        let url = Url::join(&self.settings.server, "/api/proxies")?;

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
            let port_str = self
                .settings
                .server
                .port()
                .map_or("".to_string(), |port| format!(":{}", port));

            let domain = self.settings.server.host().unwrap().to_string();
            tracing::info!(
                "Proxy on {}://{}.{}{}",
                self.settings.server.scheme(),
                &response.id,
                if domain == "0.0.0.0" {
                    "localhost"
                } else {
                    &domain
                },
                port_str
            );
            self.proxy_id = Option::from(response.id);
            Ok(())
        } else {
            tracing::error!("Acquire proxy failed. {}", response.status());
            panic!()
        }
    }

    pub async fn release_proxy(&self) -> Result<()> {
        let url = Url::join(
            &self.settings.server,
            &format!("/api/proxies/{}", self.proxy_id.clone().unwrap()),
        )?;

        let access_token = self.settings.read_token().await;

        self.client
            .delete(url)
            .header("authorization", access_token)
            .send()
            .await?;

        Ok(())
    }

    pub async fn start_proxy(self: Arc<Self>, endpoint: &str) -> Result<()> {
        let proxy_id = self
            .proxy_id
            .clone()
            .ok_or(anyhow::Error::msg("No proxy id found"))?;

        let url = match self.settings.server.port() {
            None => Url::parse(&format!(
                "{}://{}/api/proxies/{}/ws",
                if self.settings.server.scheme() == "https" {
                    "wss"
                } else {
                    "ws"
                },
                self.settings.server.host().unwrap().to_string(),
                proxy_id
            ))?,
            Some(port) => Url::parse(&format!(
                "{}://{}:{}/api/proxies/{}/ws",
                if self.settings.server.scheme() == "https" {
                    "wss"
                } else {
                    "ws"
                },
                self.settings.server.host().unwrap().to_string(),
                port,
                proxy_id
            ))?,
        };

        let url = Uri::try_from(url.as_str())?;

        let mut request = url.into_client_request()?;
        request.headers_mut().insert("x-falke-api", "yes".parse()?);

        let endpoint = endpoint.to_string();
        let (ws_stream, _) = connect_async(request).await?;
        let (sender, mut receiver) = ws_stream.split();
        let endpoint = Arc::new(endpoint.clone());

        let sender = Arc::new(Mutex::new(sender));

        while let Some(Ok(message)) = receiver.next().await {
            let this = Arc::clone(&self);
            let endpoint = Arc::clone(&endpoint);
            let sender = Arc::clone(&sender);
            tokio::spawn(async move {
                if let Ok(proxy_response) = this.transport(message, &endpoint).await {
                    let message_vec = serde_json::to_vec(&proxy_response).unwrap_or(vec![]);
                    sender
                        .lock()
                        .await
                        .send(Message::from(message_vec))
                        .await
                        .unwrap_or_else(|err| tracing::error!("{err:#?}"));
                }
            });
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

    fn build_local_url(&self, endpoint: &str, path: &str) -> Url {
        if !endpoint.starts_with("http") {
            Url::parse(&format!("http://{endpoint}{path}")).unwrap()
        } else {
            Url::parse(&format!("{endpoint}{path}")).unwrap()
        }
    }
}
