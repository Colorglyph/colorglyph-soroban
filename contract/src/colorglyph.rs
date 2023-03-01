use soroban_sdk::{contractimpl, Address, BytesN, Env, Vec};

use crate::{
    colors::{get_color, mine, xfer},
    glyphs::{get_glyph, make, scrape},
    offers::{get_offer, offer, rm_offer},
    types::{Error, Glyph, MaybeAddress, MinerColorAmount, Offer, OfferType, StorageKey},
};

pub struct ColorGlyph;

// TODO:
// Fine tooth comb what functions actually need to be public

#[contractimpl]
impl ColorGlyph {
    pub fn init(env: Env, token_id: BytesN<32>, fee_identity: Address) {
        // TODO only allow init once
        env.storage().set(&StorageKey::InitToken, &token_id);
        env.storage().set(&StorageKey::InitFeeId, &fee_identity);
    }

    // Colors
    pub fn mine(env: Env, from: Address, colors: Vec<(u32, u32)>, to: MaybeAddress) {
        mine(&env, from, colors, to);
    }
    pub fn xfer(env: Env, from: Address, colors: Vec<MinerColorAmount>, to: MaybeAddress) {
        xfer(&env, from, colors, to);
    }
    pub fn get_color(env: Env, from: Address, hex: u32, miner: Address) -> u32 {
        get_color(&env, from, hex, miner)
    }

    // Glyphs
    pub fn make(
        env: Env,
        from: Address,
        width: u32,
        colors: Vec<(Address, Vec<(u32, Vec<u32>)>)>,
    ) -> BytesN<32> {
        make(&env, from, width, colors)
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
