use config::Config;

use crate::listener::PgPool;
use diesel::r2d2;
use diesel::{r2d2::ConnectionManager, PgConnection};
use error::AppConfigError;
use error_stack::{Report, Result, ResultExt};
use serde::Deserialize;
use tracing::{info, instrument};
use tracing_subscriber::EnvFilter;

mod error;

#[derive(Debug, Deserialize, Clone)]
pub struct Chain {
    pub rpc: String,
    pub name: String,
    pub chain_id: i64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub host: [u8; 4],
    pub port: u16,
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
            .map_err(Report::from)
            .change_context(AppConfigError::BuildConfigFromFile)?;

        config
            .try_deserialize::<AppConfig>()
            .map_err(Report::from)
            .change_context(AppConfigError::DeserializeConfigIntoStruct)
    }

    pub fn db_pool(&self) -> Result<PgPool, AppConfigError> {
        let cm = ConnectionManager::<PgConnection>::new(&self.db);

        r2d2::Pool::builder()
            .build(cm)
            .map_err(Report::from)
            .attach_printable_lazy(|| format!("Unable to connect to DB URI: {}", self.db))
            .change_context(AppConfigError::FailedToCreateDB)
    }

    pub fn setup_logging() -> () {
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .compact()
            .with_line_number(true)
            .with_thread_names(true)
            .with_thread_ids(true)
            .init()
    }
}
