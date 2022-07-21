-- Your SQL goes here

CREATE TABLE ethdto
(
    id SERIAL PRIMARY KEY,
    contract VARCHAR(42) NOT NULL,
    chain_id BIGINT NOT NULL,
    contract_type VARCHAR(7) NOT NULL,
    token_id BIGINT NOT NULL,
    owner VARCHAR(42) NOT NULL,
    uri text,
    name text,
    symbol text,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(contract, chain_id, token_id)
);

CREATE INDEX chain_id_idx ON ethdto
(chain_id, owner);


