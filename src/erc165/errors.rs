use ethers::prelude::{Provider, Ws};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Erc165ServiceErrors {
    #[error("Failed to Query Chain For ERC Interfaces")]
    FailedToQueryChain(#[from] ethers::contract::ContractError<Provider<Ws>>),
}
