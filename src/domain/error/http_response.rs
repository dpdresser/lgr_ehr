use poem_openapi::{ApiResponse, Object, payload::Json};
use serde::Serialize;
use serde_json::Value;

use crate::domain::error::app_error::{
    AppError, AuthProviderError, DatabaseError, ValidationError,
};

#[derive(Object, Serialize, Debug)]
pub struct ErrorBody {
    pub code: String,
    pub message: String,
    pub request_id: String,
}

#[derive(ApiResponse, Debug)]
pub enum AppHttpResponse {
    #[oai(status = 200)]
    Ok(Json<Value>),
    #[oai(status = 201)]
    Created(Json<Value>),
    #[oai(status = 400)]
    BadRequest(Json<ErrorBody>),
    #[oai(status = 401)]
    Unauthorized(Json<ErrorBody>),
    #[oai(status = 403)]
    Forbidden(Json<ErrorBody>),
    #[oai(status = 404)]
    NotFound(Json<ErrorBody>),
    #[oai(status = 409)]
    Conflict(Json<ErrorBody>),
    #[oai(status = 502)]
    BadGateway(Json<ErrorBody>),
    #[oai(status = 500)]
    InternalServerError(Json<ErrorBody>),
}

impl std::fmt::Display for AppHttpResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl AppHttpResponse {
    fn body(code: &str, message: &str, request_id: &str) -> Json<ErrorBody> {
        Json(ErrorBody {
            code: code.to_string(),
            message: message.to_string(),
            request_id: request_id.to_string(),
        })
    }

    pub fn from_app_error(error: AppError, request_id: &str) -> Self {
        match error {
            AppError::Validation(ValidationError::InvalidEmail) => AppHttpResponse::BadRequest(
                Self::body("InvalidEmail", "The provided email is invalid", request_id),
            ),
            AppError::Validation(ValidationError::InvalidPassword) => {
                AppHttpResponse::BadRequest(Self::body(
                    "InvalidPassword",
                    "Password must be at least 8 characters long with at least one number and one special character",
                    request_id,
                ))
            }
            AppError::Validation(ValidationError::InvalidInput(msg)) => {
                AppHttpResponse::BadRequest(Self::body("InvalidInput", &msg, request_id))
            }
            AppError::AuthProvider(AuthProviderError::Unauthorized) => {
                AppHttpResponse::Unauthorized(Self::body(
                    "Unauthorized",
                    "Authentication failed",
                    request_id,
                ))
            }
            AppError::AuthProvider(AuthProviderError::InvalidAdminCredentials) => {
                AppHttpResponse::Forbidden(Self::body(
                    "InvalidAdminCredentials",
                    "Admin credentials are invalid",
                    request_id,
                ))
            }
            AppError::AuthProvider(AuthProviderError::UserExists) => AppHttpResponse::Conflict(
                Self::body("UserExists", "The user already exists", request_id),
            ),
            AppError::AuthProvider(AuthProviderError::UserNotFound) => AppHttpResponse::NotFound(
                Self::body("UserNotFound", "The user was not found", request_id),
            ),
            AppError::AuthProvider(AuthProviderError::Upstream(msg)) => {
                AppHttpResponse::BadGateway(Self::body("UpstreamError", &msg, request_id))
            }
            AppError::AuthProvider(AuthProviderError::Network(msg)) => {
                AppHttpResponse::BadGateway(Self::body("NetworkError", &msg, request_id))
            }
            AppError::Database(DatabaseError::Postgres(msg)) => {
                AppHttpResponse::InternalServerError(Self::body("DatabaseError", &msg, request_id))
            }
            AppError::Database(DatabaseError::Unknown(msg)) => {
                AppHttpResponse::InternalServerError(Self::body(
                    "UnknownDatabaseError",
                    &msg,
                    request_id,
                ))
            }
            AppError::Internal { source, .. } => AppHttpResponse::InternalServerError(Self::body(
                "InternalServerError",
                &source.to_string(),
                request_id,
            )),
        }
    }
}
