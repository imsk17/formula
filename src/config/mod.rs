use config::Config;
use eyre::Result;
use eyre::WrapErr;
use serde::Deserialize;
use tracing::{info, instrument};

#[derive(Debug, Deserialize, Clone)]
pub struct Chain {
    pub rpc: String,
    pub name: String,
    pub chain_id: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub host: String,
    pub port: i32,
    pub db: String,
    pub chains: Vec<Chain>,
}

impl AppConfig {
    #[instrument]
    pub fn from_json5(filename: &str) -> Result<Self> {
        info!("Trying to read {} for application config", filename);

        let config = Config::builder()
            .add_source(config::File::with_name(filename))
            .build()?;
        config
            .try_deserialize()
            .context("Failed to parse config from config.json5")
    }
}
