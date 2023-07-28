use soroban_sdk::{contracterror, contracttype, Address, BytesN, Map, Vec};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u32)]
pub enum Error {
    NotFound = 1,
    NotEmpty = 2,
    NotAuthorized = 3,
    NotPermitted = 4,
    MissingAddress = 5,
    MissingWidth = 6,
    MissingId = 7,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StorageKey {
    TokenAddress,
    FeeAddress,
    Color(Address, Address, u32),
    Colors(Address),
    Dust(Address),
    Glyph(BytesN<32>),
    GlyphOwner(BytesN<32>),
    GlyphMinter(BytesN<32>),
    GlyphOffer(BytesN<32>),
    AssetOffer(BytesN<32>, Address, i128),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HashType {
    Colors, // means you can only be building one glyph at a time
    Dust,   // means you can only be scraping one glyph at a time
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
pub enum OfferType {
    Glyph(BytesN<32>),
    Asset(Address, i128),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Offer {
    Glyph(Vec<OfferType>, BytesN<32>, Address, u32),
    Asset(Vec<Address>, BytesN<32>, Address, i128),
}
