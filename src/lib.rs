use anyhow::{Result, anyhow};
use poem::{
    EndpointExt, Route, Server,
    http::Method,
    listener::{self, Listener, RustlsCertificate, RustlsConfig},
    middleware::{Cors, Tracing},
    web::Data,
};
use poem_openapi::{OpenApi, OpenApiService, payload::PlainText};
use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;

use crate::{
    routes::{
        auth::{KeycloakState, SignupApi},
        health::health_check_impl,
    },
    utils::config::AppSettings,
};

pub mod routes;
pub mod utils;

#[derive(Debug)]
struct EHRApi;

#[OpenApi]
impl EHRApi {
    #[oai(path = "/health", method = "get")]
    #[tracing::instrument]
    async fn health_check(&self, Data(db): Data<&sqlx::PgPool>) -> PlainText<&'static str> {
        PlainText(health_check_impl(db).await)
    }
}

pub struct EHRApp {
    config: AppSettings,
}

impl EHRApp {
    pub fn build(config: AppSettings) -> Self {
        EHRApp { config }
    }

    pub async fn run(&self) -> Result<()> {
        tracing::info!("Starting EHR API server on {}", self.config.app_address());
        let keycloak_state = KeycloakState::from_config(&self.config)?;

        let db = PgPoolOptions::new()
            .connect(self.config.database_url().expose_secret())
            .await
            .expect("Failed to connect to the database");

        sqlx::migrate!()
            .run(&db)
            .await
            .expect("Failed to run database migrations");

        // OpenAPI
        let apis = (EHRApi, SignupApi);

        let api_service = OpenApiService::new(apis, "EHR API", "1.0")
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
            .data(db.clone())
            .data(keycloak_state);

        // TLS configuration
        let cert_data = std::fs::read(self.config.tls_cert_path())
            .map_err(|e| anyhow!("Failed to read TLS certificate: {}", e))?;
        let key_data = std::fs::read(self.config.tls_key_path())
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
