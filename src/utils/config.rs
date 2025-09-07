use secrecy::SecretString;

#[derive(Clone)]
pub struct AppSettings {
    app_host: String,
    app_port: u16,
    database_url: SecretString,
    log_level: String,
}

impl AppSettings {
    // Load settings from environment variables
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        // App settings
        let app_host = std::env::var("APP_HOST")
            .expect("APP_HOST must be set in .env or environment");

        let app_port = std::env::var("APP_PORT")
            .expect("APP_PORT must be set in .env or environment")
            .parse()
            .unwrap_or(3000);

        // Database settings
        let db_password = std::env::var("DB_PASSWORD")
            .expect("DB_PASSWORD must be set in .env or environment");

        let db_host = std::env::var("DB_HOST")
            .expect("DB_HOST must be set in .env or environment");
        let db_port = std::env::var("DB_PORT")
            .expect("DB_PORT must be set in .env or environment");
        let db_name = std::env::var("DB_NAME")
            .expect("DB_NAME must be set in .env or environment");
        let db_user = std::env::var("DB_USER")
            .expect("DB_USER must be set in .env or environment");

        // Construct the database URL
        let database_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            db_user, db_password, db_host, db_port, db_name
        );

        // Logging settings
        let log_level = std::env::var("LOG_LEVEL").unwrap_or_else(|_| "info".into());

        Self {
            app_host,
            app_port,
            database_url: SecretString::from(database_url),
            log_level,
        }
    }

    // Builder for tests w/ specific DB URL
    pub fn for_tests(database_url: SecretString) -> Self {
        let port = std::net::TcpListener::bind("127.0.0.1:0")
            .unwrap()
            .local_addr()
            .unwrap()
            .port();

        Self {
            app_host: "127.0.0.1".into(),
            app_port: port,
            database_url,
            log_level: "info".into(),
        }
    }

    // Helper methods to access settings
    pub fn database_url(&self) -> &SecretString {
        &self.database_url
    }

    // Returns the full application address in the format "host:port"
    pub fn app_address(&self) -> String {
        format!("{}:{}", self.app_host, self.app_port)
    }

    // Returns the logging level
    pub fn log_level(&self) -> &str {
        &self.log_level
    }

    // Helper function to build database URL for any database name
    fn build_database_url(db_name: &str) -> SecretString {
        dotenvy::dotenv().ok();
        
        let db_user = std::env::var("DB_USER")
            .expect("DB_USER must be set in .env or environment");
        let db_password = std::env::var("DB_PASSWORD")
            .expect("DB_PASSWORD must be set in .env or environment");
        let db_host = "localhost";
        let db_port = std::env::var("DB_PORT")
            .expect("DB_PORT must be set in .env or environment");
        
        let db_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            db_user, db_password, db_host, db_port, db_name
        );
        
        SecretString::from(db_url)
    }

    // Build admin database URL (connects to "postgres" database)
    pub fn admin_database_url() -> SecretString {
        Self::build_database_url("postgres")
    }

    // Build database URL for a specific database name
    pub fn database_url_for(db_name: &str) -> SecretString {
        Self::build_database_url(db_name)
    }
}
