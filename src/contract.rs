use soroban_sdk::{contract, contractimpl, panic_with_error, Address, BytesN, Env, Map, Vec};

use crate::{
    colors::{color_balance, colors_mine, colors_transfer},
    glyphs::{glyph_get, glyph_mint, glyph_scrape, glyph_transfer},
    interface::ColorGlyphTrait,
    offers::{offer_delete, offer_post, offers_get},
    types::{Error, GlyphType, HashType, Offer, OfferType, StorageKey},
};

pub const MAX_ENTRY_LIFETIME: u32 = 6_312_000; // A year's worth of ledgers

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

        env.storage().instance().bump(MAX_ENTRY_LIFETIME);
    }

    // Colors
    fn colors_mine(env: Env, miner: Address, to: Option<Address>, colors: Map<u32, u32>) {
        colors_mine(&env, miner, to, colors)
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
    fn glyph_transfer(env: Env, from: Address, to: Address, hash: Option<BytesN<32>>) {
        glyph_transfer(&env, from, to, hash)
    }
    fn glyph_scrape(env: Env, owner: Address, to: Option<Address>, hash_type: HashType) {
        glyph_scrape(&env, owner, to, &hash_type)
    }
    fn glyph_get(
        env: Env,
        address: Option<Address>,
        hash_type: HashType,
    ) -> Result<GlyphType, Error> {
        glyph_get(&env, address, hash_type)
    }

    // Offers
    fn offer_post(env: Env, seller: Address, sell: OfferType, buy: OfferType) -> Result<(), Error> {
        offer_post(&env, seller, sell, buy)
    }
    fn offer_delete(env: Env, seller: Address, sell: OfferType, buy: Option<OfferType>) {
        offer_delete(&env, seller, sell, &buy)
    }
    fn offers_get(env: Env, sell: OfferType, buy: Option<OfferType>) -> Result<Offer, Error> {
        offers_get(&env, sell, &buy)
    }
}
