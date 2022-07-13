use crate::erc165::erc165_interfaces::Erc165Interface;
use crate::erc165::errors::Erc165ServiceErrors;
use async_trait::async_trait;
use ethers::prelude::H160;
use std::collections::{HashMap, HashSet};

pub type Erc165Res = HashMap<String, HashSet<Erc165Interface>>;

#[async_trait]
pub trait Erc165Service {
    async fn supported_traits(&self, contracts: &[&H160])
        -> Result<Erc165Res, Erc165ServiceErrors>;
}
