use lgr_ehr::{
    EHRApp,
    utils::{config::AppSettings, tracing::init_tracing},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = AppSettings::from_env();
    init_tracing();

    let address = format!("{}:{}", config.app_host, config.app_port);

    let app = EHRApp::build(address);
    app.run().await?;

    Ok(())
}
