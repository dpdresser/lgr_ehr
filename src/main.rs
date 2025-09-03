use lgr_ehr::EHRApp;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = EHRApp::build("127.0.0.1:3000".into());
    app.run().await?;

    Ok(())
}
