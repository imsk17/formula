use eyre::Result;
use tokio;
use tracing::debug;
use tracing_subscriber::filter::EnvFilter;

mod config;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().with_env_filter(EnvFilter::from_default_env())
        .init();
    let conf = config::AppConfig::from_json5("config")?;
    debug!("Config Read Successfully. {:?}", conf);
    Ok(())
}
