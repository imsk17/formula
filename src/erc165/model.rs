use crate::schema::erc165dto;
use std::collections::HashSet;

use diesel::{Insertable, Queryable};

use super::erc165_interfaces::Erc165Interface;

#[derive(Queryable, Debug, Clone)]
pub struct Erc165Dto {
    pub id: i32,
    pub contract: String,
    pub chain_id: i64,
    pub erc721: bool,
    pub erc721_enumerable: bool,
    pub erc721_metadata: bool,
    pub erc1155: bool,
    pub erc1155_metadata: bool,
}

#[derive(Insertable, Debug, Clone)]
#[table_name = "erc165dto"]
pub struct NewErc165Dto {
    pub contract: String,
    pub chain_id: i64,
    pub erc721: bool,
    pub erc721_enumerable: bool,
    pub erc721_metadata: bool,
    pub erc1155: bool,
    pub erc1155_metadata: bool,
}

impl NewErc165Dto {
    pub fn new(contract: String, chain_id: i64, traits: &HashSet<Erc165Interface>) -> Self {
        return NewErc165Dto {
            contract,
            chain_id,
            erc721: traits.contains(&Erc165Interface::ERC721),
            erc721_enumerable: traits.contains(&Erc165Interface::ERC721Enumerable),
            erc721_metadata: traits.contains(&Erc165Interface::ERC721Metadata),
            erc1155: traits.contains(&Erc165Interface::ERC1155),
            erc1155_metadata: traits.contains(&Erc165Interface::ERC1155Metadata),
        };
    }
}

impl Into<(String, HashSet<Erc165Interface>)> for Erc165Dto {
    fn into(self) -> (String, HashSet<Erc165Interface>) {
        let mut traits = HashSet::new();
        if self.erc721 {
            traits.insert(Erc165Interface::ERC721);
        }
        if self.erc721_enumerable {
            traits.insert(Erc165Interface::ERC721Enumerable);
        }
        if self.erc721_metadata {
            traits.insert(Erc165Interface::ERC721Metadata);
        }
        if self.erc1155 {
            traits.insert(Erc165Interface::ERC1155);
        }
        if self.erc1155_metadata {
            traits.insert(Erc165Interface::ERC1155Metadata);
        }
        (self.contract, traits)
    }
}
