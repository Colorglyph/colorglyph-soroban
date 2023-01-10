use soroban_sdk::{symbol, Env, Symbol, Vec, Bytes, BytesN, Address};

use crate::{
    types::{Glyph, Color, ColorAmount, DataKey}, 
    colors::{adjust}
};

const GLYPHS: Symbol = symbol!("GLYPHS");

pub fn mint(env: &Env, glyph: Glyph) -> BytesN<32> {
    let mut b_palette = Bytes::new(&env);
    let mut m_palette: Vec<ColorAmount> = Vec::new(&env); // [Color(hex, miner), amount]

    // TODO:
        // event

    for (miner_idx, colors_indexes) in glyph.colors.iter_unchecked() {
        for (hex, indexes) in colors_indexes.iter_unchecked() {
            m_palette.push_back(ColorAmount(Color(hex, miner_idx), indexes.len()));

            // TODO: 
                // This is expensive and it's only for getting the sha256 hash. We should find a cheaper way to derive a hash from the Glyph colors themselves. 
                    // RawVal maybe?
                    // Ordering is important so you can't just hash the arg directly
                // May be able to improve perf by ordering indexes (and maybe reversing them so we extend and then insert vs lots of inserts?)

            for index in indexes.iter_unchecked() {
                // We need to extend the length of the palette
                if b_palette.len() <= index {
                    // Start wherever we have data .. wherever we need data
                    for i in (b_palette.len() / 3)..(index + 1) {
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
                    b_palette.insert_from_array(index * 3, &hex_to_rgb(hex));
                }
            }
        }
    }

    let hash = env.crypto().sha256(&b_palette);

    // minted
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

    let is_owned = env
        .storage()
        .has(DataKey::GlyOwner(hash.clone()));

    if is_owned {
        panic!("Not Empty");
    } else {
        // Save the glyph to storage {glyph hash: Glyph}
        env
            .storage()
            .set(DataKey::Glyph(hash.clone()), glyph);

        // Save the glyph owner to storage {glyph hash: Address}
        env
            .storage()
            .set(DataKey::GlyOwner(hash.clone()), env.invoker());
    }

    let is_minted = env
        .storage()
        .has(DataKey::GlyMinter(hash.clone()));

    if !is_minted {
        // Save the glyph minter to storage {glyph hash: Address}
        env
            .storage()
            .set(DataKey::GlyMinter(hash.clone()), env.invoker());
    }

    // Remove the colors from the owner
    adjust(&env, &m_palette, false);

    hash
}

pub fn get_glyph(env: &Env, hash: &BytesN<32>) -> Glyph {
    env
        .storage()
        .get(DataKey::Glyph(hash.clone()))
        .unwrap_or_else(|| panic!("Not Found"))
        .unwrap()
}

pub fn scrape(env: &Env, hash: BytesN<32>) {

    // TODO: 
        // event

    let owner: Address = env
        .storage()
        .get(DataKey::GlyOwner(hash.clone()))
        .unwrap_or_else(|| panic!("Not Found"))
        .unwrap();

    if owner != env.invoker() {
        panic!("Unauthorized");
    }

    let glyph = get_glyph(&env, &hash);
    let mut m_palette: Vec<ColorAmount> = Vec::new(&env); // [Color(hex, miner), amount]

    for (miner_idx, colors_indexes) in glyph.colors.iter_unchecked() {
        for (hex, indexes) in colors_indexes.iter_unchecked() {
            m_palette.push_back(ColorAmount(Color(hex, miner_idx), indexes.len()));
        }
    }

    // Add the colors to the owner
    adjust(&env, &m_palette, true);

    // Remove glyph
    env
        .storage()
        .remove(DataKey::Glyph(hash.clone()));

    // Remove glyph owner
    env
        .storage()
        .remove(DataKey::GlyOwner(hash.clone()));
}

fn hex_to_rgb(hex: u32) -> [u8; 3] {
    let a: [u8; 4] = hex.to_le_bytes();
    let mut b = [0; 3];
    
    b.copy_from_slice(&a[..3]);

    b
}