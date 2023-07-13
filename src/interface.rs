use soroban_sdk::{Address, BytesN, Env, Map, Vec};

use crate::types::{Error, Offer, OfferType, HashId, GlyphType};

// TODO add back get color, glyph and offer functions due to cross contract call needs

pub trait ColorGlyphTrait {
    fn initialize(env: Env, token_id: Address, fee_address: Address);

    // Colors
    fn colors_mine(env: Env, miner: Address, to: Option<Address>, colors: Map<u32, u32>);
    fn colors_transfer(env: Env, from: Address, to: Address, colors: Vec<(Address, u32, u32)>);
    fn color_balance(env: Env, owner: Address, miner: Option<Address>, color: u32) -> u32;

    // Glyphs
    fn glyph_build(
        env: Env,
        minter: Address,
        colors: Map<Address, Map<u32, Vec<u32>>>,
        id: Option<u64>,
    ) -> u64;
    fn glyph_mint(
        env: Env,
        minter: Address,
        to: Option<Address>,
        width: u32,
        id: u64,
    ) -> BytesN<32>;
    fn glyph_transfer(env: Env, from: Address, to: Address, hash_id: HashId);
    fn glyph_scrape(
        env: Env,
        owner: Address,
        to: Option<Address>,
        hash_id: HashId,
    ) -> Option<u64>;
    fn glyph_get(env: Env, hash_id: HashId) -> Result<GlyphType, Error>;

    // Offers
    fn offer_post(env: Env, seller: Address, sell: OfferType, buy: OfferType) -> Result<(), Error>;
    fn offer_delete(env: Env, seller: Address, sell: OfferType, buy: OfferType);
    fn offers_get(env: Env, sell: OfferType, buy: OfferType) -> Result<Offer, Error>;
}
