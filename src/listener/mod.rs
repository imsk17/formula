mod errors;
mod transfer;
mod transfer_single;

use crate::config::Chain;
use crate::erc165::erc165_interfaces::Erc165Interface;
use crate::erc165::network_service::Erc165NetworkService;
use crate::erc165::service::Erc165Service;
use crate::listener::transfer::Transfer;
use async_trait::async_trait;
use ethers::abi::{AbiDecode, AbiEncode};
use ethers::prelude::{
    Address, Filter, Middleware, Provider, StreamExt, ValueOrArray, Ws, H256, U256,
};
use ethers::types::{Log, Res, Topic};
use ethers::utils::keccak256;
use eyre::Result;
use tracing::{info, instrument};

#[async_trait]
pub trait Listenable {
    async fn listen(&self) -> Result<()>;
}

pub struct Listener {
    pub name: String,
    pub rpc: String,
    pub provider: Provider<Ws>,
}

impl Listener {
    pub async fn try_from(chain: &Chain) -> Result<Self> {
        let provider = Provider::<Ws>::connect(&chain.rpc).await?;
        Ok(Self {
            provider,
            name: chain.name.clone(),
            rpc: chain.rpc.clone(),
        })
    }
}

#[async_trait]
impl Listenable for Listener {
    async fn listen(&self) -> Result<()> {
        use ValueOrArray::*;
        let mut filter = Filter::new();
        filter = filter.topic0(Array(vec![H256::from(Transfer::topic())]));
        let mut subscription = self.provider.subscribe_logs(&filter).await?;
        while let Some(log) = subscription.next().await {
            let erc_service = Erc165NetworkService::from(self.provider.clone());

            let result = erc_service.supported_traits(&[&log.address]).await.unwrap();

            for (key, value) in &result {
                info!(
                    "[Contract {}: Interfaces {:?}]",
                    key,
                    value.iter().collect::<Vec<_>>()
                );
            }
        }
        Ok(())
    }
}
