use soroban_auth::Signature;
use soroban_sdk::{contracttype, AccountId, Vec, BytesN, contracterror, Address};

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
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TradeOwner {
    Address(Address),
    GlyphOwner(GlyphOwner)
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
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AssetType {
    Glyph(BytesN<32>), // Glyph hash
    Asset(AssetAmount), // Token contract id, amount
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GlyphOwner (
    pub Address, 
    pub Vec<AssetAmount>, // TODO: maybe Vec<(BytesN<32>, i128)>
    pub u32
);

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssetAmount (
    pub BytesN<32>, // Token contract id
    pub i128 // amount
);

#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ColorOwned (
    pub u32, // owner
    pub u32, // color hex
    pub u32 // miner
);

#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Color (
    pub u32, // color hex
    pub u32 // miner
);

#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ColorAmount (
    pub Color, // TODO: maybe break this into (color hex, miner)
    pub u32 // amount
);

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Glyph {
    pub width: u32,
    pub colors: Vec<(u32, Vec<(u32, Vec<u32>)>)>, // [[miner, [color, [index]]]
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Trade {
    pub buy: BytesN<32>,
    pub sell: BytesN<32>,
    pub amount: i128,
}