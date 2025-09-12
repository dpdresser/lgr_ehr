use poem::web::Data;
use poem_openapi::{ApiResponse, Object, payload::Json};

use crate::{domain::types::email::Email, state::AppState};

#[derive(Object, Debug)]
pub struct GetUserIdRequest {
    pub email: String,
}

#[derive(Object, Debug)]
pub struct GetUserIdResponse {
    pub user_id: String,
}

#[derive(ApiResponse, Debug)]
pub enum GetUserIdResult {
    #[oai(status = 200)]
    Ok(Json<GetUserIdResponse>),
    #[oai(status = 404)]
    NotFound(Json<serde_json::Value>),
    #[oai(status = 400)]
    BadRequest(Json<serde_json::Value>),
    #[oai(status = 500)]
    ServerError(Json<serde_json::Value>),
}

#[tracing::instrument(skip_all)]
pub async fn get_user_id_impl(
    state: Data<&AppState>,
    payload: Json<GetUserIdRequest>,
) -> GetUserIdResult {
    tracing::info!("Received get_user_id request for email: {}", payload.email);
    let email = match Email::new(payload.email.clone()) {
        Ok(email) => email,
        Err(e) => {
            tracing::warn!("Invalid email format: {}", e);
            return GetUserIdResult::BadRequest(Json(serde_json::json!({
                "message": "Invalid email format",
                "details": e.to_string()
            })));
        }
    };

    match state.auth_provider.read().await.get_user_id(email).await {
        Ok(Some(user_id)) => GetUserIdResult::Ok(Json(GetUserIdResponse { user_id })),
        Ok(None) => GetUserIdResult::NotFound(Json(serde_json::json!({
            "message": "User not found"
        }))),
        Err(e) => {
            tracing::error!("Error retrieving user ID: {}", e);
            GetUserIdResult::ServerError(Json(serde_json::json!({
                "message": "Internal server error",
                "details": e.to_string()
            })))
        }
    }
}
