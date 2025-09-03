use poem::{Route, Server, listener};
use poem_openapi::{OpenApi, OpenApiService, payload::PlainText};

struct EHRApi;

#[OpenApi]
impl EHRApi {
    #[oai(path = "/health", method = "get")]
    async fn health_check(&self) -> PlainText<&'static str> {
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
        let api_service = OpenApiService::new(EHRApi, "EHR API", "1.0")
            .server(format!("http://{}/api", self.app_address));
        let ui = api_service.swagger_ui();
        let app = Route::new().nest("/api", api_service).nest("/docs", ui);

        let listener = listener::TcpListener::bind(&self.app_address);
        Server::new(listener).run(app).await
    }
}
