use ethers::abi::AbiDecode;
use ethers::prelude::{Log, U256};
use ethers::types::Address;
use ethers::utils::keccak256;

pub struct Transfer {
    from: String,
    to: String,
    value: i128,
}

impl From<&Log> for Transfer {
    fn from(log: &Log) -> Self {
        Transfer {
            from: Address::from(log.topics[1]).to_string(),
            to: Address::from(log.topics[2]).to_string(),
            value: U256::decode(&log.data).unwrap().as_u128() as i128,
        }
    }
}

impl Transfer {
    pub fn topic() -> [u8; 32] {
        keccak256("Transfer(address,address,uint256)")
    }
}
