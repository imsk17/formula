use crate::events::errors::EventParsingError;
use ethers::abi::AbiDecode;
use ethers::prelude::{Log, H256, U256};
use ethers::types::Address;
use ethers::utils::keccak256;

#[derive(Debug)]
pub struct TransferEvent {
    pub from: Address,
    pub to: Address,
    pub value: U256,
}

impl TransferEvent {
    pub fn try_from(log: &Log) -> Result<Self, EventParsingError> {
        if log.topics.len() != 4 {
            return Err(EventParsingError::IncorrectTopicsLength {
                got: log.topics.len(),
            });
        }

        if log.topics[1].to_string() == TransferEvent::topic_str() {
            return Err(EventParsingError::IncorrectTopic {
                got: log.topics[1].to_string(),
                expected: TransferEvent::topic_str(),
            });
        }

        Ok(TransferEvent {
            from: Address::from(log.topics[1]),
            to: Address::from(log.topics[2]),
            value: U256::decode(log.topics[3]).unwrap(),
        })
    }
}

impl TransferEvent {
    pub fn topic() -> [u8; 32] {
        keccak256("Transfer(address,address,uint256)")
    }
    pub fn topic_h256() -> H256 {
        H256::from(keccak256("Transfer(address,address,uint256)"))
    }
    pub fn topic_str() -> String {
        H256::from(keccak256("Transfer(address,address,uint256)")).to_string()
    }
}
