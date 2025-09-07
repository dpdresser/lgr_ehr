use poem::{EndpointExt, Route, Server, listener, middleware::Tracing, web::Data};
use poem_openapi::{OpenApi, OpenApiService, payload::PlainText};
use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;

use crate::utils::config::AppSettings;

pub mod utils;

#[derive(Debug)]
struct EHRApi;

#[OpenApi]
impl EHRApi {
    #[oai(path = "/health", method = "get")]
    #[tracing::instrument]
    async fn health_check(&self, Data(db): Data<&sqlx::PgPool>) -> PlainText<&'static str> {
        tracing::info!("Health check requested");
        if sqlx::query("SELECT 1").execute(db).await.is_ok() {
            PlainText("EHR API is running")
        } else {
            PlainText("EHR API is not running")
        }
    }
}

pub struct EHRApp {
    config: AppSettings,
}

impl EHRApp {
    pub fn build(
            config: AppSettings,
    ) -> Self {
        EHRApp {
            config,
        }
    }

    pub async fn run(&self) -> Result<(), std::io::Error> {
        tracing::info!("Starting EHR API server on {}", self.config.app_address());

        let db = PgPoolOptions::new()
            .connect(self.config.database_url().expose_secret())
            .await
            .expect("Failed to connect to the database");

        sqlx::migrate!()
            .run(&db)
            .await
            .expect("Failed to run database migrations");

        let api_service = OpenApiService::new(EHRApi, "EHR API", "1.0")
            .server(format!("http://{}/api", self.config.app_address()));
        let ui = api_service.swagger_ui();
        let app = Route::new()
            .nest("/api", api_service)
            .nest("/docs", ui)
            .with(Tracing)
            .data(db.clone());

        let listener = listener::TcpListener::bind(self.config.app_address());
        Server::new(listener).run(app).await
    }
}
