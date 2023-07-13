#![allow(dead_code)]
#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]
use std::sync::Arc;

use crate::{
    config::AppConfig,
    ethdto::read_repo::EthReadRepo,
    listener::{Listenable, Listener},
    server::routers::nfts::create_nft_router,
};

use axum::Server;
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
mod server;
mod uri_getter;

#[tokio::main]
async fn main() {
    AppConfig::setup_logging();

    let config = Arc::new(AppConfig::from_json5("config").unwrap());

    debug!("Config Read Successfully. {:?}", &config);

    let pool = config.db_pool().unwrap();

    debug!("Connection Pool Established Successfully.");

    let chains = config.chains.clone();

    let listeners = chains.into_iter().map(|chain| {
        let pool = pool.clone();
        tokio::spawn(async move {
            let listener = Listener::try_from(&chain, pool.clone(), chain.chain_id)
                .await
                .unwrap();
            listener.listen().await
        })
    });

    let _ = futures::future::join_all(listeners);

    Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(create_nft_router(EthReadRepo::new(pool.clone())).into_make_service())
        .await
        .unwrap();
}
