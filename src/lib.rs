use poem::{Route, Server, listener::TcpListener};
use poem_openapi::{OpenApi, OpenApiService, payload::PlainText};

struct EHRApi;

#[OpenApi]
impl EHRApi {
    #[oai(path = "/health", method = "get")]
    async fn health_check(&self) -> PlainText<&'static str> {
        PlainText("EHR API is running")
    }
}

pub struct EHRApp;

impl EHRApp {
    fn new() -> Self {
        EHRApp {}
    }

    pub async fn run(&self) -> Result<(), std::io::Error> {
        let api_service =
            OpenApiService::new(EHRApi, "EHR API", "1.0").server("http://localhost:3000/api");
        let ui = api_service.swagger_ui();
        let app = Route::new().nest("/api", api_service).nest("/docs", ui);

        Server::new(TcpListener::bind("127.0.0.1:3000"))
            .run(app)
            .await?;

        Ok(())
    }
}

impl Default for EHRApp {
    fn default() -> Self {
        Self::new()
    }
}
