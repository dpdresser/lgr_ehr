use lgr_ehr::EHRApp;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = EHRApp::default();
    app.run().await?;

    Ok(())
}
