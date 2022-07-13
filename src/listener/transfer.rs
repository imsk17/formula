use crate::listener::errors::EventParsingError;
use ethers::abi::AbiDecode;
use ethers::prelude::{Log, U256};
use ethers::types::Address;
use ethers::utils::keccak256;
use serde::de::Unexpected::Str;

pub struct Transfer {
    from: String,
    to: String,
    value: i128,
}

impl TryFrom<&Log> for Transfer {
    type Error = EventParsingError;

    fn try_from(log: &Log) -> Result<Self, Self::Error> {
        if log.topics.len() != 4 {
            return Err(EventParsingError::IncorrectTopicsLength {
                got: log.topics.len(),
            });
        }

        if log.topics[1].to_string() == Transfer::topic_str() {
            return Err(EventParsingError::IncorrectTopic {
                got: log.topics[1].to_string(),
                expected: Transfer::topic_str(),
            });
        }

        Ok(Transfer {
            from: Address::from(log.topics[1]).to_string(),
            to: Address::from(log.topics[2]).to_string(),
            value: U256::decode(&log.data).unwrap().as_u128() as i128,
        })
    }
}

impl Transfer {
    pub fn topic() -> [u8; 32] {
        keccak256("Transfer(address,address,uint256)")
    }
    pub fn topic_str() -> String {
        String::from_utf8(Self::topic().to_vec()).unwrap()
    }
}
