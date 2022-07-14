use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::sync::Arc;

use crate::contracts::ERC165Contract;
use crate::erc165::errors::Erc165ServiceErrors;
use crate::erc165::service::Erc165Res;
use crate::erc165::service::Erc165Service;
use ethers::abi::Address;
use ethers::prelude::{Provider, Ws, H160};
use eyre::WrapErr;
use tracing::{debug, info};

use super::erc165_interfaces::*;

pub struct Erc165NetworkService {
    provider: Arc<Provider<Ws>>,
}

impl From<Provider<Ws>> for Erc165NetworkService {
    fn from(provider: Provider<Ws>) -> Self {
        Erc165NetworkService {
            provider: Arc::new(provider),
        }
    }
}

#[async_trait::async_trait]
impl Erc165Service for Erc165NetworkService {
    async fn supported_traits(
        &self,
        contracts: &[&H160],
    ) -> Result<Erc165Res, Erc165ServiceErrors> {
        let mut res: Erc165Res = HashMap::new();
        for contract_addr in contracts {
            let mut set = HashSet::<Erc165Interface>::new();
            let contract = ERC165Contract::new(**contract_addr, self.provider.clone());
            let supports_erc165: bool = contract
                .supports_interface(*ERC165)
                .call()
                .await
                .unwrap_or(false);
            if !supports_erc165 {
                continue;
            }
            let supports_erc165n: bool = contract
                .supports_interface(*ERC165N)
                .call()
                .await
                .map_err(|e| Erc165ServiceErrors::FailedToQueryChain(e))?;
            ();
            if supports_erc165n {
                res.insert(contract_addr.to_string(), set);
                continue;
            }
            let supports_erc721: bool = contract
                .supports_interface(*ERC721)
                .call()
                .await
                .map_err(|e| Erc165ServiceErrors::FailedToQueryChain(e))?;
            if supports_erc721 {
                set.insert(Erc165Interface::ERC721);
                let supports_erc721_metadata: bool = contract
                    .supports_interface(*ERC721_METADATA)
                    .call()
                    .await
                    .map_err(|e| Erc165ServiceErrors::FailedToQueryChain(e))?;
                if supports_erc721_metadata {
                    set.insert(Erc165Interface::ERC721Metadata);
                }
                let supports_erc721_enumerable: bool = contract
                    .supports_interface(*ERC721_ENUMERABLE)
                    .call()
                    .await
                    .map_err(|e| Erc165ServiceErrors::FailedToQueryChain(e))?;
                if supports_erc721_enumerable {
                    set.insert(Erc165Interface::ERC721Enumerable);
                }
            }

            let supports_erc1155: bool =
                contract.supports_interface(*ERC1155).call().await.unwrap();
            if supports_erc1155 {
                set.insert(Erc165Interface::ERC1155);
                let supports_erc1155_metadata: bool = contract
                    .supports_interface(*ERC1155_METADATA)
                    .call()
                    .await
                    .map_err(|e| Erc165ServiceErrors::FailedToQueryChain(e))?;
                if supports_erc1155_metadata {
                    set.insert(Erc165Interface::ERC1155Metadata);
                }
            }
            res.insert(contract_addr.to_string(), set);
        }
        Ok(res)
    }
}
