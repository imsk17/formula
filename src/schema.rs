table! {
    erc165dto (id) {
        id -> Int4,
        contract -> Varchar,
        chain_id -> Int8,
        erc721 -> Bool,
        erc721_enumerable -> Bool,
        erc721_metadata -> Bool,
        erc1155 -> Bool,
        erc1155_metadata -> Bool,
    }
}

table! {
    ethdto (id) {
        id -> Int4,
        contract -> Varchar,
        chain_id -> Int8,
        contract_type -> Varchar,
        token_id -> Varchar,
        owner -> Varchar,
        uri -> Nullable<Text>,
        name -> Nullable<Text>,
        symbol -> Nullable<Text>,
        updated_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    erc165dto,
    ethdto,
);
