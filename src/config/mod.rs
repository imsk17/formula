use config::{Config};
use eyre::Result;
use serde::Deserialize;
use eyre::WrapErr;

#[derive(Debug, Deserialize)]
pub struct Chain {
    pub rpc: String,
    pub name: String,
    pub chain_id: u32,
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub host: String,
    pub port: i32,
    pub db: String,
    pub chains: Vec<Chain>,
}

impl AppConfig {
    pub fn from_json5(filename: &str) -> Result<Self> {
        let config = Config::builder()
            .add_source(config::File::with_name(filename))
            .build()?;
        config.try_deserialize().context("Failed to parse config from config.json5")
    }
}