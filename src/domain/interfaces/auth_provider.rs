use crate::domain::{
    error::app_error::AppResult,
    types::{
        email::Email,
        password::Password,
        user::{User, UserUpdate},
    },
};

#[async_trait::async_trait]
pub trait AuthProvider {
    async fn retrieve_auth_token(&self) -> AppResult<String>;
    async fn signup_user(&self, user: User) -> AppResult<()>;
    async fn login_user(&self, email: Email, password: Password) -> AppResult<User>;
    async fn logout_user(&self, user_id: String) -> AppResult<()>;
    async fn delete_user(&self, user_id: String) -> AppResult<()>;
    async fn get_user_id(&self, email: Email) -> AppResult<Option<String>>;
    async fn update_user(&self, user_update: UserUpdate) -> AppResult<()>;
}
