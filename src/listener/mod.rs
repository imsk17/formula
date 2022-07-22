mod errors;
mod uri_getter;
use std::sync::Arc;

use crate::erc165::service::Erc165Service;
use crate::ethdto::repo::EthRepo;
use crate::listener::uri_getter::eth_nft_uri_getter;
use crate::{config::Chain, listener::uri_getter::EthNftId};

use crate::erc165::cache_service::Erc165CacheService;

use crate::events::transfer::TransferEvent;
use crate::events::transfer_batch::TransferBatchEvent;
use crate::events::transfer_single::TransferSingleEvent;
use async_trait::async_trait;
use diesel::{r2d2, PgConnection};
use error_stack::{IntoReport, Result, ResultExt};
use ethers::prelude::{Filter, Middleware, Provider, StreamExt, ValueOrArray, Ws};
use ethers::utils;
use tracing::warn;

use self::errors::ListenerError;

#[async_trait]
pub trait Listenable {
    async fn listen(&self) -> Result<(), ListenerError>;
}

pub type PgPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

pub struct Listener {
    pub name: String,
    pub rpc: String,
    pub provider: Provider<Ws>,
    pub pool: Arc<PgPool>,
    pub chain_id: i64,
    pub erc165_service: Erc165CacheService,
    pub eth_repo: EthRepo,
}

impl Listener {
    pub async fn try_from(
        chain: &Chain,
        pool: PgPool,
        chain_id: i64,
        eth_repo: EthRepo,
    ) -> Result<Self, ListenerError> {
        let provider = Provider::<Ws>::connect(&chain.rpc)
            .await
            .report()
            .attach_printable_lazy(|| format!("Failed to connect to RPC: {}", chain.rpc))
            .change_context(ListenerError::ProviderError)?;
        let erc165_nservice = provider.clone().into();
        let erc165_service = Erc165CacheService::new(pool.clone(), erc165_nservice, chain_id);

        Ok(Self {
            provider,
            name: chain.name.clone(),
            rpc: chain.rpc.clone(),
            pool: Arc::new(pool),
            chain_id,
            erc165_service,
            eth_repo,
        })
    }
}

#[async_trait]
impl Listenable for Listener {
    async fn listen(&self) -> Result<(), ListenerError> {
        use ValueOrArray::*;
        let mut filter = Filter::new();
        filter = filter.topic0(Array(vec![
            TransferEvent::topic_h256(),
            TransferSingleEvent::topic_h256(),
            TransferBatchEvent::topic_h256(),
        ]));
        let mut subscription = self.provider.subscribe_logs(&filter).await.unwrap();

        while let Some(log) = subscription.next().await {
            if log.topics.len() == 4 {
                if log.topics[0] == TransferEvent::topic_h256() {
                    let event = TransferEvent::try_from(&log).unwrap();
                    let id = EthNftId {
                        chain_id: self.chain_id,
                        token_id: event.value.to_string(),
                        contract: utils::to_checksum(&log.address, None),
                        owner: utils::to_checksum(&event.to, None),
                    };
                    let erc165_res = self
                        .erc165_service
                        .supported_traits(&[&log.address])
                        .await
                        .change_context(ListenerError::Erc165ResError)?;
                    if erc165_res.len() == 0 {
                        warn!("No supported traits for contract {}", log.address);
                        continue;
                    }
                    let res =
                        eth_nft_uri_getter(self.provider.clone(), erc165_res.clone(), id).await;
                    if let Some(newdto) = res {
                        self.eth_repo.in_or_up_gen(&[newdto]).unwrap();
                    }
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
