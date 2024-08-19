use core::ops::Mul;

// extern crate std;

use soroban_sdk::{
    contract, contractimpl, panic_with_error, token, Address, BytesN, Env, Map, Vec,
};

use crate::{
    glyphs::{glyph_store, glyph_verify_ownership},
    interface::{ColorGlyphTrait, ColorsInterface, Exchange, GlyphInterface},
    offers::{offer_delete, offer_post, offers_get},
    storage::{
        instance::*,
        persistent::{
            read_color, read_glyph_or_default, read_glyph_or_error, remove_glyph_offer, write_color,
        },
    },
    types::{Error, Glyph, Offer, StorageKey},
};

pub const MAX_BIT24_SIZE: usize = 40 * 40 * 3 + 1;

#[contract]
pub struct ColorGlyph;

#[contractimpl]
impl ColorGlyphTrait for ColorGlyph {
    fn initialize(
        env: Env,
        owner_address: Address,
        token_address: Address,
        fee_address: Address,
        mine_multiplier: i128,
    ) {
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
        write_mine_multiplier(&env, &mine_multiplier);
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
        mine_multiplier: Option<i128>,
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
        if let Some(rate) = mine_multiplier {
            write_mine_multiplier(&env, &rate);
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
}

#[contractimpl]
impl ColorsInterface for ColorGlyph {
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
            let current_amount = read_color(&env, &miner, &to, color);
            write_color(&env, &miner, &to, color, current_amount + amount);

            pay_amount += amount;
        }

        // crate::events::colors_mine(&env, &miner, &to, colors);

        let token_address = read_token_address(&env);
        let fee_address = read_fee_address(&env);

        let token = token::Client::new(&env, &token_address);
        let mine_multiplier = read_mine_multiplier(&env);

        token.transfer(
            &source,
            &fee_address,
            &(pay_amount as i128).mul(mine_multiplier),
        );
    }

    fn colors_transfer(env: Env, from: Address, to: Address, colors: Vec<(Address, u32, u32)>) {
        from.require_auth();

        for (miner, color, amount) in colors.iter() {
            let current_from_amount = read_color(&env, &miner, &from, color);
            let current_to_amount = read_color(&env, &miner, &to, color);

            if amount > current_from_amount {
                panic_with_error!(env, Error::NotPermitted);
            }

            write_color(&env, &miner, &from, color, current_from_amount - amount);
            write_color(&env, &miner, &to, color, current_to_amount + amount);
        }

        // crate::events::colors_transfer(&env, &from, &to, colors);
    }

    fn color_balance(env: Env, owner: Address, color: u32, miner: Option<Address>) -> u32 {
        let miner = miner.unwrap_or(owner.clone());

        read_color(&env, &miner, &owner, color)
    }
}

