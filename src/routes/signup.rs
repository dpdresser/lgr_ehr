use poem::web::Data;
use poem_openapi::{ApiResponse, Object, payload::Json};

use crate::{
    domain::{
        interfaces::auth_provider::AuthProviderError,
        types::{email::Email, password::Password, user::User},
    },
    state::AppState,
};

#[derive(Object, Debug)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Object, Debug)]
pub struct SignupResponse {
    pub user_id: String,
}

#[derive(ApiResponse, Debug)]
pub enum SignupResult {
    #[oai(status = 201)]
    Created(Json<SignupResponse>),
    #[oai(status = 400)]
    BadRequest(Json<serde_json::Value>),
    #[oai(status = 409)]
    Conflict(Json<serde_json::Value>),
    #[oai(status = 500)]
    ServerError(Json<serde_json::Value>),
}

#[tracing::instrument(skip_all)]
pub async fn signup_impl(state: Data<&AppState>, payload: Json<SignupRequest>) -> SignupResult {
    tracing::info!("Received signup request for email: {}", payload.email);

    let email = match Email::new(payload.email.clone()) {
        Ok(email) => email,
        Err(e) => {
            tracing::warn!("Invalid email format: {}", e);
            return SignupResult::BadRequest(Json(serde_json::json!({
                "message": "Invalid email format",
                "details": e.to_string()
            })));
        }
    };

    let password = match Password::new(payload.password.clone()) {
        Ok(password) => password,
        Err(e) => {
            tracing::warn!("Invalid password format: {}", e);
            return SignupResult::BadRequest(Json(serde_json::json!({
                "message": "Invalid password format",
                "details": e.to_string()
            })));
        }
    };

    let user = User::new(
        payload.email.clone(),
        email,
        password,
        payload.first_name.clone(),
        payload.last_name.clone(),
        None,
    );

    match state.auth_provider.write().await.signup_user(user).await {
        Ok(_) => {
            tracing::info!("User signed up successfully: {}", payload.email);
            SignupResult::Created(Json(SignupResponse {
                user_id: payload.email.clone(),
            }))
        }
        Err(AuthProviderError::DuplicateEmail) => {
            tracing::warn!("Signup failed - duplicate email: {}", payload.email);
            SignupResult::Conflict(Json(serde_json::json!({
                "message": "Email already in use"
            })))
        }
        Err(e) => {
            tracing::error!("Signup failed due to server error: {}", e);
            SignupResult::ServerError(Json(serde_json::json!({
                "message": "Internal server error",
                "details": e.to_string()
            })))
        }
    }
}
