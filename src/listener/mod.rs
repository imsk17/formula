mod errors;
mod transfer;
mod transfer_batch;
mod transfer_single;

use std::sync::Arc;

use crate::config::Chain;

use crate::erc165::cache_service::Erc165CacheService;

use crate::listener::transfer::TransferEvent;
use crate::listener::transfer_batch::TransferBatchEvent;
use crate::listener::transfer_single::TransferSingleEvent;
use async_trait::async_trait;

use diesel::{r2d2, PgConnection};
use ethers::prelude::{Filter, Middleware, Provider, StreamExt, ValueOrArray, Ws};
use eyre::Result;
#[async_trait]
pub trait Listenable {
    async fn listen(&self) -> Result<()>;
}

pub type PgPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

pub struct Listener {
    pub name: String,
    pub rpc: String,
    pub provider: Provider<Ws>,
    pub pool: Arc<PgPool>,
    pub chain_id: i64,
    pub _erc165_service: Erc165CacheService,
}

impl Listener {
    pub async fn try_from(chain: &Chain, pool: PgPool, chain_id: i64) -> Result<Self> {
        let provider = Provider::<Ws>::connect(&chain.rpc).await?;
        let erc165_nservice = provider.clone().into();
        let _erc165_service = Erc165CacheService::new(pool.clone(), erc165_nservice, chain_id);

        Ok(Self {
            provider,
            name: chain.name.clone(),
            rpc: chain.rpc.clone(),
            pool: Arc::new(pool),
            chain_id,
            _erc165_service,
        })
    }
}

#[async_trait]
impl Listenable for Listener {
    async fn listen(&self) -> Result<()> {
        use ValueOrArray::*;
        let mut filter = Filter::new();
        filter = filter.topic0(Array(vec![
            TransferEvent::topic_h256(),
            TransferSingleEvent::topic_h256(),
        ]));
        let mut subscription = self.provider.subscribe_logs(&filter).await?;
        while let Some(log) = subscription.next().await {
            if log.topics.len() == 4 {
                if log.topics[0] == TransferEvent::topic_h256() {
                    let _event = TransferEvent::try_from(&log).unwrap();
                } else if log.topics[0] == TransferSingleEvent::topic_h256() {
                    let _event = TransferSingleEvent::try_from(&log).unwrap();
                    continue;
                } else if log.topics[0] == TransferBatchEvent::topic_h256() {
                    let _event = TransferBatchEvent::try_from(&log).unwrap();
                    continue;
                }
            }
        }
        Ok(())
    }
}
