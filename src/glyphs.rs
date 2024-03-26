// use std::println;
// extern crate std;

use crate::{
    contract::MAX_BIT24_SIZE,
    types::{Error, Glyph, StorageKey},
};
use soroban_sdk::{panic_with_error, Address, Bytes, BytesN, Env, Map, Vec};

pub fn glyph_store(
    env: &Env,
    minter: Address,
    to: Option<Address>,
    colors: Map<Address, Map<u32, Vec<u32>>>,
    width: u8,
) -> BytesN<32> {
    let mut max_i = 0;
    let mut bit24_data = [u8::MAX; MAX_BIT24_SIZE];

    /* TODO
    Better error for not enough colors
    Should we error if there's a dupe index?
    Should we enable some concept of ranging between 2 indexs vs listing out all the indexes? 0..=5 vs 0,1,2,3,4,5
    */
    for (_, color_indexes) in colors.iter() {
        for (color, indexes) in color_indexes.iter() {
            for index in indexes.iter() {
                let i = (index * 3) as usize;

                let [_, r, g, b] = color.to_be_bytes();

                bit24_data[i] = r;
                bit24_data[i + 1] = g;
                bit24_data[i + 2] = b;

                if i + 2 > max_i {
                    max_i = i + 2;
                }
            }
        }
    }

    bit24_data[max_i + 1] = width;

    let bytes = Bytes::from_slice(&env, &bit24_data[..=(max_i + 1)]);

    let hash = env.crypto().sha256(&bytes);
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

    // Save the glyph minter to storage (if glyph hasn't already been minted)
    let glyph_minter_key = StorageKey::GlyphMinter(hash.clone());

    if !env.storage().persistent().has(&glyph_minter_key) {
        env.storage().persistent().set(&glyph_minter_key, &minter);
    }

    // Save the glyph to storage
    let glyph_key = StorageKey::Glyph(hash.clone());

    // Only save the glyph if it hasn't already been minted
    if !env.storage().persistent().has(&glyph_key) {
        env.storage().persistent().set(
            &glyph_key,
            &Glyph {
                width: width as u32,
                length: (bytes.len() - 1) / 3, // remove one byte for the length and divide by 3 for the RGB
                colors,
            },
        );
    }

    // Remove any temp Colors
    env.storage()
        .persistent()
        .remove(&StorageKey::Colors(minter));

    hash
}

pub fn glyph_verify_ownership(env: &Env, glyph_owner_key: &StorageKey) -> Address {
    let glyph_owner = env
        .storage()
        .persistent()
        .get::<StorageKey, Address>(glyph_owner_key)
        .unwrap_or_else(|| panic_with_error!(env, Error::NotFound));

    glyph_owner.require_auth();

    // env.storage()
    //     .persistent()
    //     .bump(glyph_owner_key, MAX_ENTRY_LIFETIME, MAX_ENTRY_LIFETIME);

    glyph_owner
}
