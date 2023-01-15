use soroban_auth::{Signature, Identifier};
use soroban_sdk::{contractimpl, Env, Vec, AccountId, BytesN};

use crate::{
    types::{DataKey, MaybeAccountId, ColorAmount, Glyph, Error, AssetType, TradeOwner, MaybeSignature}, 
    colors::{mine, xfer, get_color}, 
    glyphs::{mint, get_glyph, scrape}, 
    trades::{trade, get_trade, rm_trade}
};

pub struct ColorGlyph;

#[contractimpl]
impl ColorGlyph {
    pub fn init(env: Env, token_id: BytesN<32>, fee_identity: Identifier) {
        env
            .storage()
            .set(DataKey::InitToken, token_id);

        env
            .storage()
            .set(DataKey::InitFeeId, fee_identity);
    }

    // Colors
    pub fn mine(env: Env, signature: Signature, colors: Vec<(u32, u32)>, to: MaybeAccountId) {
        mine(&env, signature, colors, to);
    }
    pub fn xfer(env: Env, colors: Vec<ColorAmount>, to: MaybeAccountId) {
        xfer(&env, colors, to);
    }
    pub fn get_color(env: Env, hex: u32, miner: AccountId) -> u32 {
        get_color(&env, hex, miner)
    }

    // Glyphs
    pub fn mint(env: Env, glyph: Glyph) -> BytesN<32> {
        mint(&env, glyph)
    }
    pub fn get_glyph(env: Env, hash: BytesN<32>) -> Result<Glyph, Error> {
        get_glyph(&env, hash)
    }
    pub fn scrape(env: Env, hash: BytesN<32>) -> Result<(), Error> {
        scrape(&env, hash)
    }

    // Trades
    pub fn trade(env: Env, signature: MaybeSignature, buy: AssetType, sell: AssetType) -> Result<(), Error> {
        trade(&env, signature, buy, sell)
    }
    pub fn get_trade(env: Env, buy_hash: BytesN<32>, sell_hash: BytesN<32>, amount: i128, side: DataKey) -> Result<TradeOwner, Error> {
        get_trade(&env, buy_hash, sell_hash, amount, side)
    }
    pub fn rm_trade(env: Env, buy_hash: BytesN<32>, sell_hash: BytesN<32>, amount: i128, side: DataKey) {
        rm_trade(&env, buy_hash, sell_hash, amount, side);
    }
}