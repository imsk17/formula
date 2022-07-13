#[derive(Eq, Hash, PartialEq, Debug)]
pub enum Erc165Interface {
    ERC721 = 0x80ac58cd,
    ERC721Metadata = 0x5b5e139f,
    ERC721Enumerable = 0x780e9d63,
    ERC1155 = 0xd9b67a26,
    ERC1155Metadata = 0x0e89341c,
}

pub const ERC165: &'static [u8; 4] = &[0x01, 0xff, 0xc9, 0xa7];

pub const ERC165N: &'static [u8; 4] = &[0xff, 0xff, 0xff, 0xff];

pub const ERC721: &'static [u8; 4] = &[0x80, 0xac, 0x58, 0xcd];

pub const ERC721_METADATA: &'static [u8; 4] = &[0x5b, 0x5e, 0x13, 0x9f];

pub const ERC721_ENUMERABLE: &'static [u8; 4] = &[0x78, 0x0e, 0x9d, 0x63];

pub const ERC1155: &'static [u8; 4] = &[0xd9, 0xb6, 0x7a, 0x26];

pub const ERC1155_METADATA: &'static [u8; 4] = &[0x0e, 0x89, 0x34, 0x1c];
