use axum::http::{HeaderMap, HeaderName, HeaderValue};
use std::collections::HashMap;
use std::str::FromStr;

pub fn header_map_to_json_string(headers: HeaderMap) -> Result<String, serde_json::Error> {
    let mut map = HashMap::new();

    for (key, value) in headers.iter() {
        let key_string = key.as_str().to_string();
        let value_string = value.to_str().unwrap_or("").to_string();
        map.insert(key_string, value_string);
    }

    serde_json::to_string(&map)
}

pub fn json_string_to_header_map(json_string: String) -> Result<HeaderMap, serde_json::Error> {
    let mut headers = HeaderMap::new();
    let map: HashMap<String, String> = serde_json::from_str(&json_string)?;
    for (key, value) in map.iter() {
        headers.insert(
            HeaderName::from_str(key).unwrap(),
            HeaderValue::from_str(value).unwrap(),
        );
    }

    Ok(headers)
}
