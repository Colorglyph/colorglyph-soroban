use soroban_sdk::{contracterror, contracttype, Address, BytesN, Map, Vec};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u32)]
pub enum Error {
    GroguDown = 0,
    NotFound = 1,
    NotEmpty = 2,
    NotAuthorized = 3,
    NotPermitted = 4,
    MissingWidth = 5,
    MissingId = 6,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StorageKey {
    TokenAddress,
    FeeAddress,
    Colors(u64),
    Glyph(BytesN<32>),
    GlyphOwner(BytesN<32>),
    GlyphMinter(BytesN<32>),
    GlyphOffer(BytesN<32>),
    AssetOffer(BytesN<32>, Address, i128),
    Color(Address, Address, u32),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HashId {
    Id(u64),
    Hash(BytesN<32>),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GlyphType {
    Glyph(Glyph),
    Colors(Map<Address, Map<u32, Vec<u32>>>),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Glyph {
    pub width: u32,
    pub length: u32,
    pub colors: Map<Address, Map<u32, Vec<u32>>>, // [[miner, [[color, [index]]]]
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OfferType {
    Glyph(BytesN<32>),
    Asset(Address, i128),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Offer {
    Glyph(u32, Vec<OfferType>, Address, BytesN<32>),
    Asset(Vec<Address>, BytesN<32>, Address, i128),
}
