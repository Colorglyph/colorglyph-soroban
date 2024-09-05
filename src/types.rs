use soroban_sdk::{contracterror, contracttype, Address, BytesN, Map, Vec};

#[contracterror]
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u32)]
pub enum Error { // TODO Clean out unused errors
    NotFound = 1,
    NotEmpty = 2,
    NotAuthorized = 3,
    NotPermitted = 4,
    MissingWidth = 5,
    MissingId = 6,
    MissingAddress = 7,
    MissingBuy = 8,
    NotInitialized = 9,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum StorageKey {
    OwnerAddress,
    TokenAddress,
    FeeAddress,
    MaxEntryLifetime,
    MaxPaymentCount,
    MineMultiplier,
    MinterRoyaltyRate,
    MinerRoyaltyRate,
    Color(Address, Address, u32), // (miner, owner, color) : amount 
    Glyph(BytesN<32>),
    GlyphOwner(BytesN<32>),
    GlyphMinter(BytesN<32>),
    GlyphOffer(BytesN<32>),
    AssetOffer(BytesN<32>, Address, i128), // (hash, sac, amount) : Vec<Address>
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct Glyph {
    pub width: u32,
    pub length: u32,
    pub colors: Map<Address, Map<u32, Vec<u32>>>,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum OfferCreate {
    Glyph(BytesN<32>, Offer),
    Asset(BytesN<32>, Address, Address, i128),
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum Offer {
    Glyph(BytesN<32>),
    Asset(Address, i128), // BLOCKED once tuples support Option use that instead of AssetSell
    AssetSell(Address, Address, i128), // owner, sac, amount (NOTE currently this offer type is never stored)
}
