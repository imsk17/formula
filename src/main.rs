#![allow(dead_code, unused_imports)]
use crate::listener::{Listenable, Listener};
use eyre::Result;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;
use tokio;
use tokio::join;
use tokio::task::JoinHandle;
use tracing::debug;
use tracing_subscriber::filter::EnvFilter;

mod config;
mod contracts;
mod erc165;
mod listener;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .compact()
        .with_line_number(true)
        .with_thread_names(true)
        .init();
    let conf = config::AppConfig::from_json5("config")?;
    debug!("Config Read Successfully. {:?}", conf.clone());
    let listeners: Vec<JoinHandle<_>> = conf
        .chains
        .into_iter()
        .map(move |c| {
            tokio::spawn(async move {
                let listenable = Listener::try_from(&c).await.expect("Shant Fail");
                listenable.listen().await
            })
        })
        .collect();
    let _ = futures::future::join_all(listeners).await;
    Ok(())
}
