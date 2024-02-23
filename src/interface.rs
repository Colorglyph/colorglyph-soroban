use soroban_sdk::{Address, BytesN, Env, Map, Vec};

use crate::types::{Error, GlyphType, HashType, Offer};

pub trait ColorGlyphTrait {
    fn initialize(env: Env, owner_address: Address, token_address: Address, fee_address: Address);
    fn update(
        env: Env,
        owner_address: Option<Address>,
        token_address: Option<Address>,
        fee_address: Option<Address>,
        max_entry_lifetime: Option<u32>,
        max_payment_count: Option<u32>,
        minter_royalty_rate: Option<i128>,
        miner_royalty_rate: Option<i128>,
    );
    fn upgrade(env: Env, hash: BytesN<32>);

    // Colors
    fn colors_mine(
        env: Env,
        source: Address,
        colors: Map<u32, u32>,
        miner: Option<Address>,
        to: Option<Address>,
    );
    fn colors_transfer(env: Env, from: Address, to: Address, colors: Vec<(Address, u32, u32)>);
    fn color_balance(env: Env, owner: Address, color: u32, miner: Option<Address>) -> u32;

    // Glyphs
    fn glyph_mint(
        env: Env,
        minter: Address,
        to: Option<Address>,
        colors: Map<Address, Map<u32, Vec<u32>>>,
        width: Option<u32>,
    ) -> Option<BytesN<32>>;
    fn glyph_transfer(env: Env, to: Address, hash_type: HashType);
    fn glyph_scrape(env: Env, to: Option<Address>, hash_type: HashType);
    fn glyph_get(env: Env, hash_type: HashType) -> Result<GlyphType, Error>;

    // Offers
    fn offer_post(env: Env, sell: Offer, buy: Offer) -> Result<(), Error>;
    fn offer_delete(env: Env, sell: Offer, buy: Option<Offer>) -> Result<(), Error>;
    fn offers_get(env: Env, sell: Offer, buy: Option<Offer>) -> Result<(), Error>;
}
