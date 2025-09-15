use poem::web::Data;
use poem_openapi::{Object, payload::Json};
use serde_json::Value;

use crate::{
    domain::{
        error::app_error::AppResult,
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

pub async fn signup_impl(state: Data<&AppState>, payload: Json<SignupRequest>) -> AppResult<Value> {
    let email = Email::new(payload.email.clone())?;

    let password = Password::new(payload.password.clone())?;

    let user = User::new(
        payload.email.clone(),
        email,
        password,
        payload.first_name.clone(),
        payload.last_name.clone(),
        None,
    );

    state.auth_provider.write().await.signup_user(user).await?;

    Ok(serde_json::json!({
        "message": "User signed up successfully"
    }))
}
