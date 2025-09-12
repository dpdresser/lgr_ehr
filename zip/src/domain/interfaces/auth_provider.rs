use crate::domain::types::{
    email::Email,
    password::Password,
    user::{User, UserUpdate},
};

#[async_trait::async_trait]
pub trait AuthProvider {
    async fn retrieve_auth_token(&self) -> Result<String, AuthProviderError>;
    async fn signup_user(&self, user: User) -> Result<(), AuthProviderError>;
    async fn login_user(&self, email: Email, password: Password)
    -> Result<User, AuthProviderError>;
    async fn logout_user(&self, user_id: String) -> Result<(), AuthProviderError>;
    async fn delete_user(&self, user_id: String) -> Result<(), AuthProviderError>;
    async fn get_user_id(&self, email: Email) -> Result<Option<String>, AuthProviderError>;
    async fn update_user(&self, user_update: UserUpdate) -> Result<(), AuthProviderError>;
}

#[derive(thiserror::Error, Debug)]
pub enum AuthProviderError {
    #[error("Email already in use")]
    DuplicateEmail,
    #[error("Invalid email format")]
    InvalidEmail,
    #[error("Password does not meet complexity requirements")]
    WeakPassword,
    #[error("Auth provider not available")]
    AuthProviderUnavailable,
    #[error("Auth provider error: {0}")]
    AuthProviderError(String),
}
