use soroban_sdk::{contract, contractimpl, panic_with_error, Address, BytesN, Env, Map, Vec};

use crate::{
    colors::{color_balance, colors_mine, colors_transfer},
    glyphs::{glyph_get, glyph_mint, glyph_scrape, glyph_transfer},
    interface::ColorGlyphTrait,
    offers::{offer_delete, offer_post, offers_get},
    types::{Error, GlyphType, HashType, Offer, StorageKey},
};

pub const MAX_BIT24_SIZE: usize = 40 * 40 * 3 + 1;

#[contract]
pub struct ColorGlyph;

#[contractimpl]
impl ColorGlyphTrait for ColorGlyph {
    fn initialize(env: Env, owner_address: Address, token_address: Address, fee_address: Address) {
        owner_address.require_auth();

        if env.storage().instance().has(&StorageKey::OwnerAddress) {
            panic_with_error!(env, Error::NotEmpty);
        }

        let max_entry_lifetime: u32 = 12 * 60 * 24 * 31 - 1; // A year's worth of ledgers - 12
        let max_payment_count: u32 = 15;
        let minter_royalty_rate: i128 = 3; // 3%
        let miner_royalty_rate: i128 = 2; // 2%

        env.storage()
            .instance()
            .set(&StorageKey::OwnerAddress, &owner_address);
        env.storage()
            .instance()
            .set(&StorageKey::TokenAddress, &token_address);
        env.storage()
            .instance()
            .set(&StorageKey::FeeAddress, &fee_address);
        env.storage()
            .instance()
            .set(&StorageKey::MaxEntryLifetime, &max_entry_lifetime);
        env.storage()
            .instance()
            .set(&StorageKey::MaxPaymentCount, &max_payment_count);
        env.storage()
            .instance()
            .set(&StorageKey::MinterRoyaltyRate, &minter_royalty_rate);
        env.storage()
            .instance()
            .set(&StorageKey::MinerRoyaltyRate, &miner_royalty_rate);

        env.storage()
            .instance()
            .extend_ttl(max_entry_lifetime, max_entry_lifetime);
    }

    fn update(
        env: Env,
        owner_address: Option<Address>,
        token_address: Option<Address>,
        fee_address: Option<Address>,
        max_entry_lifetime: Option<u32>,
        max_payment_count: Option<u32>,
        minter_royalty_rate: Option<i128>,
        miner_royalty_rate: Option<i128>,
    ) {
        let owner = env
            .storage()
            .instance()
            .get::<StorageKey, Address>(&StorageKey::OwnerAddress)
            .unwrap();

        owner.require_auth();

        if owner_address.is_some() {
            env.storage()
                .instance()
                .set(&StorageKey::OwnerAddress, &owner_address.unwrap());
        }
        if token_address.is_some() {
            env.storage()
                .instance()
                .set(&StorageKey::TokenAddress, &token_address.unwrap());
        }
        if fee_address.is_some() {
            env.storage()
                .instance()
                .set(&StorageKey::FeeAddress, &fee_address.unwrap());
        }
        if max_entry_lifetime.is_some() {
            env.storage()
                .instance()
                .set(&StorageKey::MaxEntryLifetime, &max_entry_lifetime.unwrap());
        }
        if max_payment_count.is_some() {
            env.storage()
                .instance()
                .set(&StorageKey::MaxPaymentCount, &max_payment_count.unwrap());
        }
        if minter_royalty_rate.is_some() {
            env.storage().instance().set(
                &StorageKey::MinterRoyaltyRate,
                &minter_royalty_rate.unwrap(),
            );
        }
        if miner_royalty_rate.is_some() {
            env.storage()
                .instance()
                .set(&StorageKey::MinerRoyaltyRate, &miner_royalty_rate.unwrap());
        }
    }

    fn upgrade(env: Env, hash: BytesN<32>) {
        let owner = env
            .storage()
            .instance()
            .get::<StorageKey, Address>(&StorageKey::OwnerAddress)
            .unwrap();

        owner.require_auth();

        env.deployer().update_current_contract_wasm(hash);
    }

    // Colors
    fn colors_mine(
        env: Env,
        source: Address,
        colors: Map<u32, u32>,
        miner: Option<Address>,
        to: Option<Address>,
    ) {
        colors_mine(&env, source, colors, miner, to)
    }
    fn colors_transfer(env: Env, from: Address, to: Address, colors: Vec<(Address, u32, u32)>) {
        colors_transfer(&env, from, to, colors)
    }
    fn color_balance(env: Env, owner: Address, color: u32, miner: Option<Address>) -> u32 {
        color_balance(&env, owner, color, miner)
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
        glyph_scrape(&env, to, hash_type)
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
