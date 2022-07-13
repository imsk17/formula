use ethers::prelude::{builders::ContractCall, *};

abigen!(
    ERC165Contract,
    "./src/contracts/ERC165Contract.json",
    event_derives(serde::Deserialize, serde::Serialize)
);
