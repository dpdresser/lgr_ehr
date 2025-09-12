use std::sync::Arc;

use sqlx::PgPool;
use tokio::sync::RwLock;

use crate::domain::interfaces::auth_provider::AuthProvider;

#[derive(Clone)]
pub struct AppState {
    pub auth_provider: Arc<RwLock<dyn AuthProvider + Send + Sync>>,
    pub db: Arc<RwLock<PgPool>>,
}

impl AppState {
    pub fn new(
        auth_provider: Arc<RwLock<dyn AuthProvider + Send + Sync>>,
        db: Arc<RwLock<PgPool>>,
    ) -> Self {
        Self { auth_provider, db }
    }
}
