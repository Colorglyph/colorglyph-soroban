#![no_std]

use colors_contract::Client;
use soroban_sdk::{contracttype, contractimpl, symbol, Env, Symbol, Vec, Map, Bytes, BytesN};

mod colors_contract {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32-unknown-unknown/release/soroban_colors_contract.wasm"
    );
}

#[contracttype]
#[derive(PartialEq, Debug, Clone)]
pub struct Glyph {
    pub width: u32,
    pub colors: Map<u32, Vec<(u32, BytesN<3>)>>,
}

// const COUNTER: Symbol = symbol!("COUNTER");
const GLYPHS: Symbol = symbol!("GLYPHS");

pub struct GlyphContract;

#[contractimpl]
impl GlyphContract {
    pub fn mint(
        env: Env,
        glyph: Glyph,
        colors_contract_id: BytesN<32>
    ) -> Bytes {
        let client = get_client(&env, colors_contract_id);
        let mut palette = Bytes::new(&env);

        for (miner_idx, index_colors) in glyph.colors.iter_unchecked() {
            for (index, color) in index_colors.iter_unchecked() {
                let index_times_three = index * 3;

                // We need to extend the length of the palette
                if palette.len() <= index_times_three {
                    for i in (palette.len() / 3)..(index + 1) {
                        // If this is the section we're interested in filling, just fill
                        if i == index {
                            palette.insert(index_times_three, color.get_unchecked(0));
                            palette.insert(index_times_three + 1, color.get_unchecked(1));
                            palette.insert(index_times_three + 2, color.get_unchecked(2));
                        } 
                        // Push empty white pixels
                        // TODO: this is a "free" way to use white pixels atm
                        else { 
                            palette.push(255);
                            palette.push(255);
                            palette.push(255);
                        }
                    }
                } 
                // If the bytes already exist just fill them in
                else {
                    palette.set(index_times_three, color.get_unchecked(0));
                    palette.set(index_times_three + 1, color.get_unchecked(1));
                    palette.set(index_times_three + 2, color.get_unchecked(2));
                }
            }
        }

        // env.crypto().sha256(&palette);

        palette

        // let mut count: u128 = env
        // .storage()
        // .get(COUNTER)
        // .unwrap_or(Ok(0))
        // .unwrap();

        // count += 1;

        // to build out the miner map we need a color array of color:miner
        // the whole point of tracking color miners is to enable them to claim royalties
            // glyph minters will also claim royalties
        // track scraped state
        // take from user palette when minting
        // create a mining function
        // refill a user palette when scraping
        // special mint case when minting a scraped glyph
        // ensure mint uniqueness (no two identical `colors` Bytes arrays)

        // let glyph = &Glyph{width: 3, colors: bytes!(&env, [1,0,1, 0,1,0, 1,0,1,])};
        
        // let glyph_key = DataKey(symbol!("Glyphs"), count);
        // let glyph_owner_key = DataKey(symbol!("Gl_Owners"), count);
        // let glyph_minter_key = DataKey(symbol!("Gl_Minters"), count);
        // let glyph_miner_key = DataKey(symbol!("Gl_Miners"), count);

        // env.storage().set(COUNTER, count);
        // env.storage().set(glyph_key, glyph);
        // env.storage().set(glyph_owner_key, env.invoker());
        // env.storage().set(glyph_minter_key, env.invoker());
        // env.storage().set(glyph_miner_key, map!(&env, (env.invoker(), 9)));
        
        // env.crypto().sha256(&glyph.colors)
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
}

fn get_client(env: &Env, contract_id: BytesN<32>) -> Client {
    colors_contract::Client::new(env, contract_id)
}

mod test;