use lgr_ehr::EHRApp;

pub struct TestApp {
    address: String,
    http_client: reqwest::Client,
}

impl TestApp {
    pub async fn new() -> Self {
        let port = std::net::TcpListener::bind("127.0.0.1:0")
            .unwrap()
            .local_addr()
            .unwrap()
            .port();

        let address = format!("127.0.0.1:{}", port);

        let app = EHRApp::build(address.clone());

        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(async move { app.run().await });
        tokio::time::sleep(std::time::Duration::from_millis(0)).await;

        let address = format!("http://{address}");
        let http_client = reqwest::Client::new();

        Self {
            address,
            http_client,
        }
    }

    pub async fn health_check(&self) -> reqwest::Response {
        self.http_client
            .get(format!("{}/api/health", &self.address))
            .send()
            .await
            .expect("Failed to execute request")
    }
}
