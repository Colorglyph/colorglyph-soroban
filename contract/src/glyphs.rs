use soroban_sdk::{symbol, Env, Symbol, Map, Bytes, BytesN};

use crate::{
    types::{Glyph, Color}, 
    colors::burn
};

const HASH_GLYPH: Symbol = symbol!("HASH_GLYPH");
const GLYPHS: Symbol = symbol!("GLYPHS");

pub fn mint(env: &Env, glyph: Glyph) -> BytesN<32> {
    let mut b_palette = Bytes::new(&env);
    let mut m_palette: Map<Color, u32> = Map::new(&env);

    for (miner_idx, indexes_colors) in glyph.colors.iter_unchecked() {
        for (index, hex) in indexes_colors.iter_unchecked() {
            let color = Color(hex, miner_idx);

            let current_m_palette_amount = m_palette
                .get(color.clone())
                .unwrap_or(Ok(0))
                .unwrap();

            m_palette.set(color.clone(), current_m_palette_amount + 1);

            // We need to extend the length of the palette
            if b_palette.len() <= index {
                for i in (b_palette.len() / 4)..(index + 1) {
                    // If this is the section we're interested in filling, just fill
                    if i == index {
                        b_palette.insert_from_array(index * 4, &hex.to_le_bytes());
                    } 
                    // Push empty white pixels
                    // TODO: this is a "free" way to use white pixels atm
                    else {
                        b_palette.extend_from_array(&16777215u32.to_le_bytes());
                    }
                }
            } 
            // If the bytes already exist just fill them in
            else {
                b_palette.insert_from_array(index * 4, &hex.to_le_bytes());
            }
        }
    }

    burn(&env, &m_palette);

    env.crypto().sha256(&b_palette)

    // b_palette
    // m_palette

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