use ethers::abi::RawLog;

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
    operator: Address,
    #[ethevent(indexed)]
    from: Address,
    #[ethevent(indexed)]
    to: Address,
    id: U256,
    value: U256,
}

impl TryFrom<&Log> for TransferSingleEvent {
    type Error = EventParsingError;

    fn try_from(log: &Log) -> Result<Self, Self::Error> {
        TransferSingleEvent::decode_log(&RawLog {
            data: log.data.to_vec(),
            topics: log.topics.clone(),
        })
        .map_err(|e| EventParsingError::FailedEventDecoding(e))
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
