use lgr_ehr::{EHRApp, utils::tracing::init_tracing};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_tracing();

    let app = EHRApp::build("127.0.0.1:3000".into());
    app.run().await?;

    Ok(())
}
