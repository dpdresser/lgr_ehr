use sqlx::PgPool;

pub async fn health_check_impl(db: &PgPool) -> &'static str {
    tracing::info!("Health check requested");
    if sqlx::query("SELECT 1").execute(db).await.is_ok() {
        "EHR API is running"
    } else {
        "EHR API is not running"
    }
}
