use secrecy::SecretString;

#[derive(Clone, Debug)]
pub struct AppSettings {
    pub app_host: String,
    pub app_port: u16,
    pub database_url: SecretString,
    pub log_level: String,
    pub keycloak_base_url: String,
    pub keycloak_realm: String,
    pub keycloak_client_id: String,
    pub keycloak_client_secret: Option<SecretString>,
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_from: String,
    pub tls_cert_path: String,
    pub tls_key_path: String,
}

impl AppSettings {
    // Load settings from environment variables
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        // App settings
        let app_host =
            std::env::var("APP_HOST").expect("APP_HOST must be set in .env or environment");

        let app_port = std::env::var("APP_PORT")
            .expect("APP_PORT must be set in .env or environment")
            .parse()
            .unwrap_or(3000);

        // Database settings
        let db_password =
            std::env::var("DB_PASSWORD").expect("DB_PASSWORD must be set in .env or environment");

        let db_host = std::env::var("DB_HOST").expect("DB_HOST must be set in .env or environment");
        let db_port = std::env::var("DB_PORT").expect("DB_PORT must be set in .env or environment");
        let db_name = std::env::var("DB_NAME").expect("DB_NAME must be set in .env or environment");
        let db_user = std::env::var("DB_USER").expect("DB_USER must be set in .env or environment");

        // Construct the database URL
        let database_url =
            format!("postgres://{db_user}:{db_password}@{db_host}:{db_port}/{db_name}");

        // Logging settings
        let log_level = std::env::var("LOG_LEVEL").unwrap_or_else(|_| "info".into());

        // Keycloak settings
        let keycloak_base_url = std::env::var("KEYCLOAK_BASE_URL_PROD")
            .expect("KEYCLOAK_BASE_URL must be set in .env or environment");
        let keycloak_realm = std::env::var("KEYCLOAK_REALM")
            .expect("KEYCLOAK_REALM must be set in .env or environment");
        let keycloak_client_id = std::env::var("KEYCLOAK_CLIENT_ID")
            .expect("KEYCLOAK_CLIENT_ID must be set in .env or environment");
        let keycloak_client_secret = std::env::var("KEYCLOAK_CLIENT_SECRET")
            .ok()
            .map(SecretString::from);

        // SMTP settings
        let smtp_host = std::env::var("SMTP_HOST_PROD")
            .expect("SMTP_HOST_PROD must be set in .env or environment");
        let smtp_port = std::env::var("SMTP_PORT")
            .unwrap_or_else(|_| "1025".into())
            .parse()
            .expect("SMTP_PORT must be a valid u16");
        let smtp_from = std::env::var("SMTP_FROM").unwrap_or_else(|_| "no-reply@lgrehr.dev".into());

        // TLS settings
        let tls_cert_path =
            std::env::var("TLS_CERT_PATH").unwrap_or_else(|_| "certs/dev/cert.pem".into());
        let tls_key_path =
            std::env::var("TLS_KEY_PATH").unwrap_or_else(|_| "certs/dev/key.pem".into());

        Self {
            app_host,
            app_port,
            database_url: SecretString::from(database_url),
            log_level,
            keycloak_base_url,
            keycloak_realm,
            keycloak_client_id,
            keycloak_client_secret,
            smtp_host,
            smtp_port,
            smtp_from,
            tls_cert_path,
            tls_key_path,
        }
    }

    // Builder for tests w/ specific DB URL
    pub fn for_tests(database_url: SecretString) -> Self {
        dotenvy::dotenv().ok();

        let port = std::net::TcpListener::bind("127.0.0.1:0")
            .unwrap()
            .local_addr()
            .unwrap()
            .port();

        // Keycloak settings
        let keycloak_base_url = std::env::var("KEYCLOAK_BASE_URL_DEV")
            .expect("KEYCLOAK_BASE_URL must be set in .env or environment");
        let keycloak_realm = std::env::var("KEYCLOAK_REALM")
            .expect("KEYCLOAK_REALM must be set in .env or environment");
        let keycloak_client_id = std::env::var("KEYCLOAK_CLIENT_ID")
            .expect("KEYCLOAK_CLIENT_ID must be set in .env or environment");
        let keycloak_client_secret = std::env::var("KEYCLOAK_CLIENT_SECRET")
            .ok()
            .map(SecretString::from);

        // SMTP settings
        let smtp_host = std::env::var("SMTP_HOST_DEV")
            .expect("SMTP_HOST_DEV must be set in .env or environment");
        let smtp_port = std::env::var("SMTP_PORT")
            .unwrap_or_else(|_| "1025".into())
            .parse()
            .expect("SMTP_PORT must be a valid u16");
        let smtp_from = std::env::var("SMTP_FROM").unwrap_or_else(|_| "no-reply@lgrehr.dev".into());

        // TLS settings
        let tls_cert_path =
            std::env::var("TLS_CERT_PATH").unwrap_or_else(|_| "certs/dev/cert.pem".into());
        let tls_key_path =
            std::env::var("TLS_KEY_PATH").unwrap_or_else(|_| "certs/dev/key.pem".into());

        Self {
            app_host: "127.0.0.1".into(),
            app_port: port,
            database_url,
            log_level: "info".into(),
            keycloak_base_url,
            keycloak_realm,
            keycloak_client_id,
            keycloak_client_secret,
            smtp_host,
            smtp_port,
            smtp_from,
            tls_cert_path,
            tls_key_path,
        }
    }

    pub fn app_address(&self) -> String {
        format!("{}:{}", self.app_host, self.app_port)
    }

    // Helper function to build database URL for any database name
    fn build_database_url(db_name: &str) -> SecretString {
        dotenvy::dotenv().ok();

        let db_user = std::env::var("DB_USER").expect("DB_USER must be set in .env or environment");
        let db_password =
            std::env::var("DB_PASSWORD").expect("DB_PASSWORD must be set in .env or environment");
        let db_host = "localhost";
        let db_port = std::env::var("DB_PORT").expect("DB_PORT must be set in .env or environment");

        let db_url = format!("postgres://{db_user}:{db_password}@{db_host}:{db_port}/{db_name}");

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
