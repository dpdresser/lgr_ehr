use poem::{EndpointExt, Route, Server, listener, middleware::Tracing, web::Data};
use poem_openapi::{OpenApi, OpenApiService, payload::PlainText};
use sqlx::postgres::PgPoolOptions;

use crate::utils::config::AppSettings;

pub mod utils;

#[derive(Debug)]
struct EHRApi;

#[OpenApi]
impl EHRApi {
    #[oai(path = "/health", method = "get")]
    #[tracing::instrument]
    async fn health_check(&self) -> PlainText<&'static str> {
        tracing::info!("Health check requested");
        PlainText("EHR API is running")
    }

    #[oai(path = "/ready", method = "get")]
    async fn ready(&self, Data(db): Data<&sqlx::PgPool>) -> PlainText<&'static str> {
        if sqlx::query("SELECT 1").execute(db).await.is_ok() {
            PlainText("EHR API is ready")
        } else {
            PlainText("EHR API is not ready")
        }
    }
}

pub struct EHRApp {
    app_address: String,
}

impl EHRApp {
    pub fn build(address: String) -> Self {
        EHRApp {
            app_address: address,
        }
    }

    pub async fn run(&self) -> Result<(), std::io::Error> {
        tracing::info!("Starting EHR API server on {}", self.app_address);

        let settings = AppSettings::from_env();

        let db = PgPoolOptions::new()
            .connect(&settings.database_url)
            .await
            .expect("Failed to connect to the database");

        sqlx::migrate!()
            .run(&db)
            .await
            .expect("Failed to run database migrations");

        let api_service = OpenApiService::new(EHRApi, "EHR API", "1.0")
            .server(format!("http://{}/api", self.app_address));
        let ui = api_service.swagger_ui();
        let app = Route::new()
            .nest("/api", api_service)
            .nest("/docs", ui)
            .with(Tracing)
            .data(db.clone());

        let listener = listener::TcpListener::bind(&self.app_address);
        Server::new(listener).run(app).await
    }
}
