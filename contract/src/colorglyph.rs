use soroban_auth::{Signature, Identifier};
use soroban_sdk::{contractimpl, Env, Vec, AccountId, BytesN};

use crate::{
    types::{StorageKey, MaybeAccountId, ColorAmount, Glyph, Error, AssetType, OfferOwner, MaybeSignature, Side}, 
    colors::{mine, xfer, get_color}, 
    glyphs::{make, get_glyph, scrape}, 
    offers::{offer, get_offer, rm_offer}
};

pub struct ColorGlyph;

#[contractimpl]
impl ColorGlyph {
    pub fn init(env: Env, token_id: BytesN<32>, fee_identity: Identifier) {
        env
            .storage()
            .set(StorageKey::InitToken, token_id);

        env
            .storage()
            .set(StorageKey::InitFeeId, fee_identity);
    }

    // Colors
    pub fn mine(env: Env, signature: Signature, colors: Vec<(u32, i128)>, to: MaybeAccountId) {
        mine(&env, signature, colors, to);
    }
    pub fn xfer(env: Env, colors: Vec<ColorAmount>, to: MaybeAccountId) {
        xfer(&env, colors, to);
    }
    pub fn get_color(env: Env, hex: u32, miner: AccountId) -> i128 {
        get_color(&env, hex, miner)
    }

    // Glyphs
    pub fn make(env: Env, glyph: Glyph) -> BytesN<32> {
        make(&env, glyph)
    }
    pub fn get_glyph(env: Env, hash: BytesN<32>) -> Result<Glyph, Error> {
        get_glyph(&env, hash)
    }
    pub fn scrape(env: Env, hash: BytesN<32>) -> Result<(), Error> {
        scrape(&env, hash)
    }

    // Offers
    pub fn offer(env: Env, signature: MaybeSignature, buy: AssetType, sell: AssetType) -> Result<(), Error> {
        offer(&env, signature, buy, sell)
    }
    pub fn get_offer(env: Env, buy_hash: BytesN<32>, sell_hash: BytesN<32>, amount: i128, side: Side) -> Result<OfferOwner, Error> {
        get_offer(&env, buy_hash, sell_hash, amount, side)
    }
    pub fn rm_offer(env: Env, buy_hash: BytesN<32>, sell_hash: BytesN<32>, amount: i128, side: Side) {
        rm_offer(&env, buy_hash, sell_hash, amount, side);
    }
}