use std::collections::{HashMap, HashSet};

use std::sync::Arc;

use crate::contracts::ERC165Contract;
use crate::erc165::errors::Erc165ServiceErrors;
use crate::erc165::service::Erc165Res;
use crate::erc165::service::Erc165Service;

use super::erc165_interfaces::*;
use error_stack::{IntoReport, Result, ResultExt};
use ethers::prelude::{Provider, Ws, H160};
use ethers::utils;

#[derive(Clone)]
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

impl Erc165Service for Erc165NetworkService {
    async fn supported_traits(&self, contracts: &[H160]) -> Result<Erc165Res, Erc165ServiceErrors> {
        let mut res: Erc165Res = HashMap::new();
        for contract_addr in contracts {
            let mut set = HashSet::<Erc165Interface>::new();
            let contract = ERC165Contract::new(*contract_addr, self.provider.clone());
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
                .into_report()
                .attach_printable_lazy(|| {
                    format!("Failed to query erc165n interface for {:?}", contract)
                })
                .change_context(Erc165ServiceErrors::FailedToQueryChain)?;
            if supports_erc165n {
                res.insert(utils::to_checksum(contract_addr, None), set);
                continue;
            }
            let supports_erc721: bool = contract
                .supports_interface(*ERC721)
                .call()
                .await
                .into_report()
                .attach_printable_lazy(|| {
                    format!("Failed to query erc721 interface for {:?}", contract)
                })
                .change_context(Erc165ServiceErrors::FailedToQueryChain)?;
            if supports_erc721 {
                set.insert(Erc165Interface::ERC721);
                let supports_erc721_metadata: bool = contract
                    .supports_interface(*ERC721_METADATA)
                    .call()
                    .await
                    .into_report()
                    .attach_printable_lazy(|| {
                        format!(
                            "Failed to query erc721 metadata interface for {:?}",
                            contract
                        )
                    })
                    .change_context(Erc165ServiceErrors::FailedToQueryChain)?;
                if supports_erc721_metadata {
                    set.insert(Erc165Interface::ERC721Metadata);
                }
                let supports_erc721_enumerable: bool = contract
                    .supports_interface(*ERC721_ENUMERABLE)
                    .call()
                    .await
                    .into_report()
                    .attach_printable_lazy(|| {
                        format!(
                            "Failed to query erc721 enumerable interface for {:?}",
                            contract
                        )
                    })
                    .change_context(Erc165ServiceErrors::FailedToQueryChain)?;
                if supports_erc721_enumerable {
                    set.insert(Erc165Interface::ERC721Enumerable);
                }
                res.insert(utils::to_checksum(contract_addr, None), set);
                continue;
            }

            let supports_erc1155: bool = contract
                .supports_interface(*ERC1155)
                .call()
                .await
                .into_report()
                .attach_printable_lazy(|| {
                    format!("Failed to query erc1155 interface for {:?}", contract)
                })
                .change_context(Erc165ServiceErrors::FailedToQueryChain)?;
            if supports_erc1155 {
                set.insert(Erc165Interface::ERC1155);
                let supports_erc1155_metadata: bool = contract
                    .supports_interface(*ERC1155_METADATA)
                    .call()
                    .await
                    .into_report()
                    .attach_printable_lazy(|| {
                        format!(
                            "Failed to query erc1155 metadata interface for {:?}",
                            contract
                        )
                    })
                    .change_context(Erc165ServiceErrors::FailedToQueryChain)?;
                if supports_erc1155_metadata {
                    set.insert(Erc165Interface::ERC1155Metadata);
                }
            }
            res.insert(utils::to_checksum(contract_addr, None), set);
        }
        Ok(res)
    }
}
