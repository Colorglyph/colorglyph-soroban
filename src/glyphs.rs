// use std::println;
// extern crate std;

use crate::{
    contract::MAX_ENTRY_LIFETIME,
    types::{Error, Glyph, GlyphType, HashType, StorageKey},
};
use soroban_sdk::{panic_with_error, symbol_short, Address, Bytes, BytesN, Env, Map, Symbol, Vec};

pub const MAX_PAYMENT_COUNT: u8 = 15;

// TODO
// Limit number of unique miner addresses in a `glyph_mint`
// Fine tune MAX_PAYMENT_COUNT number

/* BLOCKED
Use PRNG to generate random ids
    Then use that id to hold the Colors and add an additional store to hold the Colors owner
    Note this really only helps Colors transfers and gets which is likely a rare case so probably not worth it
*/

pub fn glyph_mint(
    env: &Env,
    minter: Address,
    to: Option<Address>,
    colors: Map<Address, Map<u32, Vec<u32>>>,
    width: Option<u32>,
) -> Option<BytesN<32>> {
    minter.require_auth();

    let glyph_colors_key = StorageKey::Colors(minter.clone());
    let mut glyph_colors = env
        .storage()
        .persistent()
        .get::<StorageKey, Map<Address, Map<u32, Vec<u32>>>>(&glyph_colors_key)
        .unwrap_or(Map::new(&env));

    // No need to bump here as we'll bump later if/when we store the update Colors Map

    // spend colors
    for (miner, color_indexes) in colors.iter() {
        let mut skip = false;

        for (color, indexes) in color_indexes.iter() {
            let current_color_key = StorageKey::Color(miner.clone(), minter.clone(), color);
            let current_color_amount = env
                .storage()
                .persistent()
                .get::<StorageKey, u32>(&current_color_key)
                .unwrap_or(0);

            env.storage()
                .persistent()
                .set(&current_color_key, &(current_color_amount - indexes.len()));

            env.storage()
                .persistent()
                .bump(&current_color_key, MAX_ENTRY_LIFETIME);

            if !skip {
                match glyph_colors.get(miner.clone()) {
                    Some(result) => match result {
                        mut color_indexes_ => match color_indexes_.get(color) {
                            // Exising miner and color
                            Some(result) => match result {
                                mut indexes_ => {
                                    indexes_.append(&indexes);
                                    color_indexes_.set(color, indexes_);
                                    glyph_colors.set(miner.clone(), color_indexes_);
                                }
                            },
                            // Existing miner no color
                            None => {
                                color_indexes_.set(color, indexes);
                                glyph_colors.set(miner.clone(), color_indexes_);
                            }
                        },
                    },
                    // No miner (or no exisiting Colors)
                    None => {
                        glyph_colors.set(miner.clone(), color_indexes.clone());
                        // We set a skip vs using break to ensure we continue to bill for the spent colors
                        skip = true; // we need to break here otherwise we continue looping inside this nested color loop which we've already fully added
                    }
                }
            }

            // TODO do we need events for each color spent?
        }
    }

    match width {
        // We are storing the glyph
        Some(width) => {
            let hash = glyph_store(env, minter.clone(), to.clone(), glyph_colors, width);

            env.events()
                .publish((symbol_short!("minted"), minter, to), hash.clone());

            Some(hash)
        }
        // We are building the glyph
        None => {
            env.storage()
                .persistent()
                .set(&glyph_colors_key, &glyph_colors);

            env.storage()
                .persistent()
                .bump(&glyph_colors_key, MAX_ENTRY_LIFETIME);

            env.events().publish((symbol_short!("minting"), minter), ());

            None
        }
    }
}

