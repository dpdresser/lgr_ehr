use poem::web::Data;
use poem_openapi::{ApiResponse, Object, payload::Json};

use crate::state::AppState;

#[derive(Object, Debug)]
pub struct DeleteUserRequest {
    pub user_id: String,
}

#[derive(Object, Debug)]
pub struct DeleteUserResponse {
    pub user_id: String,
}

#[derive(ApiResponse, Debug)]
pub enum DeleteUserResult {
    #[oai(status = 200)]
    Ok(Json<DeleteUserResponse>),
    #[oai(status = 404)]
    NotFound(Json<serde_json::Value>),
    #[oai(status = 400)]
    BadRequest(Json<serde_json::Value>),
    #[oai(status = 500)]
    ServerError(Json<serde_json::Value>),
}

#[tracing::instrument(skip_all)]
pub async fn delete_user_impl(
    state: Data<&AppState>,
    payload: Json<DeleteUserRequest>,
) -> DeleteUserResult {
    tracing::info!("Received delete_user request for user: {}", payload.user_id);

    match state
        .auth_provider
        .read()
        .await
        .delete_user(payload.user_id.clone())
        .await
    {
        Ok(()) => DeleteUserResult::Ok(Json(DeleteUserResponse {
            user_id: payload.user_id.clone(),
        })),
        Err(e) => {
            tracing::error!("Error retrieving user ID: {}", e);
            DeleteUserResult::ServerError(Json(serde_json::json!({
                "message": "Internal server error",
                "details": e.to_string()
            })))
        }
    }
}
