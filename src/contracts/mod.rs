use ethers::prelude::*;

abigen!(
    ERC165Contract,
    "./src/contracts/ERC165Contract.json",
    event_derives (serde::Deserialize, serde::Serialize);

    ERC1155Contract,
    "./src/contracts/ERC1155Contract.json",
    event_derives (serde::Deserialize, serde::Serialize);

    ERC721Contract,
    "./src/contracts/ERC721Contract.json",
    event_derives (serde::Deserialize, serde::Serialize);
);
