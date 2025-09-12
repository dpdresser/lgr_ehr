use std::sync::Arc;

use anyhow::{Result, anyhow};
use poem::{
    EndpointExt, Route, Server,
    http::Method,
    listener::{self, Listener, RustlsCertificate, RustlsConfig},
    middleware::{Cors, Tracing},
    web::Data,
};
use poem_openapi::{
    OpenApi, OpenApiService,
    payload::{Json, PlainText},
};
use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
use tokio::sync::RwLock;

use crate::{
    routes::{
        health::health_check_impl,
        signup::{SignupRequest, SignupResult, signup_impl},
    },
    services::keycloak_auth_provider::{KeycloakEndpoints, KeycloakUserStore},
    state::AppState,
    utils::config::AppSettings,
};

pub mod domain;
pub mod routes;
pub mod services;
pub mod state;
pub mod utils;

#[derive(Debug)]
struct EHRApi;

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
}

pub struct EHRApp {
    config: AppSettings,
    state: AppState,
}

impl EHRApp {
    pub async fn build(config: AppSettings) -> Self {
        let db = PgPoolOptions::new()
            .connect(config.database_url.expose_secret())
            .await
            .expect("Failed to connect to the database");

        let auth_provider = KeycloakUserStore::new(
            reqwest::Client::new(),
            KeycloakEndpoints::from_config(&config),
        );

        let state = AppState::new(
            Arc::new(RwLock::new(auth_provider)),
            Arc::new(RwLock::new(db)),
        );

        EHRApp { config, state }
    }

    pub async fn run(&self) -> Result<()> {
        tracing::info!("Starting EHR API server on {}", self.config.app_address());

        sqlx::migrate!()
            .run(&*self.state.db.write().await)
            .await
            .expect("Failed to run database migrations");

        // OpenAPI
        let api_service = OpenApiService::new(EHRApi, "EHR API", "1.0")
            .server(format!("http://{}/api", self.config.app_address()));
        let ui = api_service.swagger_ui();

        // CORS
        // TODO: make configurable, tighten security for production
        let cors = Cors::new()
            .allow_origin("https://localhost:3000")
            .allow_origin("https://127.0.0.1:3000")
            .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
            .allow_headers(vec!["Authorization", "Content-Type"])
            .expose_headers(vec!["Content-Length"])
            .max_age(3600);

        let app = Route::new()
            .nest("/api", api_service)
            .nest("/docs", ui)
            .with(cors)
            .with(Tracing)
            .data(self.state.clone());

        // TLS configuration
        let cert_data = std::fs::read(&self.config.tls_cert_path)
            .map_err(|e| anyhow!("Failed to read TLS certificate: {}", e))?;
        let key_data = std::fs::read(&self.config.tls_key_path)
            .map_err(|e| anyhow!("Failed to read TLS private key: {}", e))?;
        let cert = RustlsCertificate::new().key(key_data).cert(cert_data);
        let rustls_config = RustlsConfig::new().fallback(cert);

        let listener = listener::TcpListener::bind(self.config.app_address()).rustls(rustls_config);
        Server::new(listener)
            .run(app)
            .await
            .map_err(|e| anyhow!("Server error: {}", e))
    }
}
