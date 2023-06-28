use soroban_sdk::{contractimpl, panic_with_error, Address, BytesN, Env, Vec};

use crate::{
    colors::{get_color, mine, transfer},
    glyphs::{get_glyph, mint, scrape},
    offers::{get_offer, offer, rm_offer},
    types::{Error, Glyph, MinerColorAmount, Offer, OfferType, StorageKey},
};

pub struct ColorGlyph;

// TODO:
// Fine tooth comb what functions actually need to be public

#[contractimpl]
impl ColorGlyph {
    pub fn init(env: Env, token_id: Address, fee_address: Address) {
        if env.storage().has(&StorageKey::InitToken) {
            panic_with_error!(env, Error::NotEmpty);
        }

        env.storage().set(&StorageKey::InitToken, &token_id);
        env.storage().set(&StorageKey::InitFee, &fee_address);
    }

    // Colors
    pub fn mine(env: Env, from: Address, to: Option<Address>, colors: Vec<(u32, u32)>) {
        mine(&env, from, colors, to);
    }
    pub fn get_color(env: Env, owner: Address, miner: Option<Address>, hex: u32) -> u32 {
        get_color(&env, owner, miner, hex)
    }
    pub fn transfer(env: Env, from: Address, to: Address, colors: Vec<MinerColorAmount>) {
        transfer(&env, from, to, colors);
    }

    // Glyphs
    pub fn mint(
        env: Env,
        from: Address,
        width: u32,
        colors: Vec<(Address, Vec<(u32, Vec<u32>)>)>,
    ) -> BytesN<32> {
        mint(&env, from, width, colors)
    }
    pub fn get_glyph(env: Env, hash: BytesN<32>) -> Result<Glyph, Error> {
        get_glyph(&env, hash)
    }
    pub fn scrape(env: Env, from: Address, hash: BytesN<32>) -> Result<(), Error> {
        scrape(&env, from, hash)
    }

    // Offers
    pub fn offer(env: Env, from: Address, buy: OfferType, sell: OfferType) -> Result<(), Error> {
        offer(&env, from, &buy, &sell)
    }
    pub fn get_offer(env: Env, buy: OfferType, sell: OfferType) -> Result<Offer, Error> {
        get_offer(&env, &buy, &sell)
    }
    pub fn rm_offer(env: Env, from: Address, buy: OfferType, sell: OfferType) {
        rm_offer(&env, from, &buy, &sell);
    }
}
