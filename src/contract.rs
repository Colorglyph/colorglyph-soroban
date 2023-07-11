use soroban_sdk::{contractimpl, panic_with_error, Address, BytesN, Env, Vec};

use crate::{
    colors::{colors_mine, colors_transfer},
    glyphs::{glyph_mint, glyph_scrape},
    interface::ColorGlyphTrait,
    offers::{offer_delete, offer_post},
    types::{Error, MinerColorAmount, OfferType, StorageKey},
};

pub struct ColorGlyph;

// TODO
// Fine tooth comb what functions actually need to be public. In many cases events are the better way to track data and state

#[contractimpl]
impl ColorGlyphTrait for ColorGlyph {
    fn initialize(env: Env, token_id: Address, fee_address: Address) {
        if env.storage().has(&StorageKey::TokenAddress) {
            panic_with_error!(env, Error::NotEmpty);
        }

        env.storage().set(&StorageKey::TokenAddress, &token_id);
        env.storage().set(&StorageKey::FeeAddress, &fee_address);
    }

    // Colors
    fn colors_mine(env: Env, miner: Address, to: Option<Address>, colors: Vec<(u32, u32)>) {
        colors_mine(&env, miner, to, colors)
    }
    fn colors_transfer(env: Env, from: Address, to: Address, colors: Vec<MinerColorAmount>) {
        colors_transfer(&env, from, to, colors)
    }

    // Glyphs
    fn glyph_mint(
        env: Env,
        minter: Address,
        to: Option<Address>,
        colors: Vec<(Address, Vec<(u32, Vec<u32>)>)>,
        width: u32,
        hash: Option<BytesN<32>>,
        mint: bool,
    ) -> BytesN<32> {
        glyph_mint(&env, minter, to, colors, width, hash, mint)
    }
    fn glyph_scrape(env: Env, owner: Address, to: Option<Address>, hash: BytesN<32>) {
        glyph_scrape(&env, owner, to, hash)
    }

    // Offers
    fn offer_post(env: Env, seller: Address, sell: OfferType, buy: OfferType) -> Result<(), Error> {
        offer_post(&env, seller, &sell, &buy)
    }
    fn offer_delete(env: Env, seller: Address, sell: OfferType, buy: OfferType) {
        offer_delete(&env, seller, &sell, &buy)
    }
}
