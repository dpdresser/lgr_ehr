use reqwest::StatusCode;
use secrecy::{ExposeSecret, SecretString};

use crate::{
    domain::{
        error::app_error::{AppResult, AuthProviderError},
        interfaces::auth_provider::AuthProvider,
        types::{
            email::Email,
            password::Password,
            user::{User, UserUpdate},
        },
    },
    utils::config::AppSettings,
};

pub struct KeycloakEndpoints {
    pub admin_enpoint: String,
    pub token_endpoint: String,
    pub users_endpoint: String,
    pub client_id: String,
    pub client_secret: Option<SecretString>,
}

impl KeycloakEndpoints {
    pub fn from_config(config: &AppSettings) -> Self {
        let admin_enpoint = format!(
            "{}/admin/realms/{}",
            config.keycloak_base_url, config.keycloak_realm
        );
        let token_endpoint = format!(
            "{}/realms/{}/protocol/openid-connect/token",
            config.keycloak_base_url, config.keycloak_realm
        );
        let users_endpoint = format!("{admin_enpoint}/users");
        Self {
            admin_enpoint,
            token_endpoint,
            users_endpoint,
            client_id: config.keycloak_client_id.clone(),
            client_secret: config.keycloak_client_secret.clone(),
        }
    }
}

pub struct KeycloakUserStore {
    pub client: reqwest::Client,
    pub endpoints: KeycloakEndpoints,
}

impl KeycloakUserStore {
    pub fn new(client: reqwest::Client, endpoints: KeycloakEndpoints) -> Self {
        Self { client, endpoints }
    }
}

#[async_trait::async_trait]
impl AuthProvider for KeycloakUserStore {
    #[tracing::instrument(skip_all)]
    async fn retrieve_auth_token(&self) -> AppResult<String> {
        let form = [
            ("grant_type", "client_credentials"),
            ("client_id", &self.endpoints.client_id),
            (
                "client_secret",
                self.endpoints
                    .client_secret
                    .as_ref()
                    .ok_or(AuthProviderError::Upstream(
                        "Client secret not set for auth provider".to_string(),
                    ))?
                    .expose_secret(),
            ),
        ];

        let response = self
            .client
            .post(&self.endpoints.token_endpoint)
            .form(&form)
            .send()
            .await
            .map_err(|_| AuthProviderError::Upstream(
                "Failed to send request to Keycloak".to_string(),
            ))?;

        if !response.status().is_success() {
            return Err(AuthProviderError::Upstream(format!(
                "Failed to get admin token from Keycloak: {}",
                response.status()
            )))?;
        }

        let token: serde_json::Value = response.json().await.map_err(|e| {
            AuthProviderError::Upstream(format!("Failed to parse Keycloak response: {e}"))
        })?;
        let access_token = token.get("access_token").and_then(|t| t.as_str()).ok_or(
            AuthProviderError::Upstream("Missing access token in Keycloak response".to_string()),
        )?;
        Ok(access_token.to_string())
    }

    #[tracing::instrument(skip_all)]
    async fn signup_user(&self, user: User) -> AppResult<()> {
        let response = self
            .client
            .post(&self.endpoints.users_endpoint)
            .bearer_auth(&self.retrieve_auth_token().await?)
            .json(&user.signup_json(true, true))
            .send()
            .await
            .map_err(|e| {
                AuthProviderError::Upstream(format!("Failed to send request to Keycloak: {e}"))
            })?;

        match response.status() {
            StatusCode::CREATED => Ok(()),
            StatusCode::CONFLICT => Err(AuthProviderError::UserExists)?,
            status => Err(AuthProviderError::Network(format!(
                "Failed to create user in Keycloak: {status}"
            )))?,
        }
    }

    #[tracing::instrument(skip_all)]
    async fn login_user(&self, _email: Email, _password: Password) -> AppResult<User> {
        // Implementation to log in a user via Keycloak
        unimplemented!()
    }

    #[tracing::instrument(skip_all)]
    async fn logout_user(&self, _user_id: String) -> AppResult<()> {
        // Implementation to log out a user via Keycloak
        unimplemented!()
    }

    #[tracing::instrument(skip_all)]
    async fn delete_user(&self, user_id: String) -> AppResult<()> {
        let url = format!("{}/{}", &self.endpoints.users_endpoint, user_id);
        let response = self
            .client
            .delete(&url)
            .bearer_auth(&self.retrieve_auth_token().await?)
            .send()
            .await
            .map_err(|e| {
                AuthProviderError::Upstream(format!("Failed to send request to Keycloak: {e}"))
            })?;

        match response.status() {
            StatusCode::NO_CONTENT => Ok(()),
            StatusCode::NOT_FOUND => Err(AuthProviderError::UserNotFound)?,
            status => Err(AuthProviderError::Network(format!(
                "Failed to delete user in Keycloak: {status}"
            )))?,
        }
    }

    #[tracing::instrument(skip_all)]
    async fn get_user_id(&self, email: Email) -> AppResult<Option<String>> {
        let response = self
            .client
            .get(&self.endpoints.users_endpoint)
            .bearer_auth(&self.retrieve_auth_token().await?)
            .query(&[("email", email.as_ref().expose_secret())])
            .send()
            .await
            .map_err(|e| {
                AuthProviderError::Network(format!("Failed to send request to Keycloak: {e}"))
            })?;

        if !response.status().is_success() {
            return Err(AuthProviderError::Upstream(format!(
                "Failed to get user from Keycloak: {}",
                response.status()
            )))?;
        }

        let users: Vec<serde_json::Value> = response.json().await.map_err(|e| {
            AuthProviderError::Network(format!("Failed to parse Keycloak response: {e}"))
        })?;

        if let Some(user) = users.first() {
            if let Some(id) = user.get("id").and_then(|id| id.as_str()) {
                return Ok(Some(id.to_string()));
            }
        }

        Ok(None)
    }

    #[tracing::instrument(skip_all)]
    async fn update_user(&self, _user_update: UserUpdate) -> AppResult<()> {
        // Implementation to update a user's details in Keycloak
        unimplemented!()
    }
}
