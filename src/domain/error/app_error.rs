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
    #[error("User already exists")]
    UserExists,
    #[error("User not found")]
    UserNotFound,
    #[error("Network error: {0}")]
    Network(String),
}

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Postgres error: {0}")]
    Postgres(String),
}

#[derive(Debug, Error)]
pub enum EmailClientError {
    #[error("SMTP error: {0}")]
    Smtp(String),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    Validation(#[from] ValidationError),
    #[error(transparent)]
    AuthProvider(#[from] AuthProviderError),
    #[error(transparent)]
    Database(#[from] DatabaseError),
    #[error(transparent)]
    EmailClient(#[from] EmailClientError),
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

// Fixed type definition
pub type AppResult<T> = Result<T, AppError>;
