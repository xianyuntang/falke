use crate::converter::json::{header_map_to_json_string, json_string_to_header_map};
use crate::infrastructure::error::ApiError;
use axum::body::Body;
use axum::body::Bytes;
use axum::http::{HeaderMap, Method, StatusCode};
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ProxyRequest {
    pub id: String,
    pub tunnel_id: String,
    pub method: String,
    pub headers: String,
    pub path: String,
    pub body: Vec<u8>,
}

impl ProxyRequest {
    pub fn new(
        id: &str,
        tunnel_id: &str,
        method: Method,
        headers: HeaderMap,
        path: &str,
        body: Bytes,
    ) -> Self {
        Self {
            id: id.to_string(),
            tunnel_id: tunnel_id.to_string(),
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

impl axum::response::IntoResponse for ProxyResponse {
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
    fn into_response(
        self,
    ) -> impl std::future::Future<Output = Result<axum::response::Response, ApiError>> + Send;
}

pub struct ReqwestResponse(reqwest::Response);

impl ReqwestResponse {
    pub fn new(response: reqwest::Response) -> Self {
        ReqwestResponse(response)
    }
}

impl IntoResponseAsync for ReqwestResponse {
    async fn into_response(self) -> Result<axum::response::Response, ApiError> {
        let reqwest_response = self.0;

        let response_status = reqwest_response.status().clone();
        let response_headers = reqwest_response.headers().clone();
        let response_body = Body::from(reqwest_response.bytes().await?);

        let mut response = axum::response::Response::builder().status(response_status);

        for (name, value) in response_headers {
            if let Some(name) = name {
                if name != "transfer-encoding" {
                    response = response.header(name, value.to_str().unwrap());
                }
            }
        }

        Ok(response.body(response_body)?.into_response())
    }
}