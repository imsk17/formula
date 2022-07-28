use std::{collections::HashSet, str::FromStr, sync::Arc};

use chrono::Utc;
use ethers::prelude::{Provider, Ws, H160, U256};

use crate::{
    contracts::{ERC1155Contract, ERC721Contract},
    erc165::{erc165_interfaces::Erc165Interface, service::Erc165Res},
    ethdto::dto::NewEthDto,
};

use super::ethnftid::EthNftId;

pub async fn eth_nft_uri_getter(
    provider: Provider<Ws>,
    erc165_res: Erc165Res,
    id: EthNftId,
) -> Option<NewEthDto> {
    let e = HashSet::<Erc165Interface>::new();
    let traits = erc165_res.get(&id.contract).unwrap_or(&e);
    let mut ret: Option<NewEthDto> = None;
    if traits.contains(&Erc165Interface::ERC721) {
        let contract = ERC721Contract::new(
            H160::from_str(&id.contract).unwrap(),
            Arc::new(provider.clone()),
        );
        let token = U256::from_dec_str(&id.token_id)
            .map_err(|e| {
                panic!("TokenID: {} - {}", &id.token_id, e);
            })
            .unwrap();
        let uri_txn = contract.token_uri(token);
        let name: String = contract.name().call().await.unwrap_or_default();
        let symbol: String = contract.symbol().call().await.unwrap_or_default();
        let uri: String = uri_txn.call().await.unwrap_or_default();

        ret = Some(NewEthDto {
            chain_id: id.chain_id,
            contract: id.contract,
            contract_type: "ERC721".to_string(),
            name: if name == "" { None } else { Some(name) },
            symbol: if symbol == "" { None } else { Some(symbol) },
            uri: if uri == "" { None } else { Some(uri) },
            owner: id.owner,
            token_id: id.token_id,
            updated_at: Utc::now().naive_utc(),
        });
    } else if traits.contains(&Erc165Interface::ERC1155) {
        let contract = ERC1155Contract::new(
            H160::from_str(&id.contract).unwrap(),
            Arc::new(provider.clone()),
        );
        let token = U256::from_str(&id.token_id).unwrap();
        let uri_txn = contract.uri(token);

        let uri: String = uri_txn.call().await.unwrap_or_default();

        ret = Some(NewEthDto {
            chain_id: id.chain_id,
            contract: id.contract,
            contract_type: "ERC721".to_string(),
            name: None,
            symbol: None,
            uri: if uri == "" { None } else { Some(uri) },
            owner: id.owner,
            token_id: id.token_id,
            updated_at: Utc::now().naive_utc(),
        });
    }

    ret
}
