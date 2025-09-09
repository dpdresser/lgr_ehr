use poem::web::Data;
use poem_openapi::{ApiResponse, Object, OpenApi, payload::Json};
use validator::Validate;

use crate::routes::auth::KeycloakState;

// TODO: refactor to include types for Email and Password
#[derive(Object, Debug, Validate)]
pub struct SignupRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1, max = 100))]
    pub first_name: String,
    #[validate(length(min = 1, max = 100))]
    pub last_name: String,
    #[validate(length(min = 8, max = 128))]
    pub password: String,
}

#[derive(Object, Debug)]
pub struct SignupResponse {
    pub keycloak_id: String,
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

#[derive(Debug)]
pub struct SignupApi;

// TODO: better error handling and mapping to SignupResult
#[OpenApi]
impl SignupApi {
    #[tracing::instrument(skip_all)]
    #[oai(path = "/auth/signup", method = "post", operation_id = "auth_signup")]
    async fn signup_impl(
        &self,
        Data(keycloak_state): Data<&KeycloakState>,
        Json(body): Json<SignupRequest>,
    ) -> SignupResult {
        tracing::debug!("Signing up user with email: {}", body.email);

        if let Err(e) = body.validate() {
            tracing::warn!("Invalid signup request: {}", e);
            return SignupResult::BadRequest(Json(serde_json::json!({
                "message": "Invalid request",
                "details": e.to_string()
            })));
        }

        let admin_token = match keycloak_state.admin_token().await {
            Ok(token) => token,
            Err(e) => {
                tracing::error!("Failed to get admin token: {}", e);
                return SignupResult::ServerError(Json(serde_json::json!({
                    "message": "Failed to communicate with authentication server",
                    "details": e.to_string()
                })));
            }
        };

        let user_json = serde_json::json!({
            "username": body.email,
            "email": body.email,
            "firstName": body.first_name,
            "lastName": body.last_name,
            "enabled": true,
            "emailVerified": true,
            "credentials": [{
                "type": "password",
                "value": body.password,
                "temporary": false
            }]
        });

        // TODO: store client in app state
        let client = reqwest::Client::new();
        let resp = match client
            .post(keycloak_state.users_endpoint())
            .bearer_auth(&admin_token)
            .json(&user_json)
            .send()
            .await
        {
            Ok(r) => r,
            Err(e) => {
                tracing::error!(error=%e, "failed to call Keycloak /users");
                return SignupResult::ServerError(Json(
                    serde_json::json!({"error":"user_create_request_failed"}),
                ));
            }
        };

        match resp.status().as_u16() {
            201 => {
                let id = resp
                    .headers()
                    .get(reqwest::header::LOCATION)
                    .and_then(|h| h.to_str().ok())
                    .and_then(|loc| loc.rsplit('/').next())
                    .unwrap_or_default()
                    .to_string();

                // TODO: has to postgres

                SignupResult::Created(Json(SignupResponse { keycloak_id: id }))
            }
            409 => SignupResult::Conflict(Json(serde_json::json!({
                "error": "user_exists",
                "message": "An account with this email already exists."
            }))),
            code => {
                let text = resp.text().await.unwrap_or_default();
                tracing::error!(status=code, body=%text, "unexpected KC response");
                SignupResult::ServerError(Json(serde_json::json!({
                    "error":"user_create_failed",
                    "status": code,
                    "body": text
                })))
            }
        }
    }
}
