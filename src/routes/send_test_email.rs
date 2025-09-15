use poem::web::Data;
use poem_openapi::{Object, payload::Json};
use serde_json::Value;

use crate::{
    domain::{
        error::app_error::{AppError, AppResult, ValidationError},
        types::{email::Email, email_content::EmailContent},
    },
    state::AppState,
};

#[derive(Object, Debug)]
pub struct SendEmailRequest {
    pub to: String,
}

pub async fn send_test_email_impl(
    state: Data<&AppState>,
    payload: Json<SendEmailRequest>,
) -> AppResult<Value> {
    if payload.to.trim().is_empty() {
        return Err(AppError::Validation(ValidationError::InvalidInput(
            "Recipient email cannot be empty".to_string(),
        )));
    }

    // Parse the email string into an Email type
    let to_email = Email::new(payload.to.clone())
        .map_err(|_| AppError::Validation(ValidationError::InvalidEmail))?;

    let content = EmailContent {
        subject: "Test Email".to_string(),
        html: "<h1>This is a test email</h1>".to_string(),
        body: "This is a test email".to_string(),
    };

    state
        .email_client
        .read()
        .await
        .send_email(&to_email, &content)
        .await?;

    Ok(serde_json::json!({
        "to": payload.to,
        "message": "Email sent successfully"
    }))
}
