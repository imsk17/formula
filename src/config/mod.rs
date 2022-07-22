use config::Config;

use error::AppConfigError;
use error_stack::{IntoReport, Result, ResultExt};
use serde::Deserialize;
use tracing::{info, instrument};

mod error;

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
    pub fn from_json5(filename: &str) -> Result<AppConfig, AppConfigError> {
        info!("Trying to read {} for application config", filename);

        let config = Config::builder()
            .add_source(config::File::with_name(filename))
            .build()
            .report()
            .change_context(AppConfigError::BuildConfigFromFile)?;

        config
            .try_deserialize::<AppConfig>()
            .report()
            .change_context(AppConfigError::DeserializeConfigIntoStruct)
    }
}
