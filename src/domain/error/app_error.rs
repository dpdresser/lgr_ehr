use thiserror::Error;
use tracing_error::SpanTrace;

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("Invalid email")]
    InvalidEmail,
    #[error("Invalid password")]
    InvalidPassword,
    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

#[derive(Debug, Error)]
pub enum AuthProviderError {
    #[error("Upstream auth provider error: {0}")]
    Upstream(String),
    #[error("Invalid admin credentials")]
    InvalidAdminCredentials,
    #[error("User already exists")]
    UserExists,
    #[error("User not found")]
    UserNotFound,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Network error: {0}")]
    Network(String),
}

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Postgres error: {0}")]
    Postgres(String),
    #[error("Unknown error: {0}")]
    Unknown(String),
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    Validation(#[from] ValidationError),
    #[error(transparent)]
    AuthProvider(#[from] AuthProviderError),
    #[error(transparent)]
    Database(#[from] DatabaseError),
    #[error("Internal server error")]
    Internal {
        #[source]
        source: anyhow::Error,
        span: SpanTrace,
    },
}

impl AppError {
    pub fn internal<E: Into<anyhow::Error>>(e: E) -> Self {
        Self::Internal {
            source: e.into(),
            span: SpanTrace::capture(),
        }
    }
}

pub type AppResult<T> = color_eyre::eyre::Result<T, AppError>;
