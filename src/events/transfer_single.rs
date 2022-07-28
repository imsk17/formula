use ethers::abi::RawLog;

use error_stack::{IntoReport, Report, ResultExt};
use ethers::prelude::{Address, EthEvent, Log, H256, U256};
use ethers::utils::keccak256;

use super::errors::EventParsingError;

#[derive(EthEvent, Debug)]
#[ethevent(
    name = "TransferSingle",
    abi = "TransferSingle(address,address,address,uint256,uint256)"
)]
pub struct TransferSingleEvent {
    #[ethevent(indexed)]
    pub operator: Address,
    #[ethevent(indexed)]
    pub from: Address,
    #[ethevent(indexed)]
    pub to: Address,
    pub id: U256,
    pub value: U256,
}

impl TryFrom<&Log> for TransferSingleEvent {
    type Error = Report<EventParsingError>;
    fn try_from(log: &Log) -> Result<TransferSingleEvent, Report<EventParsingError>> {
        TransferSingleEvent::decode_log(&RawLog {
            data: log.data.to_vec(),
            topics: log.topics.clone(),
        })
        .report()
        .attach_printable_lazy(|| {
            format!("Failed to decode Transfer Batch Event From Log: {:?}", log)
        })
        .change_context(EventParsingError::FailedEventDecoding)
    }
}

impl TransferSingleEvent {
    pub fn topic() -> [u8; 32] {
        keccak256("TransferSingle(address,address,address,uint256,uint256)")
    }
    pub fn topic_h256() -> H256 {
        H256::from(keccak256(
            "TransferSingle(address,address,address,uint256,uint256)",
        ))
    }
    pub fn topic_str() -> String {
        String::from_utf8(Self::topic().to_vec()).unwrap()
    }
}
