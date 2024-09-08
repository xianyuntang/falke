use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::path::Path;
use tokio::fs;
use url::Url;

#[derive(Serialize, Deserialize, Debug)]
pub struct Credential {
    pub name: String,
    pub access_token: String,
}

pub struct Settings {
    pub server: Url,
    path: String,
    credentials: Vec<Credential>,
}

impl Settings {
    pub async fn new(server: &str) -> Self {
        let path = format!(
            "{}/.subway/.settings.json",
            dirs::home_dir().unwrap().to_str().unwrap()
        );
        let settings_path = Path::new(&path);

        if let Some(dir_path) = settings_path.parent() {
            if !dir_path.exists() {
                fs::create_dir_all(dir_path).await.unwrap();
            }
        }

        if !settings_path.exists() {
            let json_string = serde_json::to_string(&json!([])).unwrap();
            fs::write(&settings_path, json_string).await.unwrap()
        }

        let credential_string = fs::read(&settings_path).await.unwrap();
        let credentials =
            serde_json::from_str(&String::from_utf8(credential_string).unwrap()).unwrap();

        Self {
            server: Url::parse(server).unwrap(),
            path: settings_path.to_str().unwrap().to_string(),
            credentials,
        }
    }

    pub async fn read_token(&self) -> String {
        let access_token = self
            .credentials
            .iter()
            .find(|credential| credential.name == self.server.as_str())
            .map(|credential| credential.access_token.to_string())
            .unwrap_or("".to_string());

        access_token
    }

    pub async fn write_token(&mut self, access_token: &str) -> Result<()> {
        if let Some(exist_credential) = self
            .credentials
            .iter_mut()
            .find(|credential| credential.name == self.server.as_str())
        {
            exist_credential.access_token = access_token.to_string();
        } else {
            let new_credential = Credential {
                name: self.server.to_string(),
                access_token: access_token.to_string(),
            };
            self.credentials.push(new_credential);
        }

        let credential_string = serde_json::to_string(&json!(&self.credentials))?;

        fs::write(&self.path, credential_string).await?;

        Ok(())
    }
}
