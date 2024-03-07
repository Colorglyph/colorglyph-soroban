use soroban_sdk::{contract, contractimpl, panic_with_error, token, Address, BytesN, Env, Map, Vec};

use crate::{
    glyphs::{glyph_get, glyph_mint, glyph_scrape, glyph_transfer}, interface::ColorGlyphTrait, offers::{offer_delete, offer_post, offers_get}, storage::{instance::*, persistent::{read_color, write_color}}, types::{Error, GlyphType, HashType, Offer, StorageKey}
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

        write_owner_address(&env, &owner_address);
        write_token_address(&env, &token_address);
        write_fee_address(&env, &fee_address);
        write_max_entry_lifetime(&env, &max_entry_lifetime);
        write_max_payment_count(&env, &max_payment_count);
        write_minter_royalty_rate(&env, &minter_royalty_rate);
        write_miner_royalty_rate(&env, &miner_royalty_rate);

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
        let owner = read_owner_address(&env);
        owner.require_auth();

        if let Some(owner) = owner_address {
            write_owner_address(&env, &owner)
        }
        if let Some(address) = token_address {
            write_token_address(&env, &address);
        }
        if let Some(address) = fee_address {
            write_fee_address(&env, &address);
        }
        if let Some(lifetime) = max_entry_lifetime {
            write_max_entry_lifetime(&env, &lifetime);
        }
        if let Some(count) = max_payment_count {
            write_max_payment_count(&env, &count);
        }
        if let Some(rate) = minter_royalty_rate {
            write_minter_royalty_rate(&env, &rate);
        }
        if let Some(rate) = miner_royalty_rate {
            write_miner_royalty_rate(&env, &rate);
        }        
    }

    fn upgrade(env: Env, hash: BytesN<32>) {
        let owner = read_owner_address(&env);
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
        source.require_auth();

        let miner = miner.unwrap_or(source.clone());
        let to = to.unwrap_or(source.clone());
        
        let mut pay_amount: u32 = 0;

        for (color, amount) in colors.iter() {
            let current_amount = read_color(&env, miner.clone(), to.clone(), color);
            write_color(&env, miner.clone(), to.clone(), color, current_amount + amount);

            pay_amount += amount;
        }

        crate::events::colors_mine(&env, &miner, &to, colors);

        let token_address = read_token_address(&env);
        let fee_address = read_fee_address(&env);
        
        let token = token::Client::new(&env, &token_address);

        // TODO this is just a stroop fee so not sufficient. This will need to be adjusted before going live
        token.transfer(&source, &fee_address, &(pay_amount as i128));
    }

    fn colors_transfer(env: Env, from: Address, to: Address, colors: Vec<(Address, u32, u32)>) {
        from.require_auth();

        for (miner, color, amount) in colors.iter() {
            let current_from_amount = read_color(&env, miner.clone(), from.clone(), color);
            let current_to_amount = read_color(&env, miner.clone(), to.clone(), color);
            
            if amount > current_from_amount {
                panic_with_error!(env, Error::NotPermitted);
            }

            write_color(&env, miner.clone(), from.clone(), color, current_from_amount - amount);
            write_color(&env, miner.clone(), to.clone(), color, current_to_amount + amount);
        }

        crate::events::colors_transfer(&env, &from, &to, colors);
    }

    fn color_balance(env: Env, owner: Address, color: u32, miner: Option<Address>) -> u32 {
        let miner = miner.unwrap_or(owner.clone());
        
        read_color(&env, miner, owner, color)
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
