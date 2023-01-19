use soroban_auth::{Identifier, Signature};
use soroban_sdk::{contractimpl, AccountId, BytesN, Env, Vec};

use crate::{
    colors::{get_color, mine, xfer},
    glyphs::{get_glyph, make, scrape},
    offers::{get_offer, offer, rm_offer},
    types::{
        MinerColorAmount, Error, Glyph, MaybeAccountId, MaybeSignature, Offer, OfferType, StorageKey,
    },
};

pub struct ColorGlyph;

// TODO:
// Fine tooth comb what functions actually need to be public

#[contractimpl]
impl ColorGlyph {
    pub fn init(env: Env, token_id: BytesN<32>, fee_identity: Identifier) {
        env.storage().set(StorageKey::InitToken, token_id);
        env.storage().set(StorageKey::InitFeeId, fee_identity);
    }

    // Colors
    pub fn mine(env: Env, signature: Signature, colors: Vec<(u32, u32)>, to: MaybeAccountId) {
        mine(&env, signature, colors, to);
    }
    pub fn xfer(env: Env, colors: Vec<MinerColorAmount>, to: MaybeAccountId) {
        xfer(&env, colors, to);
    }
    pub fn get_color(env: Env, hex: u32, miner: AccountId) -> u32 {
        get_color(&env, hex, miner)
    }

    // Glyphs
    pub fn make(env: Env, width: u32, colors: Vec<(AccountId, Vec<(u32, Vec<u32>)>)>) -> BytesN<32> {
        make(&env, width, colors)
    }
    pub fn get_glyph(env: Env, hash: BytesN<32>) -> Result<Glyph, Error> {
        get_glyph(&env, hash)
    }
    pub fn scrape(env: Env, hash: BytesN<32>) -> Result<(), Error> {
        scrape(&env, hash)
    }

    // Offers
    pub fn offer(
        env: Env,
        signature: MaybeSignature,
        buy: OfferType,
        sell: OfferType,
    ) -> Result<(), Error> {
        offer(&env, &signature, &buy, &sell)
    }
    pub fn get_offer(env: Env, buy: OfferType, sell: OfferType) -> Result<Offer, Error> {
        get_offer(&env, &buy, &sell)
    }
    pub fn rm_offer(env: Env, buy: OfferType, sell: OfferType) {
        rm_offer(&env, &buy, &sell);
    }
}
