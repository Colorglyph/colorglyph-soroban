#![no_std]

use colors::{mine, xfer, get_color};
use glyphs::mint;
use soroban_sdk::{contractimpl, Env, Vec, AccountId, Bytes, BytesN};
use types::{SourceAccount, ColorAmount, Glyph};

mod colors;
mod glyphs;
mod trades;
mod types;
mod colors_test;
mod glyphs_test;

pub struct ColorGlyph;

#[contractimpl]
impl ColorGlyph {
    // Colors
    pub fn mine(env: Env, colors: Vec<(u32, u32)>, to: SourceAccount) {
        mine(&env, colors, to);
    }
    pub fn xfer(env: Env, colors: Vec<ColorAmount>, to: SourceAccount) {
        xfer(&env, colors, to);
    }
    pub fn get_color(env: Env, hex: u32, miner: AccountId) -> u32 {
        get_color(&env, hex, miner)
    }

    // Glyphs
    pub fn mint(env: Env, glyph: Glyph) ->
        // Bytes
        BytesN<32>
    {
        mint(&env, glyph)
    }

    // Trades
}