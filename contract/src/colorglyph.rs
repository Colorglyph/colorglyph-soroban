use soroban_auth::{Signature, Identifier};
use soroban_sdk::{contractimpl, Env, Vec, AccountId, BytesN};

use crate::{
    types::{DataKey, SourceAccount, ColorAmount, Glyph, Error}, 
    colors::{mine, xfer, get_color}, 
    glyphs::{mint, get_glyph, scrape}
};

pub struct ColorGlyph;

#[contractimpl]
impl ColorGlyph {
    pub fn init(env: Env, token_id: BytesN<32>, fee_identity: Identifier) {
        env
            .storage()
            .set(DataKey::TokenId, token_id);

        env
            .storage()
            .set(DataKey::FeeIden, fee_identity);
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
        get_glyph(&env, &hash)
    }
    pub fn scrape(env: Env, hash: BytesN<32>) {
        scrape(&env, hash);
    }

    // Trades
}