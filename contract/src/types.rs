use soroban_sdk::{contracttype, AccountId, Vec, BytesN, Address};

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
    GlyMinter(BytesN<32>)
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, Copy)]
pub struct ColorOwned (
    pub u32, // owner
    pub u32, // color hex
    pub u32 // miner
);

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, Copy)]
pub struct Color (
    pub u32, // color hex
    pub u32 // miner
);

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, Copy)]
pub struct ColorAmount (
    pub Color,
    pub u32 // amount
);

#[contracttype]
#[derive(PartialEq, Debug, Clone)]
pub struct Glyph {
    pub width: u32,
    pub colors: Vec<(u32, Vec<(u32, Vec<u32>)>)>, // [[miner, [color, [index]]]
}