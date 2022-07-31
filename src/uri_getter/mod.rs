use crate::{erc165::service::Erc165Res, ethdto::dto::NewEthDto, listener::ethnftid::EthNftId};
pub mod eth_uri_getter;

#[async_trait::async_trait]
pub trait UriGetter {
    async fn get_uri(&self, res: Erc165Res, id: EthNftId) -> Option<NewEthDto>;
}
