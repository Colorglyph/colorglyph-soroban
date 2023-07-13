use std::println;
extern crate std;

use crate::types::{Error, Glyph, GlyphType, GlyphTypeArg, MinerOwnerColor, StorageKey};
use soroban_sdk::{panic_with_error, xdr::ToXdr, Address, Bytes, BytesN, Env, Map, Vec};

// TODO
// Limit number of unique miner addresses in a mint `colors` Vec

pub const MAX_PAYMENT_COUNT: u8 = 15;

// TODO use PRNG to generate random ids vs using `env.ledger().timestamp()`
// Then use that id to hold the colors and add an additional store to hold the GlyphBox owner
// Note this really only helps GlyphBox transfers which is likely a rare case so probably not worth it

pub fn glyph_build(
    env: &Env,
    minter: Address,
    colors: Map<Address, Map<u32, Vec<u32>>>,
    mut id: Option<u64>,
) -> u64 {
    minter.require_auth();

    let mut miners_colors_indexes: Map<Address, Map<u32, Vec<u32>>> = Map::new(&env);

    match id {
        None => {
            let mut id_ = env.ledger().timestamp();

            for byte in minter.clone().to_xdr(&env).into_iter() {
                id_ = id_.wrapping_add(byte as u64);
            }

            id = Some(id_);
        }
        Some(id_) => {
            miners_colors_indexes = env
                .storage()
                .get::<StorageKey, Map<Address, Map<u32, Vec<u32>>>>(&StorageKey::GlyphBox(id_))
                .unwrap_or_else(|| panic_with_error!(env, Error::NotFound))
                .unwrap();

            id = Some(id_);
        }
    }

    // spend colors
    for (miner, color_indexes) in colors.iter_unchecked() {
        for (color, indexes) in color_indexes.iter_unchecked() {
            let miner_owner_color = MinerOwnerColor(miner.clone(), minter.clone(), color);
            let current_amount = env
                .storage()
                .get::<MinerOwnerColor, u32>(&miner_owner_color)
                .unwrap_or(Ok(0))
                .unwrap();

            env.storage()
                .set(&miner_owner_color, &(current_amount - indexes.len()));

            match miners_colors_indexes.get(miner.clone()) {
                Some(result) => match result {
                    Ok(mut color_indexes_) => match color_indexes_.get(color) {
                        // Exising miner and color
                        Some(result) => match result {
                            Ok(mut indexes_) => {
                                for index in indexes.iter_unchecked() {
                                    indexes_.push_back(index);
                                }
                                color_indexes_.set(color, indexes_);
                                miners_colors_indexes.set(miner.clone(), color_indexes_);
                            }
                            _ => panic!(),
                        },
                        // No color
                        None => {
                            color_indexes_.set(color, indexes);
                            miners_colors_indexes.set(miner.clone(), color_indexes_);
                        }
                    },
                    _ => panic!(),
                },
                // No miner (or not exisiting glyphbox)
                None => {
                    miners_colors_indexes.set(miner.clone(), color_indexes.clone());
                    break; // we need to break here otherwise we continue looping inside this nested color loop which we've already fully added
                }
            }
        }
    }

    // println!("{:?}", miners_colors_indexes);

    // save glyph
    env.storage()
        .set(&StorageKey::GlyphBox(id.unwrap()), &miners_colors_indexes);

    id.unwrap()
}

