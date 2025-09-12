use poem::web::Data;
use poem_openapi::{
    OpenApi,
    payload::{Json, PlainText},
};

use crate::{
    routes::{
        delete_user::{DeleteUserRequest, DeleteUserResult, delete_user_impl},
        get_user_id::{GetUserIdRequest, GetUserIdResult, get_user_id_impl},
        health::health_check_impl,
        signup::{SignupRequest, SignupResult, signup_impl},
    },
    state::AppState,
};

#[derive(Debug)]
pub struct EHRApi;

#[OpenApi]
impl EHRApi {
    #[oai(path = "/health", method = "get")]
    async fn health_check(&self, state: Data<&AppState>) -> PlainText<&'static str> {
        health_check_impl(state).await
    }

    #[oai(path = "/auth/signup", method = "post")]
    async fn signup(&self, state: Data<&AppState>, payload: Json<SignupRequest>) -> SignupResult {
        signup_impl(state, payload).await
    }

    #[oai(path = "/auth/get_user_id", method = "post")]
    async fn get_user_id(
        &self,
        state: Data<&AppState>,
        payload: Json<GetUserIdRequest>,
    ) -> GetUserIdResult {
        get_user_id_impl(state, payload).await
    }

    #[oai(path = "/auth/delete_user", method = "post")]
    async fn delete_user(
        &self,
        state: Data<&AppState>,
        payload: Json<DeleteUserRequest>,
    ) -> DeleteUserResult {
        delete_user_impl(state, payload).await
    }
}
