use ethers::abi::RawLog;

use ethers::core::types::*;
use ethers::prelude::{EthEvent, Log, U256};
use ethers::utils::keccak256;

use super::errors::EventParsingError;

#[derive(EthEvent, Debug)]
#[ethevent(
    name = "TransferBatch",
    abi = "TransferBatch(address,address,address,uint256[],uint256[])"
)]
pub struct TransferBatchEvent {
    #[ethevent(indexed)]
    operator: Address,
    #[ethevent(indexed)]
    from: Address,
    #[ethevent(indexed)]
    to: Address,
    id: Vec<U256>,
    value: Vec<U256>,
}

impl TryFrom<&Log> for TransferBatchEvent {
    type Error = EventParsingError;

    fn try_from(log: &Log) -> Result<Self, Self::Error> {
        TransferBatchEvent::decode_log(&RawLog {
            data: log.data.to_vec(),
            topics: log.topics.clone(),
        })
        .map_err(|e| EventParsingError::FailedEventDecoding(e))
    }
}

impl TransferBatchEvent {
    pub fn topic() -> [u8; 32] {
        keccak256("TransferBatch(address,address,address,uint256[],uint256[])")
    }
    pub fn topic_h256() -> H256 {
        H256::from(keccak256(
            "TransferBatch(address,address,address,uint256[],uint256[])",
        ))
    }
    pub fn topic_str() -> String {
        String::from_utf8(Self::topic().to_vec()).unwrap()
    }
}