#[contractimpl]
impl GlyphInterface for ColorGlyph {
    fn glyph_mint(
        env: Env,
        hash: BytesN<32>,
        minter: Address,
        to: Option<Address>,
        colors: Map<Address, Map<u32, Vec<u32>>>,
        width: Option<u32>,
    ) {
        let mut glyph = read_glyph_or_default(&env, &hash);

        // Only mint if the glyph hasn't yet been minted
        if glyph.width != 0 || glyph.length != 0 {
            panic_with_error!(env, Error::NotEmpty);
        }

        let glyph_owner_key = StorageKey::GlyphOwner(hash.clone());
        let new_owner = match to.clone() {
            Some(address) => address,
            None => minter.clone(),
        };

        // Starting the mint, assign an owner
        if glyph.colors.is_empty() {
            env.storage().persistent().set(&glyph_owner_key, &new_owner);
        } else {
            let existing_owner = glyph_verify_ownership(&env, &glyph_owner_key);

            if existing_owner != new_owner {
                env.storage().persistent().set(&glyph_owner_key, &new_owner);
            }
        }

        // spend colors
        for (miner, color_indexes) in colors.iter() {
            let mut skip = false;

            for (color, indexes) in color_indexes.iter() {
                let current_color_amount = read_color(&env, &miner, &minter, color);
                write_color(
                    &env,
                    &miner,
                    &minter,
                    color,
                    current_color_amount - indexes.len(),
                );

                // crate::events::colors_out(&env, &miner, &minter, color, indexes.len());

                if !skip {
                    match glyph.colors.get(miner.clone()) {
                        Some(result) => match result {
                            mut color_indexes_ => match color_indexes_.get(color) {
                                // Existing miner and color
                                Some(result) => match result {
                                    mut indexes_ => {
                                        indexes_.append(&indexes);
                                        color_indexes_.set(color, indexes_);
                                        glyph.colors.set(miner.clone(), color_indexes_);
                                    }
                                },
                                // Existing miner no color
                                None => {
                                    color_indexes_.set(color, indexes);
                                    glyph.colors.set(miner.clone(), color_indexes_);
                                }
                            },
                        },
                        // No miner (or no exisiting Colors)
                        None => {
                            glyph.colors.set(miner.clone(), color_indexes.clone());
                            // We set a skip vs using break to ensure we continue to bill for the spent colors
                            skip = true; // we need to break here otherwise we continue looping inside this nested color loop which we've already fully added
                        }
                    }
                }
            }
        }

        match width {
            // We are storing the glyph
            Some(width) => {
                let computed_hash = glyph_store(&env, minter.clone(), glyph.colors, width as u8);

                // println!("HASH: {:?}", computed_hash);

                if hash != computed_hash {
                    panic_with_error!(env, Error::NotPermitted);
                }

                crate::events::minted_event(&env, &minter, to, &hash);
            }
            // We are building the glyph
            None => {
                let glyph_key = StorageKey::Glyph(hash.clone());

                env.storage()
                    .persistent()
                    .set::<StorageKey, Glyph>(&glyph_key, &glyph);

                crate::events::minting_event(&env, &minter);
            }
        }
    }
    fn glyph_transfer(env: Env, to: Address, hash: BytesN<32>) {
        let glyph_owner_key = StorageKey::GlyphOwner(hash.clone());

        glyph_verify_ownership(&env, &glyph_owner_key);

        env.storage().persistent().set(&glyph_owner_key, &to);

        crate::events::transfer_glyph_event(&env, &to, &hash);
    }
    fn glyph_scrape(env: Env, to: Option<Address>, hash: BytesN<32>) {
        let owner_key = StorageKey::GlyphOwner(hash.clone());
        let owner = glyph_verify_ownership(&env, &owner_key);

        // Ensure we don't start a scrape while there's a pending mint, otherwise we'll overwrite the pending with the new
        // We use the Address vs the BytesN<32> as the key in order to maintain ownership of the Colors
        // If we wanted to support scraping multiple glyphs at once we'd need to track ownership another way

        let mut glyph = read_glyph_or_error(&env, &hash);

        // Remove all glyph sell offers
        if glyph.width != 0 || glyph.length != 0 {
            remove_glyph_offer(&env, &hash);
        }

        crate::events::scrape_glyph_event(&env, &owner, to.clone(), &hash);

        // loop through the glyph colors and send them to `to`
        let mut payment_count: u8 = 0;
        let to_address = to.unwrap_or(owner.clone());

        let max_payment_count = read_max_payment_count(&env) as u8;

        for (miner, mut colors_indexes) in glyph.colors.iter() {
            if payment_count >= max_payment_count {
                break;
            }

            for (color, indexes) in colors_indexes.iter() {
                // TODO do we need to dupe this line with the above?
                if payment_count >= max_payment_count {
                    break;
                }

                let current_amount = read_color(&env, &miner, &to_address, color);
                write_color(
                    &env,
                    &miner,
                    &to_address,
                    color,
                    current_amount + indexes.len(),
                );

                colors_indexes.remove(color);
                payment_count += 1;

                // crate::events::color_in_event(&env, &miner, &to_address, color, indexes.len());
            }

            if colors_indexes.is_empty() {
                glyph.colors.remove(miner);
            } else {
                glyph.colors.set(miner, colors_indexes);
            }
        }

        let glyph_key = StorageKey::Glyph(hash.clone());

        env.storage().persistent().set::<StorageKey, Glyph>(
            &glyph_key,
            &Glyph {
                width: 0,
                length: 0,
                colors: glyph.colors,
            },
        );

        // NOTE Not sure we actually need to remove the glyph owner as we won't check for it when re-minting if the glyph.colors is empty
        // if glyph.colors.is_empty() {
        //     remove_glyph_owner(&env, &hash);
        // }
    }
    fn glyph_get(env: Env, hash: BytesN<32>) -> Result<Glyph, Error> {
        Ok(read_glyph_or_error(&env, &hash))
    }
}

#[contractimpl]
impl Exchange for ColorGlyph {
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
