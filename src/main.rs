#![allow(dead_code)]
use crate::{
    config::AppConfig,
    listener::{Listenable, Listener},
};
use diesel::{r2d2::ConnectionManager, PgConnection};
use eyre::Result;

use tokio;
use tracing::debug;
use tracing_subscriber::filter::EnvFilter;
#[macro_use]
extern crate diesel;
use diesel::r2d2;

mod config;
mod contracts;
mod erc165;
mod listener;
mod schema;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .compact()
        .with_line_number(true)
        .with_thread_names(true)
        .init();

    let config = AppConfig::from_json5("config")?;

    let cm = ConnectionManager::<PgConnection>::new(&config.db);

    debug!("Config Read Successfully. {:?}", config.clone());

    let pool = r2d2::Pool::builder().build(cm).unwrap();

    let mut listeners = vec![];

    for chain in config.chains.clone().into_iter() {
        let pool = pool.clone();
        let chain_id = chain.chain_id;

        let handle = tokio::spawn(async move {
            let listener = Listener::try_from(&chain, pool.clone(), chain_id).await?;
            listener.listen().await
        });
        listeners.push(handle);
    }
    let _ = futures::future::join_all(listeners).await;
    Ok(())
}
