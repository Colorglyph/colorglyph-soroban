use soroban_sdk::{contractimpl, panic_with_error, Address, Env, Map, Vec, contract};

use crate::{
    colors::{color_balance, colors_mine, colors_transfer},
    glyphs::{glyph_get, glyph_mint, glyph_scrape, glyph_transfer},
    interface::ColorGlyphTrait,
    offers::{offer_delete, offer_post, offers_get},
    types::{Error, GlyphType, HashId, Offer, OfferType, StorageKey},
};

#[contract]
pub struct ColorGlyph;

// TODO
// Fine tooth comb what functions actually need to be public. In many cases events are the better way to track data and state

#[contractimpl]
impl ColorGlyphTrait for ColorGlyph {
    fn initialize(env: Env, token_id: Address, fee_address: Address) {
        if env.storage().persistent().has(&StorageKey::TokenAddress) {
            panic_with_error!(env, Error::NotEmpty);
        }

        env.storage().persistent().set(&StorageKey::TokenAddress, &token_id);
        env.storage().persistent().set(&StorageKey::FeeAddress, &fee_address);
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
        colors: Option<Map<Address, Map<u32, Vec<u32>>>>,
        width: Option<u32>,
        id: Option<u64>,
    ) -> HashId {
        glyph_mint(&env, minter, to, colors, width, id)
    }
    fn glyph_transfer(env: Env, from: Address, to: Address, hash_id: HashId) {
        glyph_transfer(&env, from, to, hash_id)
    }
    fn glyph_scrape(env: Env, owner: Address, to: Option<Address>, hash_id: HashId) -> Option<u64> {
        glyph_scrape(&env, owner, to, hash_id)
    }
    fn glyph_get(env: Env, hash_id: HashId) -> Result<GlyphType, Error> {
        glyph_get(&env, hash_id)
    }

    // Offers
    fn offer_post(env: Env, seller: Address, sell: OfferType, buy: OfferType) -> Result<(), Error> {
        offer_post(&env, seller, sell, buy)
    }
    fn offer_delete(env: Env, seller: Address, sell: OfferType, buy: OfferType) {
        offer_delete(&env, seller, sell, buy)
    }
    fn offers_get(env: Env, sell: OfferType, buy: OfferType) -> Result<Offer, Error> {
        offers_get(&env, sell, buy)
    }
}
