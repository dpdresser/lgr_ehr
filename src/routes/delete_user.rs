use poem::web::Data;
use poem_openapi::{Object, payload::Json};
use serde_json::Value;

use crate::{domain::error::app_error::AppError, state::AppState};

#[derive(Object, Debug)]
pub struct DeleteUserRequest {
    pub user_id: String,
}

pub async fn delete_user_impl(
    state: Data<&AppState>,
    payload: Json<DeleteUserRequest>,
) -> Result<Value, AppError> {
    tracing::info!("Received delete_user request for user: {}", payload.user_id);

    if payload.user_id.trim().is_empty() {
        return Err(AppError::Validation(
            crate::domain::error::app_error::ValidationError::InvalidInput(
                "User ID cannot be empty".to_string(),
            ),
        ));
    }

    state
        .auth_provider
        .read()
        .await
        .delete_user(payload.user_id.clone())
        .await?;

    Ok(serde_json::json!({
        "user_id": payload.user_id,
        "message": "User deleted successfully"
    }))
}
