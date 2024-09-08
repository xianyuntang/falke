use crate::services::api_client::ApiService;
use anyhow::Result;
use common::dto::auth::{
    SignInRequestDto, SignInResponseDto, ValidateTokenRequestDto, ValidateTokenResponseDto,
};
use std::io::Write;
use url::Url;

impl ApiService {
    pub async fn sign_in(&mut self) -> Result<()> {
        let access_token = self.settings.read_token().await;
        let is_valid = self.validate_token(access_token.clone()).await?;

        if is_valid {
            return Ok(());
        };

        tracing::warn!("Token is invalid, please sign in again.");
        print!("Enter your email: ");
        std::io::stdout().flush()?;
        let mut email = String::new();
        std::io::stdin().read_line(&mut email)?;
        let email = email.trim();
        let password = rpassword::prompt_password("Enter your password: ")?;

        let url = format!("{}/api/auth/sign-in", &self.settings.server);

        let dto = SignInRequestDto {
            email: email.to_string(),
            password,
        };

        let response = self.client.post(url).json(&dto).send().await?;

        if response.status().is_success() {
            let response: SignInResponseDto = response.json().await?;
            tracing::info!("Sign in successful.");
            self.settings.write_token(&response.access_token).await?;
            Ok(())
        } else {
            let status = response.status();
            let error_message = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            tracing::error!(
                "Sign in failed. Status: {}. Error: {}",
                status,
                error_message
            );
            panic!()
        }
    }

    async fn validate_token(&self, access_token: String) -> Result<bool> {
        let url = Url::parse(&format!(
            "{}/api/auth/validate-token",
            self.settings.server.as_str()
        ))?;

        let dto = ValidateTokenRequestDto { access_token };

        let response = self.client.post(url).json(&dto).send().await?;

        if response.status().is_success() {
            let response: ValidateTokenResponseDto = response.json().await?;
            Ok(response.is_valid)
        } else {
            let status = response.status();
            let error_message = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            tracing::error!(
                "Validate token failed. Status: {}. Error: {}",
                status,
                error_message
            );
            panic!()
        }
    }
}
