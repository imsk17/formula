use ethers::abi::RawLog;

use error_stack::{Report, ResultExt};
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
    pub operator: Address,
    #[ethevent(indexed)]
    pub from: Address,
    #[ethevent(indexed)]
    pub to: Address,
    pub id: Vec<U256>,
    pub value: Vec<U256>,
}

impl TryFrom<&Log> for TransferBatchEvent {
    type Error = Report<EventParsingError>;
    fn try_from(log: &Log) -> Result<TransferBatchEvent, Report<EventParsingError>> {
        TransferBatchEvent::decode_log(&RawLog {
            data: log.data.to_vec(),
            topics: log.topics.clone(),
        })
        .map_err(Report::from)
        .attach_printable_lazy(|| {
            format!("Failed to decode Transfer Batch Event From Log: {:?}", log)
        })
        .change_context(EventParsingError::FailedEventDecoding)
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
