use soroban_sdk::{panic_with_error, Address, Bytes, BytesN, Env, Vec};

use crate::{
    colors::adjust,
    types::{Error, Glyph, MinerColorAmount, StorageKey},
};

// const GLYPHS: Symbol = symbol!("GLYPHS");

// TODO:
// Limit number of miners

pub fn mint(
    env: &Env,
    from: Address,
    width: u32,
    colors: Vec<(Address, Vec<(u32, Vec<u32>)>)>,
) -> BytesN<32> {
    from.require_auth();

    let mut b_palette = Bytes::new(&env);
    let mut m_palette: Vec<MinerColorAmount> = Vec::new(&env);

    // TODO: event

    for (miner_address, colors_indexes) in colors.iter_unchecked() {
        for (hex, indexes) in colors_indexes.iter_unchecked() {
            m_palette.push_back(MinerColorAmount(miner_address.clone(), hex, indexes.len()));

            // TODO:
            // This is expensive and it's only for getting the sha256 hash. We should find a cheaper way to derive a hash from the Glyph colors themselves.
            // RawVal maybe?
            // Ordering is important so you can't just hash the arg directly
            // May be able to improve perf by ordering indexes (and maybe reversing them so we extend and then insert vs lots of inserts?)

            for index in indexes.iter_unchecked() {
                // We need to extend the length of the palette
                if (b_palette.len() / 3) <= index {
                    // Start wherever we have data .. wherever we need data
                    for i in (b_palette.len() / 3)..=index {
                        // If this is the section we're interested in filling, just fill
                        if i == index {
                            b_palette.insert_from_array(index * 3, &hex_to_rgb(hex));
                        }
                        // Push empty white pixels
                        // NOTE: this is a "free" way to use white pixels atm
                        else {
                            b_palette.extend_from_array(&[255; 3]);
                        }
                    }
                }
                // If the bytes already exist just fill them in
                else {
                    let [r, g, b] = hex_to_rgb(hex);

                    b_palette.set(index * 3, r);
                    b_palette.set(index * 3 + 1, g);
                    b_palette.set(index * 3 + 2, b);
                }
            }
        }
    }

    // TODO should the hash also include something with the width? Otherwise two identical palettes with different widths would clash

    let hash = env.crypto().sha256(&b_palette);

    // made
    // owner
    // minter
    // exists
    // scraped
    // no owner
    // minter
    // not exists
    // new
    // no owner
    // no minter
    // not exists

    let is_owned = env.storage().has(&StorageKey::GlyphOwner(hash.clone()));

    if is_owned {
        panic_with_error!(env, Error::NotEmpty);
    } else {
        // Save the glyph to storage {glyph hash: Glyph}
        env.storage().set(
            &StorageKey::Glyph(hash.clone()),
            &Glyph {
                width,
                length: b_palette.len() / 3, // because there are 3 values per color
                colors,
            },
        );

        // Save the glyph owner to storage {glyph hash: Address}
        env.storage()
            .set(&StorageKey::GlyphOwner(hash.clone()), &from);
    }

    let is_made = env.storage().has(&StorageKey::GlyphMinter(hash.clone()));

    if !is_made {
        // Save the glyph minter to storage {glyph hash: Address}
        env.storage()
            .set(&StorageKey::GlyphMinter(hash.clone()), &from);
    }

    // Remove the colors from the owner
    adjust(&env, &from, &m_palette, false);

    hash
}

pub fn get_glyph(env: &Env, hash: BytesN<32>) -> Result<Glyph, Error> {
    env.storage()
        .get(&StorageKey::Glyph(hash.clone()))
        .ok_or(Error::NotFound)?
        .unwrap()
}

// TODO: transfer glyph fn

pub fn scrape(env: &Env, from: Address, hash: BytesN<32>) -> Result<(), Error> {
    from.require_auth();

    // TODO: event

    let owner: Address = env
        .storage()
        .get(&StorageKey::GlyphOwner(hash.clone()))
        .unwrap_or_else(|| panic_with_error!(env, Error::NotFound))
        .unwrap();

    if owner != from {
        panic_with_error!(env, Error::NotAuthorized);
    }

    let glyph = get_glyph(&env, hash.clone()).unwrap();
    let mut m_palette: Vec<MinerColorAmount> = Vec::new(&env); // [Color(hex, miner), amount]

    for (miner_address, colors_indexes) in glyph.colors.iter_unchecked() {
        for (hex, indexes) in colors_indexes.iter_unchecked() {
            m_palette.push_back(MinerColorAmount(miner_address.clone(), hex, indexes.len()));
        }
    }

    // Add the colors to the owner
    adjust(&env, &from, &m_palette, true);

    // Remove glyph
    env.storage().remove(&StorageKey::Glyph(hash.clone()));

    // Remove glyph owner
    env.storage().remove(&StorageKey::GlyphOwner(hash.clone()));

    Ok(())
}

fn hex_to_rgb(hex: u32) -> [u8; 3] {
    let a: [u8; 4] = hex.to_le_bytes();
    let mut b = [0; 3];

    b.copy_from_slice(&a[..3]);

    b
}
