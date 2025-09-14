use lgr_ehr::{
    EHRApp,
    utils::{config::AppSettings, tracing::init_tracing},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = AppSettings::from_env();
    init_tracing(&config.log_level);

    let app = EHRApp::build(config).await;
    app.run().await?;

    Ok(())
}
