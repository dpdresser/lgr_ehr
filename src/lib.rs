use poem::{EndpointExt, Route, Server, listener, middleware::Tracing};
use poem_openapi::{OpenApi, OpenApiService, payload::PlainText};

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

        let api_service = OpenApiService::new(EHRApi, "EHR API", "1.0")
            .server(format!("http://{}/api", self.app_address));
        let ui = api_service.swagger_ui();
        let app = Route::new()
            .nest("/api", api_service)
            .nest("/docs", ui)
            .with(Tracing);

        let listener = listener::TcpListener::bind(&self.app_address);
        Server::new(listener).run(app).await
    }
}
