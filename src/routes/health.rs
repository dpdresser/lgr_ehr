use poem::web::Data;
use poem_openapi::payload::PlainText;

use crate::state::AppState;

#[tracing::instrument(skip_all)]
pub async fn health_check_impl(state: Data<&AppState>) -> PlainText<&'static str> {
    let message = if sqlx::query("SELECT 1")
        .execute(&*state.db.write().await)
        .await
        .is_ok()
    {
        "EHR API is running"
    } else {
        "EHR API is not running"
    };

    PlainText(message)
}
