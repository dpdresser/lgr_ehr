use lettre::{Message, SmtpTransport, Transport, message::header::ContentType};
use secrecy::ExposeSecret;

use crate::domain::{
    error::app_error::{AppResult, EmailClientError},
    interfaces::email_client::EmailClient,
    types::{email::Email, email_content::EmailContent},
};

pub struct LettreMailhogEmailClient {
    mailer: SmtpTransport,
    from: String,
}

impl LettreMailhogEmailClient {
    pub fn new(host: &str, port: u16, from: String) -> Self {
        let mailer = SmtpTransport::builder_dangerous(host).port(port).build();

        Self { mailer, from }
    }
}

#[async_trait::async_trait]
impl EmailClient for LettreMailhogEmailClient {
    async fn send_email(&self, to: &Email, content: &EmailContent) -> AppResult<()> {
        let email = Message::builder()
            .from(self.from.parse().map_err(|_| {
                EmailClientError::Other(anyhow::anyhow!("Could not parse to MailBox"))
            })?)
            .to(to.as_ref().expose_secret().parse().map_err(|_| {
                EmailClientError::Other(anyhow::anyhow!("Could not parse to MailBox"))
            })?)
            .subject(&content.subject)
            .header(ContentType::TEXT_HTML)
            .body(content.html.clone())
            .map_err(|e| EmailClientError::Other(e.into()))?;

        self.mailer
            .send(&email)
            .map_err(|e| EmailClientError::Smtp(e.to_string()))?;

        Ok(())
    }
}
