use poem::web::Data;
use poem_openapi::{Object, payload::Json};
use serde_json::Value;

use crate::{
    domain::{error::app_error::AppError, types::email::Email},
    state::AppState,
};

#[derive(Object, Debug)]
pub struct GetUserIdRequest {
    pub email: String,
}

pub async fn get_user_id_impl(
    state: Data<&AppState>,
    payload: Json<GetUserIdRequest>,
) -> Result<Value, AppError> {
    tracing::info!("Received get_user_id request for email: {}", payload.email);

    let email = Email::new(payload.email.clone())?;

    match state.auth_provider.read().await.get_user_id(email).await? {
        Some(user_id) => Ok(serde_json::json!({
            "user_id": user_id
        })),
        None => Err(AppError::AuthProvider(
            crate::domain::error::app_error::AuthProviderError::UserNotFound,
        )),
    }
}