fn glyph_store(
    env: &Env,
    minter: Address,
    to: Option<Address>,
    colors: Map<Address, Map<u32, Vec<u32>>>,
    width: u32,
) -> BytesN<32> {
    let mut bit24_data = Vec::new(&env);

    // TODO
    // Better error for not enough colors
    // Should we error if there's a dupe index?
    // Should we enable some concept of ranging between 2 indexs vs listing out all the indexes? 0..=5 vs 0,1,2,3,4,5
    for (_, color_indexes) in colors.iter() {
        for (color, indexes) in color_indexes.iter() {
            for index in indexes.iter() {
                // fill between the gap with white pixels
                if bit24_data.len() <= index {
                    // Start wherever we have data..=wherever we need data
                    for i in bit24_data.len()..=index {
                        bit24_data.push_back(if i == index {
                            // If this is the tail of the array fill it with the color
                            color
                        } else {
                            // Push empty white pixels
                            // NOTE: this is a "free" way to use white pixels atm
                            16777215
                        });
                    }
                }
                // If the bytes already exist just fill them in
                else {
                    bit24_data.set(index, color);
                }
            }
        }
    }

    let mut hash_data = Bytes::new(&env);

    for color in bit24_data.iter() {
        hash_data.extend_from_slice(&color.to_be_bytes()[1..]);
    }

    // the hash includes the width. Otherwise two identical palettes with different widths would clash
    hash_data.extend_from_slice(&width.to_be_bytes());

    let hash = env.crypto().sha256(&hash_data);
    let glyph_owner_key = StorageKey::GlyphOwner(hash.clone());

    // Glyph has already been minted and is currently owned (not scraped)
    if env.storage().persistent().has(&glyph_owner_key) {
        panic_with_error!(env, Error::NotEmpty);
    }

    // Save the glyph owner to storage
    env.storage().persistent().set(
        &glyph_owner_key,
        &match to {
            Some(address) => address,
            None => minter.clone(),
        },
    );

    env.storage()
        .persistent()
        .bump(&glyph_owner_key, MAX_ENTRY_LIFETIME);

    // Save the glyph minter to storage (if glyph hasn't already been minted)
    let glyph_minter_key = StorageKey::GlyphMinter(hash.clone());

    if !env.storage().persistent().has(&glyph_minter_key) {
        env.storage().persistent().set(&glyph_minter_key, &minter);
    }

    env.storage()
        .persistent()
        .bump(&glyph_minter_key, MAX_ENTRY_LIFETIME);

    // Save the glyph to storage
    let glyph_key = StorageKey::Glyph(hash.clone());

    // Only save the glyph if it hasn't already been minted
    if !env.storage().persistent().has(&glyph_key) {
        env.storage().persistent().set(
            &glyph_key,
            &Glyph {
                width,
                length: bit24_data.len(),
                colors,
            },
        );
    }

    env.storage()
        .persistent()
        .bump(&glyph_key, MAX_ENTRY_LIFETIME);

    // Remove any temp Colors
    env.storage()
        .persistent()
        .remove(&StorageKey::Colors(minter));

    hash
}

pub fn glyph_transfer(env: &Env, to: Address, hash_type: HashType) {
    match hash_type {
        HashType::Colors(from) => {
            glyph_transfer_color_dust(
                env,
                from.clone(),
                StorageKey::Colors(from.clone()),
                StorageKey::Colors(to.clone()),
            );

            env.events()
                .publish((Symbol::new(env, "transfer_colors"), from, to), ());
        }
        HashType::Dust(from) => {
            glyph_transfer_color_dust(
                env,
                from.clone(),
                StorageKey::Dust(from.clone()),
                StorageKey::Dust(to.clone()),
            );

            env.events()
                .publish((Symbol::new(env, "transfer_dust"), from, to), ());
        }
        HashType::Glyph(glyph_hash) => {
            let glyph_owner_key = StorageKey::GlyphOwner(glyph_hash.clone());

            glyph_verify_ownership(env, &glyph_owner_key);

            env.storage().persistent().set(&glyph_owner_key, &to);

            env.storage()
                .persistent()
                .bump(&glyph_owner_key, MAX_ENTRY_LIFETIME);

            env.events().publish(
                (
                    Symbol::new(env, "transfer_glyph"),
                    glyph_owner_key,
                    to,
                    glyph_hash,
                ),
                (),
            );
        }
    }
}

