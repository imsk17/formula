#![allow(dead_code)]
use crate::{
    config::AppConfig,
    ethdto::repo::EthRepo,
    listener::{Listenable, Listener},
};

use diesel::{r2d2::ConnectionManager, PgConnection};

use tokio;
use tracing::debug;
use tracing_subscriber::filter::EnvFilter;
#[macro_use]
extern crate diesel;
use diesel::r2d2;

mod config;
mod contracts;
mod erc165;
mod ethdto;
mod events;
mod listener;
mod schema;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .compact()
        .with_line_number(true)
        .with_thread_names(true)
        .with_thread_ids(true)
        .init();

    let config = AppConfig::from_json5("config").unwrap();

    let cm = ConnectionManager::<PgConnection>::new(&config.db);

    debug!("Config Read Successfully. {:?}", &config);

    let pool = r2d2::Pool::builder().build(cm).unwrap();

    let chains = config.chains;

    let listeners = chains.into_iter().map(|chain| {
        let pool = pool.clone();
        let eth_repo = EthRepo::new(pool.clone());
        tokio::spawn(async move {
            let listener = Listener::try_from(&chain, pool.clone(), chain.chain_id, eth_repo)
                .await
                .unwrap();
            listener.listen().await
        })
    });

    let _ = futures::future::join_all(listeners).await;
}
