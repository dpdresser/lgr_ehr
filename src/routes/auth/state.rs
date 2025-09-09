use anyhow::{Result, anyhow};
use secrecy::{ExposeSecret, SecretString};

use crate::utils::config::AppSettings;

#[derive(Clone, Debug)]
pub struct KeycloakState {
    base_url: String,
    realm: String,
    client_id: String,
    client_secret: Option<SecretString>,
}

impl KeycloakState {
    pub fn from_config(config: &AppSettings) -> Result<Self> {
        Ok(Self {
            base_url: config.key_cloak_base_url().to_string(),
            realm: config.key_cloak_realm().to_string(),
            client_id: config.key_cloak_client_id().to_string(),
            client_secret: config.key_cloak_client_secret().cloned(),
        })
    }

    pub fn token_endpoint(&self) -> String {
        format!(
            "{}/realms/{}/protocol/openid-connect/token",
            self.base_url, self.realm
        )
    }

    pub fn admin_base(&self) -> String {
        format!("{}/admin/realms/{}", self.base_url, self.realm)
    }

    pub fn users_endpoint(&self) -> String {
        format!("{}/users", self.admin_base())
    }

    pub async fn admin_token(&self) -> Result<String> {
        let secret = self
            .client_secret
            .as_ref()
            .ok_or_else(|| anyhow!("KEYCLOAK_CLIENT_SECRET must be set"))?;

        let form = [
            ("grant_type", "client_credentials"),
            ("client_id", &self.client_id),
            ("client_secret", secret.expose_secret()),
        ];

        let response = reqwest::Client::new()
            .post(self.token_endpoint())
            .form(&form)
            .send()
            .await
            .map_err(|e| anyhow!("Failed to send request to Keycloak: {}", e))?;

        if !response.status().is_success() {
            return Err(anyhow!(
                "Failed to get admin token from Keycloak: {}",
                response.status()
            ));
        }

        let v: serde_json::Value = response
            .json()
            .await
            .map_err(|e| anyhow!("Failed to parse Keycloak response: {}", e))?;

        v.get("access_token")
            .and_then(|t| t.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| anyhow!("Keycloak response missing access_token"))
    }
}
