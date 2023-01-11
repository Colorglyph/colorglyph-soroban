use soroban_sdk::{contracttype, AccountId, Vec, BytesN, contracterror};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub enum SourceAccount {
    #[default]
    None,
    AccountId(AccountId)
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Glyph(BytesN<32>),
    GlyOwner(BytesN<32>),
    GlyMinter(BytesN<32>),
    TokenId,
    FeeIden
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u32)]
pub enum Error {
    NotFound = 1,
    NotEmpty = 2,
    NotAuthorized = 3,
}

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
    pub Color,
    pub u32 // amount
);

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Glyph {
    pub width: u32,
    pub colors: Vec<(u32, Vec<(u32, Vec<u32>)>)>, // [[miner, [color, [index]]]
}