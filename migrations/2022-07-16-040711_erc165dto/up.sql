-- Your SQL goes here

CREATE TABLE erc165dto
(
    id SERIAL PRIMARY KEY,
    contract VARCHAR(42) NOT NULL,
    chain_id BIGINT NOT NULL,
    erc721 BOOLEAN NOT NULL,
    erc721_enumerable BOOLEAN NOT NULL,
    erc721_metadata BOOLEAN NOT NULL,
    erc1155 BOOLEAN NOT NULL,
    erc1155_metadata BOOLEAN NOT NULL,
    UNIQUE(contract, chain_id)
)