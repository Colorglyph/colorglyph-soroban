use soroban_sdk::{contractimpl, panic_with_error, Address, BytesN, Env, Vec};

use crate::{
    colors::{color_balance, colors_mine, colors_transfer},
    glyphs::{glyph_get, glyph_mint, glyph_scrape},
    interface::ColorGlyphTrait,
    offers::{offer_delete, offer_post, offers_get},
    types::{Error, Glyph, MinerColorAmount, Offer, OfferType, StorageKey},
};

pub struct ColorGlyph;

// TODO
// Fine tooth comb what functions actually need to be public. In many cases events are the better way to track data and state

#[contractimpl]
impl ColorGlyphTrait for ColorGlyph {
    fn initialize(env: Env, token_id: Address, fee_address: Address) {
        if env.storage().has(&StorageKey::InitToken) {
            panic_with_error!(env, Error::NotEmpty);
        }

        env.storage().set(&StorageKey::InitToken, &token_id);
        env.storage().set(&StorageKey::InitFee, &fee_address);
    }

    // Colors
    fn colors_mine(env: Env, miner: Address, to: Option<Address>, colors: Vec<(u32, u32)>) {
        colors_mine(&env, miner, to, colors);
    }
    fn colors_transfer(env: Env, from: Address, to: Address, colors: Vec<MinerColorAmount>) {
        colors_transfer(&env, from, to, colors);
    }
    fn color_balance(env: Env, owner: Address, miner: Option<Address>, color: u32) -> u32 {
        color_balance(&env, owner, miner, color)
    }

    // Glyphs
    fn glyph_mint(
        env: Env,
        minter: Address,
        to: Option<Address>,
        colors: Vec<(Address, Vec<(u32, Vec<u32>)>)>,
        width: u32,
    ) -> BytesN<32> {
        glyph_mint(&env, minter, to, colors, width)
    }
    fn glyph_get(env: Env, hash: BytesN<32>) -> Result<Glyph, Error> {
        glyph_get(&env, hash)
    }
    fn glyph_scrape(
        env: Env,
        owner: Address,
        to: Option<Address>,
        hash: BytesN<32>,
    ) -> Result<(), Error> {
        glyph_scrape(&env, owner, to, hash)
    }

    // Offers
    fn offer_post(env: Env, seller: Address, sell: OfferType, buy: OfferType) -> Result<(), Error> {
        offer_post(&env, seller, &sell, &buy)
    }
    fn offers_get(env: Env, sell: OfferType, buy: OfferType) -> Result<Offer, Error> {
        offers_get(&env, &sell, &buy)
    }
    fn offer_delete(env: Env, seller: Address, sell: OfferType, buy: OfferType) {
        offer_delete(&env, seller, &sell, &buy);
    }
}
