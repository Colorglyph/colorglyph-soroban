use soroban_sdk::{contracterror, contracttype, Address, BytesN, Vec};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u32)]
pub enum Error {
    NotFound = 1,
    NotEmpty = 2,
    NotAuthorized = 3,
    NotPermitted = 4,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub enum StorageKey {
    #[default]
    None,
    InitToken,
    InitFee,
    Glyph(BytesN<32>),
    GlyphOwner(BytesN<32>),
    GlyphMinter(BytesN<32>),
    GlyphOffer(BytesN<32>),
    AssetOffer(AssetOffer),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MinerColorAmount(
    pub Address, // miner
    pub u32,     // color
    pub u32,     // amount
);

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MinerOwnerColor(
    pub Address, // miner
    pub Address, // owner
    pub u32,     // color
);

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Glyph {
    pub width: u32,
    pub length: u32,
    pub colors: Vec<(Address, Vec<(u32, Vec<u32>)>)>, // [[miner, [[color, [index]]]]
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OfferType {
    Glyph(BytesN<32>),
    Asset(AssetAmount),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Offer {
    Glyph(GlyphOfferArg),
    Asset(AssetOfferArg),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GlyphOfferArg(
    pub u32,            // offer index
    pub Vec<OfferType>, // all glyph sell offers
    pub Address,        // offer (and glyph) owner
    pub BytesN<32>,     // glyph hash
);

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssetOfferArg(
    pub Vec<Address>, // all asset sell offers
    pub AssetOffer,
);

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssetOffer(
    // This first arg is needed as we store asset sell offers off the whole buy and sell side of the offer
    pub BytesN<32>, // glyph hash
    pub Address,    // asset address
    pub i128,       // amount
);

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssetAmount(
    pub Address, // asset address
    pub i128,    // amount
);
