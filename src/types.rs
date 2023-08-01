use soroban_sdk::{contracterror, contracttype, Address, BytesN, Map, Vec};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u32)]
pub enum Error {
    NotFound = 1,
    NotEmpty = 2,
    NotAuthorized = 3,
    NotPermitted = 4,
    MissingWidth = 5,
    MissingId = 6,
    MissingAddress = 7,
    MissingBuy = 8,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StorageKey {
    TokenAddress,
    FeeAddress,
    Color(Address, Address, u32),
    Colors(Address),
    Glyph(BytesN<32>),
    Dust(Address),
    GlyphOwner(BytesN<32>),
    GlyphMinter(BytesN<32>),
    GlyphOffer(BytesN<32>),
    AssetOffer(BytesN<32>, Address, i128),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HashType {
    Colors(Address), // means you can only be building one glyph at a time
    Dust(Address),   // means you can only be scraping one glyph at a time
    Glyph(BytesN<32>),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GlyphType {
    Colors(Map<Address, Map<u32, Vec<u32>>>),
    Glyph(Glyph),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Glyph {
    pub width: u32,
    pub length: u32,
    pub colors: Map<Address, Map<u32, Vec<u32>>>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OfferCreate {
    Glyph(BytesN<32>, Offer),
    Asset(BytesN<32>, Address, Address, i128),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Offer {
    Glyph(BytesN<32>),
    Asset(Address, i128), // BLOCKED once tuples support Option use that instead of AssetSell
    AssetSell(Address, Address, i128),
}
