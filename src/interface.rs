use soroban_sdk::{Address, BytesN, Env, Vec};

use crate::types::{Error, Glyph, MinerColorAmount, Offer, OfferType};

pub trait ColorGlyphTrait {
    fn initialize(env: Env, token_id: Address, fee_address: Address);

    // Colors
    fn colors_mine(env: Env, miner: Address, to: Option<Address>, colors: Vec<(u32, u32)>);
    fn colors_transfer(env: Env, from: Address, to: Address, colors: Vec<MinerColorAmount>);
    fn color_balance(env: Env, owner: Address, miner: Option<Address>, color: u32) -> u32;

    // Glyphs
    fn glyph_mint(
        env: Env,
        minter: Address,
        to: Option<Address>,
        colors: Vec<(Address, Vec<(u32, Vec<u32>)>)>,
        width: u32,
    ) -> BytesN<32>;
    fn glyph_get(env: Env, hash: BytesN<32>) -> Result<Glyph, Error>;
    fn glyph_scrape(
        env: Env,
        owner: Address,
        to: Option<Address>,
        hash: BytesN<32>,
    ) -> Result<(), Error>;

    // Offers
    fn offer_post(env: Env, seller: Address, sell: OfferType, buy: OfferType) -> Result<(), Error>;
    fn offers_get(env: Env, sell: OfferType, buy: OfferType) -> Result<Offer, Error>;
    fn offer_delete(env: Env, seller: Address, sell: OfferType, buy: OfferType);
}
