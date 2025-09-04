use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct AppSettings {
    pub app_host: String,
    pub app_port: u16,
    pub database_url: String,
    pub rust_log: String,
}

impl AppSettings {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        // TODO: hide sensitive DB information

        let app_host = std::env::var("APP_HOST").unwrap_or_else(|_| "0.0.0.0".into());
        let app_port = std::env::var("APP_PORT")
            .unwrap_or_else(|_| "3000".into())
            .parse()
            .unwrap_or(3000);
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@db:5432/ehr".into());
        let rust_log = std::env::var("RUST_LOG").unwrap_or_else(|_| "info,sqlx=warn".into());

        Self {
            app_host,
            app_port,
            database_url,
            rust_log,
        }
    }
}