fn glyph_transfer_color_dust(
    env: &Env,
    from: Address,
    from_colors_key: StorageKey,
    to_colors_key: StorageKey,
) {
    from.require_auth();

    let colors = env
        .storage()
        .persistent()
        .get::<StorageKey, Map<Address, Map<u32, Vec<u32>>>>(&from_colors_key)
        .unwrap_or_else(|| panic_with_error!(env, Error::NotFound));

    // TODO
    // This is a pretty expensive transfer. Separating StorageKey::Colors from maybe a StorageKey::ColorsOwner might be the better way to go
    // On the other hand this is a pretty rare case so maybe it's not worth it

    env.storage().persistent().remove(&from_colors_key);

    env.storage().persistent().set(&to_colors_key, &colors);

    env.storage()
        .persistent()
        .bump(&to_colors_key, MAX_ENTRY_LIFETIME);
}

pub fn glyph_scrape(env: &Env, to: Option<Address>, hash_type: &HashType) {
    let owner: Address;
    let mut miners_colors_indexes: Map<Address, Map<u32, Vec<u32>>> = Map::new(&env);

    match hash_type {
        HashType::Colors(owner_) => {
            owner = glyph_scrape_color_dust(
                env,
                owner_,
                StorageKey::Colors(owner_.clone()),
                &mut miners_colors_indexes,
            );

            env.events().publish(
                (Symbol::new(env, "scrape_colors"), owner.clone(), to.clone()),
                (),
            );
        }
        HashType::Dust(owner_) => {
            owner = glyph_scrape_color_dust(
                env,
                owner_,
                StorageKey::Dust(owner_.clone()),
                &mut miners_colors_indexes,
            );

            env.events().publish(
                (Symbol::new(env, "scrape_dust"), owner.clone(), to.clone()),
                (),
            );
        }
        HashType::Glyph(glyph_hash) => {
            let glyph_owner_key = StorageKey::GlyphOwner(glyph_hash.clone());
            let glyph_owner = glyph_verify_ownership(env, &glyph_owner_key);

            // Ensure we don't start a new scrape while there's a pending scrape, otherwise we'll overwrite the pending with the new
            // We use the Address vs the BytesN<32> as the key in order to maintain ownership of the Dust Colors
            // If we wanted to support scraping multiple glyphs at once we'd need to track ownership another way
            if env
                .storage()
                .persistent()
                .has(&StorageKey::Dust(glyph_owner.clone()))
            {
                panic_with_error!(env, Error::NotEmpty);
            }

            let glyph_key = StorageKey::Glyph(glyph_hash.clone());
            let glyph = env
                .storage()
                .persistent()
                .get::<StorageKey, Glyph>(&glyph_key)
                .unwrap_or_else(|| panic_with_error!(env, Error::NotFound));

            // Remove glyph owner
            env.storage()
                .persistent()
                .remove(&StorageKey::GlyphOwner(glyph_hash.clone()));

            // Remove all glyph sell offers
            env.storage()
                .persistent()
                .remove(&StorageKey::GlyphOffer(glyph_hash.clone()));

            miners_colors_indexes = glyph.colors;
            owner = glyph_owner.clone();

            env.events().publish(
                (
                    Symbol::new(env, "scrape_glyph"),
                    glyph_owner,
                    to.clone(),
                    glyph_hash.clone(),
                ),
                (),
            );
        }
    }

    // loop through the glyph colors and send them to `to`
    let mut payment_count: u8 = 0;
    let to_address = match to.clone() {
        Some(address) => address,
        None => owner.clone(),
    };

    for (miner, mut colors_indexes) in miners_colors_indexes.iter() {
        if payment_count >= MAX_PAYMENT_COUNT {
            break;
        }

        for (color, indexes) in colors_indexes.iter() {
            // TODO do we need to dupe this line with the above?
            if payment_count >= MAX_PAYMENT_COUNT {
                break;
            }

            let miner_owner_color = StorageKey::Color(miner.clone(), to_address.clone(), color);
            let current_amount = env
                .storage()
                .persistent()
                .get::<StorageKey, u32>(&miner_owner_color)
                .unwrap_or(0);

            env.storage()
                .persistent()
                .set(&miner_owner_color, &(current_amount + indexes.len()));

            env.storage()
                .persistent()
                .bump(&miner_owner_color, MAX_ENTRY_LIFETIME);

            colors_indexes.remove(color);

            payment_count += 1;

            // TODO do we need to emit events for each color repayment?
        }

        if colors_indexes.is_empty() {
            miners_colors_indexes.remove(miner);
        } else {
            miners_colors_indexes.set(miner, colors_indexes);
        }
    }

    // We have two different partial scrape scenarios in order to not overwrite partial mints when scraping a minted glyph
    match hash_type {
        HashType::Colors(_) => {
            let colors_key = StorageKey::Colors(owner);

            if miners_colors_indexes.is_empty() {
                env.storage().persistent().remove(&colors_key);
            } else {
                // save glyph
                env.storage()
                    .persistent()
                    .set(&colors_key, &miners_colors_indexes);

                env.storage()
                    .persistent()
                    .bump(&colors_key, MAX_ENTRY_LIFETIME);
            }
        }
        _ => {
            let colors_key = StorageKey::Dust(owner);

            if miners_colors_indexes.is_empty() {
                env.storage().persistent().remove(&colors_key);
            } else {
                // save glyph
                env.storage()
                    .persistent()
                    .set(&colors_key, &miners_colors_indexes);

                env.storage()
                    .persistent()
                    .bump(&colors_key, MAX_ENTRY_LIFETIME);
            }
        }
    }
}