pub fn glyph_mint(
    env: &Env,
    minter: Address,
    to: Option<Address>,
    width: u32,
    id: u64,
) -> BytesN<32> {
    minter.require_auth();

    let colors = env
        .storage()
        .get::<StorageKey, Map<Address, Map<u32, Vec<u32>>>>(&StorageKey::GlyphBox(id))
        .unwrap_or_else(|| panic_with_error!(env, Error::NotFound))
        .unwrap();

    let mut hash_data = Bytes::new(&env);

    // TODO
    // better error for not enough colors
    // should we error if there's a dupe index? (will result in burned colors)
    // Need to ensure hash gen is consistent when duping indexes or mixing in white/missing pixels
    // Should we enable some concept of ranging between 2 indexs vs listing out all the indexes? 0..=5 vs 0,1,2,3,4,5

    for (_, color_indexes) in colors.iter_unchecked() {
        for (color, indexes) in color_indexes.iter_unchecked() {
            // TODO
            // This is expensive and it's only for getting the sha256 hash. We should find a cheaper way to derive a hash from the Glyph colors themselves.
            // RawVal maybe?
            // Ordering is important so you can't just hash the arg directly
            // May be able to improve perf by ordering indexes (and maybe reversing them so we extend and then insert vs lots of inserts?)

            for index in indexes.iter_unchecked() {
                // We need to extend the length of the palette
                if (hash_data.len() / 3) <= index {
                    // Start wherever we have data .. wherever we need data
                    for i in (hash_data.len() / 3)..=index {
                        // If this is the section we're interested in filling, just fill
                        if i == index {
                            let slice: [u8; 3] = color.to_le_bytes()[..3].try_into().unwrap();
                            hash_data.insert_from_slice(index * 3, &slice);
                        }
                        // Push empty white pixels
                        // NOTE: this is a "free" way to use white pixels atm
                        else {
                            hash_data.extend_from_slice(&[255; 3]);
                        }
                    }
                }
                // If the bytes already exist just fill them in
                else {
                    let slice: [u8; 3] = color.to_le_bytes()[..3].try_into().unwrap();
                    hash_data.copy_from_slice(index, &slice);
                }
            }
        }
    }

    // NOTE
    // the hash should include something with the width. Otherwise two identical palettes with different widths would clash

    hash_data.extend_from_slice(&width.to_le_bytes());
    let hash = env.crypto().sha256(&hash_data);

    // Glyph has already been minted and is currently owned (not scraped)
    if env.storage().has(&StorageKey::GlyphOwner(hash.clone())) {
        panic_with_error!(env, Error::NotEmpty);
    }

    // Save the glyph owner to storage
    env.storage().set(
        &StorageKey::GlyphOwner(hash.clone()),
        &match to {
            None => minter.clone(),
            Some(address) => address,
        },
    );

    // Save the glyph minter to storage (if glyph hasn't already been minted)
    if !env.storage().has(&StorageKey::GlyphMinter(hash.clone())) {
        env.storage()
            .set(&StorageKey::GlyphMinter(hash.clone()), &minter);
    }

    // Save the glyph to storage
    env.storage().set(
        &StorageKey::Glyph(hash.clone()),
        &Glyph {
            width,
            length: (hash_data.len() - 4) / 3, // -4 because we're appending width, /3 because there are 3 u8 values per u32 color
            colors,
        },
    );

    // Remove the temp GlyphBox
    env.storage().remove(&StorageKey::GlyphBox(id));

    hash
}

// TODO support transfering GlyphBox as well
pub fn glyph_transfer(env: &Env, from: Address, to: Address, hash_id: GlyphTypeArg) {
    from.require_auth();

    match hash_id {
        GlyphTypeArg::Hash(hash) => {
            glyph_verify_ownership(env, from.clone(), hash.clone());

            env.storage()
                .set(&StorageKey::GlyphOwner(hash.clone()), &to);
        }
        GlyphTypeArg::Id(id) => {
            let miners_colors_indexes = env
                .storage()
                .get::<StorageKey, Map<Address, Map<u32, Vec<u32>>>>(&StorageKey::GlyphBox(
                    id.clone(),
                ))
                .unwrap_or_else(|| panic_with_error!(env, Error::NotFound))
                .unwrap();

            env.storage().remove(&StorageKey::GlyphBox(id));

            env.storage()
                .set(&StorageKey::GlyphBox(id.clone()), &miners_colors_indexes);
        }
    }
}

