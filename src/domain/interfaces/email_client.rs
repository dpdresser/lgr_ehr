use crate::domain::{
    error::app_error::AppResult,
    types::{email::Email, email_content::EmailContent},
};

#[async_trait::async_trait]
pub trait EmailClient {
    async fn send_email(&self, to: &Email, content: &EmailContent) -> AppResult<()>;
}
