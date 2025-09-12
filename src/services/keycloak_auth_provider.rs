use reqwest::StatusCode;
use secrecy::{ExposeSecret, SecretString};

use crate::{
    domain::{
        interfaces::auth_provider::{AuthProvider, AuthProviderError},
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
    async fn retrieve_auth_token(&self) -> Result<String, AuthProviderError> {
        let form = [
            ("grant_type", "client_credentials"),
            ("client_id", &self.endpoints.client_id),
            (
                "client_secret",
                self.endpoints
                    .client_secret
                    .as_ref()
                    .ok_or(AuthProviderError::AuthProviderError(
                        "Client secret not set".to_string(),
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
            .map_err(|e| {
                AuthProviderError::AuthProviderError(format!(
                    "Failed to send request to Keycloak: {e}"
                ))
            })?;

        if !response.status().is_success() {
            return Err(AuthProviderError::AuthProviderError(format!(
                "Failed to get admin token from Keycloak: {}",
                response.status()
            )));
        }

        let token: serde_json::Value = response.json().await.map_err(|e| {
            AuthProviderError::AuthProviderError(format!("Failed to parse Keycloak response: {e}"))
        })?;
        let access_token = token.get("access_token").and_then(|t| t.as_str()).ok_or(
            AuthProviderError::AuthProviderError(
                "Missing access token in Keycloak response".to_string(),
            ),
        )?;
        Ok(access_token.to_string())
    }

    #[tracing::instrument(skip_all)]
    async fn signup_user(&self, user: User) -> Result<(), AuthProviderError> {
        let response = self
            .client
            .post(&self.endpoints.users_endpoint)
            .bearer_auth(&self.retrieve_auth_token().await?)
            .json(&user.signup_json(true, true))
            .send()
            .await
            .map_err(|e| {
                AuthProviderError::AuthProviderError(format!(
                    "Failed to send request to Keycloak: {e}"
                ))
            })?;

        match response.status() {
            StatusCode::CREATED => Ok(()),
            StatusCode::CONFLICT => Err(AuthProviderError::DuplicateEmail),
            status => Err(AuthProviderError::AuthProviderError(format!(
                "Failed to create user in Keycloak: {status}"
            ))),
        }
    }

    #[tracing::instrument(skip_all)]
    async fn login_user(
        &self,
        _email: Email,
        _password: Password,
    ) -> Result<User, AuthProviderError> {
        // Implementation to log in a user via Keycloak
        unimplemented!()
    }

    #[tracing::instrument(skip_all)]
    async fn logout_user(&self, _user_id: String) -> Result<(), AuthProviderError> {
        // Implementation to log out a user via Keycloak
        unimplemented!()
    }

    #[tracing::instrument(skip_all)]
    async fn delete_user(&self, _user_id: String) -> Result<(), AuthProviderError> {
        // Implementation to delete a user in Keycloak
        unimplemented!()
    }

    #[tracing::instrument(skip_all)]
    async fn get_user_id(&self, _email: Email) -> Result<Option<String>, AuthProviderError> {
        // Implementation to get a user ID by email from Keycloak
        unimplemented!()
    }

    #[tracing::instrument(skip_all)]
    async fn update_user(&self, _user_update: UserUpdate) -> Result<(), AuthProviderError> {
        // Implementation to update a user's details in Keycloak
        unimplemented!()
    }
}
