use soroban_sdk::{contracttype, AccountId, Map, Vec};

#[contracttype]
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub enum SourceAccount {
    #[default]
    None,
    AccountId(AccountId)
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, Copy)]
pub struct OwnedColor (
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
#[derive(PartialEq, Debug, Clone)]
pub struct Glyph {
    pub width: u32,
    pub colors: Map<u32, Vec<(u32, u32)>>, // miner: <index, color> // TODO should this Vec be a Map? <index: color>
}