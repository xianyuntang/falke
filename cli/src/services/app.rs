use crate::services::api_client::ApiService;
use anyhow::Result;
use url::Url;

impl ApiService {
    pub async fn health_check(&self) -> Result<()> {
        let url = Url::parse(&format!("{}/api/ping", &self.settings.server))?;
        self.client.get(url).send().await?;
        Ok(())
    }
}
