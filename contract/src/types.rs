use soroban_auth::Signature;
use soroban_sdk::{contracterror, contracttype, AccountId, Address, BytesN, Vec};

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
    InitFeeId,
    Glyph(BytesN<32>),      // glyph hash
    GlyphOwner(BytesN<32>), // glyph hash
    GlyphMaker(BytesN<32>), // glyph hash
    GlyphOffer(BytesN<32>), // glyph owner
    AssetOffer(AssetOffer),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub enum MaybeAccountId {
    #[default]
    None,
    AccountId(AccountId),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub enum MaybeSignature {
    #[default]
    None,
    Signature(Signature),
}

#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ColorOwner(
    pub u32, // color hex
    pub u32, // miner
    pub u32, // owner
);

#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ColorAmount(
    pub u32,  // color hex
    pub u32,  // miner
    pub i128, // amount
);

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Glyph {
    pub width: u32,
    pub colors: Vec<(u32, Vec<(u32, Vec<u32>)>)>, // [[miner, [color, [index]]]
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OfferType {
    Glyph(BytesN<32>), // glyph hash
    Asset(AssetAmount),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Offer {
    Glyph(GlyphSellOffer),
    Asset(AssetSellOffer),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GlyphSellOffer(
    // TODO: I think including the Sell keyword here is confusing
    pub Address,        // offer (and glyph) owner
    pub BytesN<32>,     // glyph hash
    pub Vec<OfferType>, // all glyph sell offers
    pub u32,            // offer index
);

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssetSellOffer(
    // TODO: I think including the Sell keyword here is confusing
    pub Address,    // offer owner
    pub BytesN<32>, // glyph hash // TODO: I don't think I need this value as you can derive it from the counter offer
    pub BytesN<32>, // asset hash
    pub i128,       // amount
);

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssetOffer(
    pub BytesN<32>, // glyph hash // TODO: Do I definitely need this or can it be derived as needed in a counter offer? If it's not needed we can likely drop this struct and use AssetAmount instead
    pub BytesN<32>, // asset hash
    pub i128,       // amount
);

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssetAmount(
    pub BytesN<32>, // asset hash
    pub i128,       // amount
);
