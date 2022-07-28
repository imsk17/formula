#[derive(Debug, Clone)]
pub struct EthNftId {
    pub contract: String,
    pub token_id: String,
    pub owner: String,
    pub chain_id: i64,
}