fn glyph_scrape_color_dust(
    env: &Env,
    owner: &Address,
    colors_key: StorageKey,
    miners_colors_indexes: &mut Map<Address, Map<u32, Vec<u32>>>,
) -> Address {
    owner.require_auth();

    *miners_colors_indexes = env
        .storage()
        .persistent()
        .get::<StorageKey, Map<Address, Map<u32, Vec<u32>>>>(&colors_key)
        .unwrap_or_else(|| panic_with_error!(env, Error::NotFound));

    env.storage()
        .persistent()
        .bump(&colors_key, MAX_ENTRY_LIFETIME);

    owner.clone()
}

pub fn glyph_get(env: &Env, hash_type: HashType) -> Result<GlyphType, Error> {
    match hash_type {
        HashType::Colors(address) => glyph_get_color_dust(env, StorageKey::Colors(address)),
        HashType::Dust(address) => glyph_get_color_dust(env, StorageKey::Dust(address)),
        HashType::Glyph(hash) => {
            let glyph_key = StorageKey::Glyph(hash.clone());
            let glyph_owner_key = StorageKey::GlyphOwner(hash.clone());

            if !env.storage().persistent().has(&glyph_owner_key) {
                return Err(Error::NotFound);
            }

            let glyph = env
                .storage()
                .persistent()
                .get::<StorageKey, Glyph>(&glyph_key)
                .ok_or(Error::NotFound)?;

            env.storage()
                .persistent()
                .bump(&glyph_key, MAX_ENTRY_LIFETIME);

            Ok(GlyphType::Glyph(glyph))
        }
    }
}

fn glyph_get_color_dust(env: &Env, colors_key: StorageKey) -> Result<GlyphType, Error> {
    let colors = env
        .storage()
        .persistent()
        .get::<StorageKey, Map<Address, Map<u32, Vec<u32>>>>(&colors_key)
        .ok_or(Error::NotFound)?;

    env.storage()
        .persistent()
        .bump(&colors_key, MAX_ENTRY_LIFETIME);

    Ok(GlyphType::Colors(colors))
}

pub fn glyph_verify_ownership(env: &Env, glyph_owner_key: &StorageKey) -> Address {
    let glyph_owner = env
        .storage()
        .persistent()
        .get::<StorageKey, Address>(glyph_owner_key)
        .unwrap_or_else(|| panic_with_error!(env, Error::NotFound));

    glyph_owner.require_auth();

    env.storage()
        .persistent()
        .bump(glyph_owner_key, MAX_ENTRY_LIFETIME);

    glyph_owner
}
