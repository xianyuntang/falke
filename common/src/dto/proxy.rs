use crate::converter::json::{header_map_to_json_string, json_string_to_header_map};
use anyhow::Result;
use axum::body::Body;
use axum::body::Bytes;
use axum::http::{HeaderMap, Method, StatusCode};
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use std::future::Future;
use validator::Validate;

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct AcquireProxyRequestDto {
    pub subdomain: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AcquireProxyResponseDto {
    pub id: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProxyRequest {
    pub id: String,
    pub proxy_id: String,
    pub method: String,
    pub headers: String,
    pub path: String,
    pub body: Vec<u8>,
}

impl ProxyRequest {
    pub fn new(
        id: &str,
        proxy_id: &str,
        method: Method,
        headers: HeaderMap,
        path: &str,
        body: Bytes,
    ) -> Self {
        Self {
            id: id.to_string(),
            proxy_id: proxy_id.to_string(),
            method: method.to_string(),
            headers: header_map_to_json_string(headers).unwrap(),
            path: path.to_string(),
            body: body.to_vec(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProxyResponse {
    pub id: String,
    pub headers: String,
    pub status_code: u16,
    pub body: Vec<u8>,
}

impl ProxyResponse {
    pub fn new(id: &str, headers: HeaderMap, status_code: StatusCode, body: Bytes) -> Self {
        Self {
            id: id.to_string(),
            headers: header_map_to_json_string(headers).unwrap(),
            status_code: status_code.into(),
            body: body.to_vec(),
        }
    }
}

impl IntoResponse for ProxyResponse {
    fn into_response(self) -> axum::response::Response {
        let mut response = axum::response::Response::builder().status(self.status_code);
        let headers = json_string_to_header_map(self.headers).expect("Header format error");
        for (name, value) in headers {
            if let Some(name) = name {
                if name != "transfer-encoding" {
                    response = response.header(name, value.to_str().unwrap());
                }
            }
        }

        response
            .body(Body::from(self.body))
            .unwrap()
            .into_response()
    }
}
pub trait IntoResponseAsync {
    fn into_response(self) -> impl Future<Output = Result<axum::response::Response>> + Send;
}

pub trait IntoProxyResponseAsync {
    fn into_proxy_response(self, id: String) -> impl Future<Output = Result<ProxyResponse>> + Send;
}

pub struct ReqwestResponse(reqwest::Response);

impl ReqwestResponse {
    pub fn new(response: reqwest::Response) -> Self {
        ReqwestResponse(response)
    }
}

impl IntoResponseAsync for ReqwestResponse {
    async fn into_response(self) -> Result<axum::response::Response> {
        let reqwest_response = self.0;

        let response_status = reqwest_response.status().clone();
        let response_headers = reqwest_response.headers().clone();
        let response_body = Body::from(reqwest_response.bytes().await?);

        let mut response = axum::response::Response::builder().status(response_status);

        for (name, value) in response_headers {
            if let Some(name) = name {
                if name != "transfer-encoding" {
                    response = response.header(name, value.to_str()?);
                }
            }
        }

        Ok(response.body(response_body)?.into_response())
    }
}

impl IntoProxyResponseAsync for ReqwestResponse {
    async fn into_proxy_response(self, id: String) -> Result<ProxyResponse> {
        let response = self.0;
        let response_headers = response.headers().clone();
        let response_status = response.status();
        let response_body = response.bytes().await?;

        let proxy_response =
            ProxyResponse::new(&id, response_headers, response_status, response_body);

        Ok(proxy_response)
    }
}
