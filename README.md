# formula

> Listen for NFT Events on the chain and feed them into a database

## Features 
- [X] Supports ERC721 and ERC1155
- [X] Checks for ERC165 Supported Interfaces 
- [X] Server to fetch NFTs for a user
- [ ] Multiple Database Adapters (Only Supports Postgres For Now)
- [ ] Support for more chains (NON EVM)

## Release

Latest Release Can Be Fetched from the Releases Section of this repoistory.

> OR
 
## How to Build

Make sure you have rust installed on your machine

- Clone the repo `git clone https://github.com/imsk17/formula --depth=1`
- Open the directory `cd formula`
- Build the binary using the command `cargo build --release`
- Copy the config.json5.example to config.json5 and fill in the required details `cp config.json5.example config.json5` 
- Fire the procees using `cargo run --release`
- Server is running up on host and port that you specify in config.json5 .

# License

© 2024 Sumit Kumar - [Apache License V2](./LICENSE)
