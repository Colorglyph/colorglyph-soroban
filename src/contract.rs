use soroban_sdk::{contract, contractimpl, panic_with_error, Address, BytesN, Env, Map, Vec};

use crate::{
    colors::{color_balance, colors_mine, colors_transfer},
    glyphs::{glyph_get, glyph_mint, glyph_scrape, glyph_transfer},
    interface::ColorGlyphTrait,
    offers::{offer_delete, offer_post, offers_get},
    types::{Error, GlyphType, HashType, Offer, StorageKey},
};

pub const MAX_ENTRY_LIFETIME: u32 = 12 * 60 * 24 * 31 - 12; // A year's worth of ledgers - 12

#[contract]
pub struct ColorGlyph;

#[contractimpl]
impl ColorGlyphTrait for ColorGlyph {
    fn initialize(env: Env, token_address: Address, fee_address: Address) {
        if env.storage().instance().has(&StorageKey::TokenAddress) {
            panic_with_error!(env, Error::NotEmpty);
        }

        env.storage()
            .instance()
            .set(&StorageKey::TokenAddress, &token_address);
        env.storage()
            .instance()
            .set(&StorageKey::FeeAddress, &fee_address);

        env.storage()
            .instance()
            .extend_ttl(MAX_ENTRY_LIFETIME, MAX_ENTRY_LIFETIME);
    }

    // Colors
    fn colors_mine(
        env: Env,
        source: Address,
        miner: Option<Address>,
        to: Option<Address>,
        colors: Map<u32, u32>,
    ) {
        colors_mine(&env, source, miner, to, colors)
    }
    fn colors_transfer(env: Env, from: Address, to: Address, colors: Vec<(Address, u32, u32)>) {
        colors_transfer(&env, from, to, colors)
    }
    fn color_balance(env: Env, owner: Address, miner: Option<Address>, color: u32) -> u32 {
        color_balance(&env, owner, miner, color)
    }

    // Glyphs
    fn glyph_mint(
        env: Env,
        minter: Address,
        to: Option<Address>,
        colors: Map<Address, Map<u32, Vec<u32>>>,
        width: Option<u32>,
    ) -> Option<BytesN<32>> {
        glyph_mint(&env, minter, to, colors, width)
    }
    fn glyph_transfer(env: Env, to: Address, hash_type: HashType) {
        glyph_transfer(&env, to, hash_type)
    }
    fn glyph_scrape(env: Env, to: Option<Address>, hash_type: HashType) {
        glyph_scrape(&env, to, &hash_type)
    }
    fn glyph_get(env: Env, hash_type: HashType) -> Result<GlyphType, Error> {
        glyph_get(&env, hash_type)
    }

    // Offers
    fn offer_post(env: Env, sell: Offer, buy: Offer) -> Result<(), Error> {
        offer_post(&env, sell, buy)
    }
    fn offer_delete(env: Env, sell: Offer, buy: Option<Offer>) -> Result<(), Error> {
        offer_delete(&env, sell, buy)
    }
    fn offers_get(env: Env, sell: Offer, buy: Option<Offer>) -> Result<(), Error> {
        offers_get(&env, sell, buy)
    }
}