pub fn glyph_scrape(
    env: &Env,
    owner: Address,
    to: Option<Address>,
    hash_id: GlyphTypeArg,
) -> Option<u64> {
    owner.require_auth();

    let mut miners_colors_indexes: Map<Address, Map<u32, Vec<u32>>>;
    let mut id_u64: u64;
    let id: Option<u64>;

    match &hash_id {
        GlyphTypeArg::Hash(hash) => {
            glyph_verify_ownership(env, owner.clone(), hash.clone());

            let glyph = env
                .storage()
                .get::<StorageKey, Glyph>(&StorageKey::Glyph(hash.clone()))
                .unwrap_or_else(|| panic_with_error!(env, Error::NotFound))
                .unwrap();

            // Remove glyph
            env.storage().remove(&StorageKey::Glyph(hash.clone()));

            // Remove glyph owner
            env.storage().remove(&StorageKey::GlyphOwner(hash.clone()));

            // Remove all glyph sell offers
            env.storage().remove(&StorageKey::GlyphOffer(hash.clone()));

            miners_colors_indexes = glyph.colors;

            id_u64 = env.ledger().timestamp();

            for byte in owner.clone().to_xdr(&env).into_iter() {
                id_u64 = id_u64.wrapping_add(byte as u64);
            }
        }
        GlyphTypeArg::Id(id_) => {
            miners_colors_indexes = env
                .storage()
                .get::<StorageKey, Map<Address, Map<u32, Vec<u32>>>>(&StorageKey::GlyphBox(
                    id_.clone(),
                ))
                .unwrap_or_else(|| panic_with_error!(env, Error::NotFound))
                .unwrap();

            id_u64 = id_.clone();
        }
    }

    // loop through the glyph colors and send them to `to`
    let mut payment_count: u8 = 0;

    for (miner, mut colors_indexes) in miners_colors_indexes.iter_unchecked() {
        if payment_count >= MAX_PAYMENT_COUNT {
            break;
        }

        for (color, indexes) in colors_indexes.iter_unchecked() {
            if payment_count >= MAX_PAYMENT_COUNT {
                break;
            }

            let miner_owner_color = MinerOwnerColor(
                miner.clone(),
                match to.clone() {
                    None => owner.clone(),
                    Some(address) => address,
                },
                color,
            );
            let current_amount = env
                .storage()
                .get::<MinerOwnerColor, u32>(&miner_owner_color)
                .unwrap_or(Ok(0))
                .unwrap();

            env.storage()
                .set(&miner_owner_color, &(current_amount + indexes.len()));

            colors_indexes.remove(color);

            payment_count += 1;
        }

        if colors_indexes.len() == 0 {
            miners_colors_indexes.remove(miner);
        } else {
            miners_colors_indexes.set(miner, colors_indexes);
        }
    }

    if miners_colors_indexes.len() == 0 {
        match &hash_id {
            GlyphTypeArg::Id(id) => {
                env.storage().remove(&StorageKey::GlyphBox(id.clone()));
            }
            _ => {}
        }

        id = None;
    } else {
        // save glyph
        env.storage()
            .set(&StorageKey::GlyphBox(id_u64), &miners_colors_indexes);

        id = Some(id_u64);
    }

    id
}

pub fn glyph_get(env: &Env, hash_id: GlyphTypeArg) -> Result<GlyphType, Error> {
    match hash_id {
        GlyphTypeArg::Hash(hash) => {
            let glyph = env
                .storage()
                .get::<StorageKey, Glyph>(&StorageKey::Glyph(hash))
                .unwrap_or_else(|| panic_with_error!(env, Error::NotFound))
                .unwrap();

            Ok(GlyphType::Glyph(glyph))
        }
        GlyphTypeArg::Id(id) => {
            let glyph = env
                .storage()
                .get::<StorageKey, Map<Address, Map<u32, Vec<u32>>>>(&StorageKey::GlyphBox(id))
                .unwrap_or_else(|| panic_with_error!(env, Error::NotFound))
                .unwrap();

            Ok(GlyphType::GlyphBox(glyph))
        }
    }
}

pub fn glyph_verify_ownership(env: &Env, from: Address, glyph_hash: BytesN<32>) {
    let glyph_owner = env
        .storage()
        .get::<StorageKey, Address>(&StorageKey::GlyphOwner(glyph_hash))
        .unwrap_or_else(|| panic_with_error!(env, Error::NotFound))
        .unwrap();

    if glyph_owner != from {
        panic_with_error!(env, Error::NotAuthorized);
    }
}
