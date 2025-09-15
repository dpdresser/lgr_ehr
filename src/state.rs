use std::sync::Arc;

use sqlx::PgPool;
use tokio::sync::RwLock;

use crate::domain::interfaces::{auth_provider::AuthProvider, email_client::EmailClient};

#[derive(Clone)]
pub struct AppState {
    pub auth_provider: Arc<RwLock<dyn AuthProvider + Send + Sync>>,
    pub db: Arc<RwLock<PgPool>>,
    pub email_client: Arc<RwLock<dyn EmailClient + Send + Sync>>,
}

impl AppState {
    pub fn new(
        auth_provider: Arc<RwLock<dyn AuthProvider + Send + Sync>>,
        db: Arc<RwLock<PgPool>>,
        email_client: Arc<RwLock<dyn EmailClient + Send + Sync>>,
    ) -> Self {
        Self {
            auth_provider,
            db,
            email_client,
        }
    }
}
