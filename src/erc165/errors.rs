use thiserror::Error;

#[derive(Error, Debug)]
pub enum Erc165ServiceErrors {
    #[error("Failed to Query Chain For ERC Interfaces")]
    FailedToQueryChain,
}
