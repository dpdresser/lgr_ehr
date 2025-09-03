use lgr_ehr::{EHRApp, utils::tracing::init_tracing};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_tracing();

    let address = std::env::var("BIND_ADDRESS").unwrap_or_else(|_| "0.0.0.0:3000".into());

    let app = EHRApp::build(address);
    app.run().await?;

    Ok(())
}
