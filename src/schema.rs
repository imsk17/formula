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
