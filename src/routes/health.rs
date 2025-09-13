use poem::web::Data;
use poem_openapi::payload::PlainText;

use crate::{
    domain::error::app_error::{AppError, DatabaseError},
    state::AppState,
};

pub async fn health_check_impl(
    state: Data<&AppState>,
) -> Result<PlainText<&'static str>, AppError> {
    sqlx::query("SELECT 1")
        .execute(&*state.db.write().await)
        .await
        .map_err(|_| DatabaseError::Postgres("Failed to execute health check query".into()))?;

    Ok(PlainText("EHR API is running"))
}
