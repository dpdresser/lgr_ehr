use poem::web::Data;
use poem_openapi::{
    OpenApi,
    payload::{Json, PlainText},
};

use crate::{
    domain::error::http_response::AppHttpResponse,
    routes::{
        delete_user::{DeleteUserRequest, delete_user_impl},
        get_user_id::{GetUserIdRequest, get_user_id_impl},
        health::health_check_impl,
        signup::{SignupRequest, signup_impl},
    },
    state::AppState,
    utils::tracing::RequestContext,
};

#[derive(Debug)]
pub struct EHRApi;

#[OpenApi]
impl EHRApi {
    #[oai(path = "/health", method = "get")]
    #[tracing::instrument(name = "health_check", skip_all, err)]
    async fn health_check(
        &self,
        ctx: RequestContext,
        state: Data<&AppState>,
    ) -> Result<PlainText<&str>, AppHttpResponse> {
        health_check_impl(state)
            .await
            .map_err(|e| AppHttpResponse::from_app_error(e, &ctx.request_id))
    }

    #[oai(path = "/auth/signup", method = "post")]
    #[tracing::instrument(name = "signup", skip_all)]
    async fn signup(
        &self,
        ctx: RequestContext,
        state: Data<&AppState>,
        payload: Json<SignupRequest>,
    ) -> AppHttpResponse {
        match signup_impl(state, payload).await {
            Ok(response) => AppHttpResponse::Created(Json(response)),
            Err(e) => AppHttpResponse::from_app_error(e, &ctx.request_id),
        }
    }

    #[oai(path = "/auth/get_user_id", method = "post")]
    #[tracing::instrument(name = "get_user_id", skip_all)]
    async fn get_user_id(
        &self,
        ctx: RequestContext,
        state: Data<&AppState>,
        payload: Json<GetUserIdRequest>,
    ) -> AppHttpResponse {
        match get_user_id_impl(state, payload).await {
            Ok(response) => AppHttpResponse::Ok(Json(response)),
            Err(e) => AppHttpResponse::from_app_error(e, &ctx.request_id),
        }
    }

    #[oai(path = "/auth/delete_user", method = "post")]
    #[tracing::instrument(name = "delete_user", skip_all)]
    async fn delete_user(
        &self,
        ctx: RequestContext,
        state: Data<&AppState>,
        payload: Json<DeleteUserRequest>,
    ) -> AppHttpResponse {
        match delete_user_impl(state, payload).await {
            Ok(response) => AppHttpResponse::Ok(Json(response)),
            Err(e) => AppHttpResponse::from_app_error(e, &ctx.request_id),
        }
    }
}
