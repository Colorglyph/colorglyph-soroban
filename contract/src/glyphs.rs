use soroban_sdk::{symbol, Env, Symbol, Vec, Bytes, BytesN};

use crate::{
    types::{Glyph, Color, ColorAmount, DataKey}, 
    colors::burn
};

const HASH_GLYPH: Symbol = symbol!("HASH_GLYPH");
const GLYPHS: Symbol = symbol!("GLYPHS");

pub fn mint(env: &Env, glyph: Glyph) ->
    // Bytes
    BytesN<32>
{
    let mut b_palette = Bytes::new(&env);
    let mut m_palette: Vec<ColorAmount> = Vec::new(&env); // [Color(hex, miner), amount]

    for (miner_idx, colors_indexes) in glyph.colors.iter_unchecked() {
        for (hex, indexes) in colors_indexes.iter_unchecked() {
            m_palette.push_back(ColorAmount(Color(hex, miner_idx), indexes.len()));

            for index in indexes.iter_unchecked() {
                // We need to extend the length of the palette
                if b_palette.len() <= index {
                    for i in (b_palette.len() / 3)..(index + 1) {
                        // If this is the section we're interested in filling, just fill
                        if i == index {
                            let rga = hex_to_rgb(hex);

                            b_palette.insert_from_array(index * 3, &rga);
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
                    let rga = hex_to_rgb(hex);

                    b_palette.insert_from_array(index * 3, &rga);
                }
            }
        }
    }

    // Remove the colors from the owner
    burn(&env, &m_palette);

    let hash = env.crypto().sha256(&b_palette);

    // Save the glyph to storage {glyph hash: Glyph}
    env
        .storage()
        .set(DataKey::Glyph(hash.clone()), glyph);

    // Save the glyph owner to storage {glyph hash: Address}
    env
        .storage()
        .set(DataKey::GlyOwner(hash.clone()), env.invoker());

    // Save the glyph minter to storage {glyph hash: Address}
    env
        .storage()
        .set(DataKey::GlyMinter(hash.clone()), env.invoker());

    hash

    // TODO save glyph

    // to build out the miner map we need a color array of color:miner
    // the whole point of tracking color miners is to enable them to claim royalties
        // glyph minters will also claim royalties
    // track scraped state
    // take from user palette when minting
    // create a mining function
    // refill a user palette when scraping
    // special mint case when minting a scraped glyph
    // ensure mint uniqueness (no two identical `colors` Bytes arrays)
    
    // let glyph_key = DataKey(symbol!("Glyphs"), count);
    // let glyph_owner_key = DataKey(symbol!("Gl_Owners"), count);
    // let glyph_minter_key = DataKey(symbol!("Gl_Minters"), count);
    // let glyph_miner_key = DataKey(symbol!("Gl_Miners"), count);

    // env.storage().set(glyph_key, glyph);
    // env.storage().set(glyph_owner_key, env.invoker());
    // env.storage().set(glyph_minter_key, env.invoker());
    // env.storage().set(glyph_miner_key, map!(&env, (env.invoker(), 9)));
}

// pub fn get(env: Env, count: u128) -> Vec<RawVal> {
//     let glyph_key = DataKey(symbol!("Glyphs"), count);
//     let glyph_owner_key = DataKey(symbol!("Gl_Owners"), count);
//     let glyph_minter_key = DataKey(symbol!("Gl_Minters"), count);
//     let glyph_miner_key = DataKey(symbol!("Gl_Miners"), count);

//     vec![
//         &env, 

//         env
//         .storage()
//         .get(COUNTER)
//         .unwrap_or_else(|| panic!("noop"))
//         .unwrap(),

//         env
//         .storage()
//         .get(glyph_key)
//         .unwrap_or_else(|| panic!("noop"))
//         .unwrap(),

//         env
//         .storage()
//         .get(glyph_owner_key)
//         .unwrap_or_else(|| panic!("noop"))
//         .unwrap(),

//         env
//         .storage()
//         .get(glyph_minter_key)
//         .unwrap_or_else(|| panic!("noop"))
//         .unwrap(),

//         env
//         .storage()
//         .get(glyph_miner_key)
//         .unwrap_or_else(|| panic!("noop"))
//         .unwrap()
//     ]
// }

fn hex_to_rgb(hex: u32) -> [u8; 3] {
    let a: [u8; 4] = hex.to_le_bytes();
    let mut b = [0; 3];
    
    b.copy_from_slice(&a[..3]);

    b
}