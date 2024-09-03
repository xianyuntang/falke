use serde::de::DeserializeOwned;
use std::error::Error;

async fn handle_response<T: DeserializeOwned>(
    response: reqwest::Response,
) -> Result<T, Box<dyn Error>> {
    if response.status().is_success() {
        let result = response.json::<T>().await?;
        Ok(result)
    } else {
        let status = response.status();
        let error_message = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        tracing::error!(
            "Request failed. Status: {}. Error: {}",
            status,
            error_message
        );
        Err(anyhow::anyhow!("Request failed with status: {}", status).into())
    }
}
