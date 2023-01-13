use soroban_auth::{Signature, Identifier};
use soroban_sdk::{contractimpl, Env, Vec, AccountId, BytesN, Address};

use crate::{
    types::{DataKey, SourceAccount, ColorAmount, Glyph, Error, AssetType, AssetAmount, TradeOwner}, 
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
    pub fn mine(env: Env, signature: Signature, colors: Vec<(u32, u32)>, to: SourceAccount) {
        mine(&env, signature, colors, to);
    }
    pub fn xfer(env: Env, colors: Vec<ColorAmount>, to: SourceAccount) {
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
    pub fn scrape(env: Env, hash: BytesN<32>) {
        scrape(&env, hash);
    }

    // Trades
    pub fn trade(env: Env, buy: AssetType, sell: AssetType) {
        trade(&env, buy, sell);
    }
    pub fn get_trade(env: Env, buy_hash: BytesN<32>, sell_hash: BytesN<32>, amount: i128, side: DataKey) -> TradeOwner {
        get_trade(&env, buy_hash, sell_hash, amount, side)
    }
    pub fn rm_trade(env: Env, buy_hash: BytesN<32>, sell_hash: BytesN<32>, amount: i128, side: DataKey) {
        rm_trade(&env, buy_hash, sell_hash, amount, side);
    }
}