use lgr_ehr::{utils::config::AppSettings, EHRApp};
use secrecy::ExposeSecret;
use sqlx::{postgres::PgPoolOptions, Executor, PgPool};

pub struct TestApp {
    address: String,
    http_client: reqwest::Client,
    _db_pool: PgPool,
    db_name: String,
    cleanup_called: bool,
}

impl TestApp {
    pub async fn new() -> Self {
        // Create test database first
        let (db_pool, db_name) = create_test_database().await;

        // Create database URL for the test database
        let test_db_url = AppSettings::database_url_for(&db_name);

        // Create settings with test database
        let settings = AppSettings::for_tests(test_db_url);
        let app = EHRApp::build(settings.clone());

        // Spawn the server
        tokio::spawn(async move {
            app.run().await.expect("Failed to start test server")
        });
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        let address = format!("http://{}", settings.app_address());
        let http_client = reqwest::Client::new();

        Self {
            address,
            http_client,
            _db_pool: db_pool,
            db_name,
            cleanup_called: false,
        }
    }

    pub async fn health_check(&self) -> reqwest::Response {
        self.http_client
            .get(format!("{}/api/health", &self.address))
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn cleanup(&mut self) {
        if !self.cleanup_called {
            cleanup_test_database(&self.db_name).await;
            self.cleanup_called = true;
        }
    }
}

// Panic if TestApp is dropped without cleanup
impl Drop for TestApp {
    fn drop(&mut self) {
        if !self.cleanup_called {
            panic!("TestApp was dropped without calling cleanup(). Database {} may not be deleted.", self.db_name);
       }
    }
}

async fn create_test_database() -> (PgPool, String) {
    // Generate unique database name
    let db_name = format!("test_{}", uuid::Uuid::new_v4().simple());

    // Get admin database URL from environment
    let admin_db_url = AppSettings::admin_database_url();

    // Connect to postgres admin database
    let admin_connection = PgPoolOptions::new()
        .connect(&admin_db_url.expose_secret())
        .await
        .expect("Failed to connect to PostgreSQL admin database");

    // Create the test database
    admin_connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, db_name).as_str())
        .await
        .expect("Failed to create test database");

    // Connect to the new test database
    let test_db_url = AppSettings::database_url_for(&db_name);

    let test_connection = PgPoolOptions::new()
        .connect(&test_db_url.expose_secret())
        .await
        .expect("Failed to connect to test database");

    // Run migrations on the test database
    sqlx::migrate!("./migrations")
        .run(&test_connection)
        .await
        .expect("Failed to run migrations on test database");

    (test_connection, db_name)
}

async fn cleanup_test_database(db_name: &str) {
    let admin_db_url = AppSettings::admin_database_url();

    if let Ok(admin_connection) = PgPoolOptions::new().connect(&admin_db_url.expose_secret()).await {
        // Terminate connections to test database
        let _ = admin_connection
            .execute(
                format!(
                    r#"
                    SELECT pg_terminate_backend(pg_stat_activity.pid)
                    FROM pg_stat_activity
                    WHERE pg_stat_activity.datname = '{}'
                    AND pid <> pg_backend_pid();"#,
                    db_name
                )
                .as_str(),
            )
            .await;

        // Drop the test database
        let _ = admin_connection
            .execute(format!(r#"DROP DATABASE IF EXISTS "{}";"#, db_name).as_str())
            .await;
    }
}
