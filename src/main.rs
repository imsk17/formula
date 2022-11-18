#![allow(dead_code)]
use std::sync::Arc;

use crate::{
    config::AppConfig,
    listener::{Listenable, Listener},
};

use error_stack::{ResultExt, Result};
use errors::FormulaErrors;
use tokio;
use tracing::debug;
#[macro_use]
extern crate diesel;

mod config;
mod contracts;
mod erc165;
mod ethdto;
mod events;
mod listener;
mod schema;
mod uri_getter;
mod errors;

#[tokio::main]
async fn main() -> Result<(), FormulaErrors> {
    AppConfig::setup_logging();

    let config = Arc::new(AppConfig::from_json5("config").change_context(FormulaErrors::ConfigError)?);

    debug!("Config Read Successfully. {:?}", &config);

    let pool = config.db_pool().change_context(FormulaErrors::ConfigError)?;

    debug!("Connection Pool Established Successfully.");

    let chains = config.chains.clone();

    let listeners = chains.into_iter().map(|chain| {
        let pool = pool.clone();
        tokio::spawn(async move {
            let listener = Listener::try_from(&chain, pool.clone(), chain.chain_id)
                .await
                .change_context(FormulaErrors::ListenerError(chain.chain_id)).unwrap();
            listener.listen().await
        })
    });

    let results = futures::future::join_all(listeners).await;

    Ok(())
}
