mod errors;
pub mod ethnftid;
use std::{str::FromStr, sync::Arc};

use crate::erc165::service::Erc165Service;
use crate::uri_getter::eth_uri_getter::EthUriGetter;
use crate::{config::Chain, ethdto::repo::EthRepo};

use ethnftid::EthNftId;

use crate::erc165::cache_service::Erc165CacheService;
use crate::events::transfer::TransferEvent;
use crate::events::transfer_batch::TransferBatchEvent;
use crate::events::transfer_single::TransferSingleEvent;
use async_trait::async_trait;
use diesel::{r2d2, PgConnection};
use error_stack::{IntoReport, Result, ResultExt};
use ethers::{
    prelude::{Filter, Middleware, Provider, StreamExt, ValueOrArray, Ws, H160, U256},
    utils::to_checksum,
};
use futures::SinkExt;
use tracing::info;

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
    ) -> Result<Self, ListenerError> {
        let provider = Provider::<Ws>::connect(&chain.rpc)
            .await
            .report()
            .attach_printable_lazy(|| format!("Failed to connect to RPC: {}", chain.rpc))
            .change_context(ListenerError::ProviderError)?;
        let erc165_nservice = provider.clone().into();
        let erc165_service = Erc165CacheService::new(pool.clone(), erc165_nservice, chain_id);
        let uri_getter = EthUriGetter::new(provider.clone());
        let eth_repo = EthRepo::new(pool.clone(), uri_getter);

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
        let (mut tx, mut rx) = futures::channel::mpsc::unbounded();
        filter = filter.topic0(Array(vec![
            TransferEvent::topic_h256(),
            TransferSingleEvent::topic_h256(),
            TransferBatchEvent::topic_h256(),
        ]));
        let provider = self.provider.clone();
        let cid = self.chain_id;
        let sender = tokio::spawn(async move {
            let mut subscription = provider.subscribe_logs(&filter).await.unwrap();

            while let Some(log) = subscription.next().await {
                if let Ok(event) = TransferEvent::try_from(&log) {
                    let ethnft = EthNftId {
                        chain_id: cid,
                        contract: to_checksum(&log.address, None),
                        owner: to_checksum(&event.to, None),
                        token_id: event.value.to_string(),
                    };
                    tx.send(ethnft).await.ok();
                }

                if let Ok(event) = TransferSingleEvent::try_from(&log) {
                    if event.value.eq(&U256::one()) {
                        let ethnft = EthNftId {
                            chain_id: cid,
                            contract: to_checksum(&log.address, None),
                            owner: to_checksum(&event.to, None),
                            token_id: event.value.to_string(),
                        };
                        tx.send(ethnft).await.ok();
                    };
                }

                if let Ok(event) = TransferBatchEvent::try_from(&log) {
                    for (i, v) in event.id.iter().enumerate() {
                        if event.value[i].eq(&U256::one()) {
                            let ethnft = EthNftId {
                                chain_id: cid,
                                contract: to_checksum(&log.address, None),
                                owner: to_checksum(&event.to, None),
                                token_id: v.to_string(),
                            };
                            tx.send(ethnft).await.ok();
                        }
                    }
                    // tx.send(event).await.ok()
                }
            }
        });
        let provider = self.provider.clone();
        let erc165_service = self.erc165_service.clone();
        let ethrepo = self.eth_repo.clone();
        let cname = self.name.clone();
        let cid = self.chain_id;
        let receiver = tokio::spawn(async move {
            let mut subscription = provider.subscribe_blocks().await.unwrap();

            while let Some(b) = subscription.next().await {
                if let Some((contracts, ids)) = (&mut rx)
                    .ready_chunks(1000)
                    .map(|logs| {
                        let contracts = logs
                            .clone()
                            .iter()
                            .map(|log| H160::from_str(&log.contract).unwrap())
                            .collect::<Vec<_>>();
                        info!(
                            "Got {:?} EthNfts in block: {:?} for {}[{}]",
                            logs.len(),
                            b.number.unwrap(),
                            cname,
                            cid
                        );
                        (contracts, logs)
                    })
                    .next()
                    .await
                {
                    let erc165_service = erc165_service.clone();
                    let ethrepo = ethrepo.clone();
                    tokio::spawn(async move {
                        let erc165res = erc165_service.supported_traits(&*contracts).await.unwrap();
                        let _ = ethrepo.in_or_up_gen(&ids, erc165res).await;
                    })
                    .await
                    .ok();
                }
            }
        });
        let _ = tokio::join!(sender, receiver);
        Ok(())
    }
}
