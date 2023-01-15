use soroban_auth::Signature;
use soroban_sdk::{contracttype, AccountId, Vec, BytesN, contracterror, Address};

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
    Glyph(BytesN<32>), // glyph hash
    GlyphOwner(BytesN<32>), // glyph hash
    GlyphMaker(BytesN<32>), // glyph hash
    InitToken,
    InitFeeId
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub enum Side {
    #[default]
    None,
    Buy,
    Sell,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub enum MaybeAccountId {
    #[default]
    None,
    AccountId(AccountId)
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub enum MaybeSignature {
    #[default]
    None,
    Signature(Signature)
}

#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ColorOwner (
    pub u32, // color hex
    pub u32, // miner
    pub u32, // owner
);

#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ColorAmount (
    pub u32, // color hex
    pub u32, // miner
    pub i128, // amount
);

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AssetType {
    Glyph(BytesN<32>), // Glyph hash
    Asset(AssetAmount), // Token contract id, amount
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssetAmount (
    pub BytesN<32>, // Token contract id
    pub i128 // amount
);

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OfferOwner {
    Address(Address),
    Glyph(GlyphOffer)
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GlyphOffer (
    pub Address, // offer owner // TODO: not sure I need to track this as glyph offers will always be owned by the current glyph owner
    pub u32, // offer index
    pub Vec<AssetAmount>, // all glyph sell offers
);

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Glyph {
    pub width: u32,
    pub colors: Vec<(u32, Vec<(u32, Vec<u32>)>)>, // [[miner, [color, [index]]]
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Offer {
    pub buy: BytesN<32>,
    pub sell: BytesN<32>,
    pub amount: i128,
}