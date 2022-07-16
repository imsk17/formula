use config::Config;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::PgConnection;
use eyre::Result;
use eyre::WrapErr;
use serde::Deserialize;
use tracing::{info, instrument};

use crate::listener::PgPool;

#[derive(Debug, Deserialize, Clone)]
pub struct Chain {
    pub rpc: String,
    pub name: String,
    pub chain_id: i64,
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
