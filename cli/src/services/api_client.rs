use http::{HeaderName, HeaderValue};
use reqwest::header::HeaderMap;
use reqwest::redirect::Policy;
use reqwest::Client;

use crate::infrastructure::settings::Settings;

pub struct ApiService {
    pub proxy_client: Client,
    pub client: Client,
    pub settings: Settings,
    pub proxy_id: Option<String>,
}

impl ApiService {
    pub fn new(settings: Settings) -> Self {
        let headers = vec![("x-subway-api".to_string(), "yes".to_string())];
        let proxy_client = Self::build_client(None);
        let client = Self::build_client(Some(headers));

        Self {
            proxy_client,
            client,
            settings,
            proxy_id: None,
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
}
