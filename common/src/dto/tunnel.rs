use crate::converter::json::header_map_to_json_string;
use axum::body::Bytes;
use axum::http::{HeaderMap, Method, StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TunnelRequest {
    pub id: String,
    pub tunnel_id: String,
    pub method: String,
    pub headers: String,
    pub upgrade: String,
    pub body: Vec<u8>,
}

impl TunnelRequest {
    pub fn new(
        id: &str,
        tunnel_id: &str,
        method: Method,
        headers: HeaderMap,
        upgrade: &str,
        body: Bytes,
    ) -> Self {
        Self {
            id: id.to_string(),
            tunnel_id: tunnel_id.to_string(),
            method: method.to_string(),
            headers: header_map_to_json_string(headers).unwrap(),
            upgrade: upgrade.to_string(),
            body: body.to_vec(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TunnelResponse {
    pub id: String,
    pub headers: String,
    pub status_code: u16,
    pub body: Vec<u8>,
}

impl TunnelResponse {
    pub fn new(id: &str, headers: HeaderMap, status_code: StatusCode, body: Bytes) -> Self {
        Self {
            id: id.to_string(),
            headers: header_map_to_json_string(headers).unwrap(),
            status_code: status_code.into(),
            body: body.to_vec(),
        }
    }
}
