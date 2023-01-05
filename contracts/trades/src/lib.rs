#![no_std]

use error::ContractError;
use soroban_sdk::{panic_with_error, contracttype, contractimpl, symbol, Env, Symbol, Vec, vec, bytes, Bytes, BytesN, map, RawVal, Address};

#[contracttype]
#[derive(PartialEq, Debug, Clone)]
pub struct Glyph {
    pub width: u32,
    pub colors: Bytes,
}
#[contracttype]
pub struct Color(
    pub u32,
    pub u32,
    pub u32
);
#[contracttype]
pub struct ColorMap {
    pub miner: Address,
    pub color: Color,
}
#[contracttype]
pub struct DataKey(
    pub Symbol, 
    pub u128,
);

const COUNTER: Symbol = symbol!("COUNTER");

pub struct ColorGlyphContract;

#[contractimpl]
impl ColorGlyphContract {

    pub fn mint(env: Env) -> BytesN<32> {
        let mut count: u128 = env
        .storage()
        .get(COUNTER)
        .unwrap_or(Ok(0))
        .unwrap();

        count += 1;

        // to build out the miner map we need a color array of color:miner
        // the whole point of tracking color miners is to enable them to claim royalties
            // glyph minters will also claim royalties
        // track scraped state
        // take from user palette when minting
        // create a mining function
        // refill a user palette when scraping
        // special mint case when minting a scraped glyph
        // ensure mint uniqueness (no two identical `colors` Bytes arrays)

        let glyph = &Glyph{width: 3, colors: bytes!(&env, [1,0,1, 0,1,0, 1,0,1,])};

        let glyph_key = DataKey(symbol!("Glyphs"), count);
        let glyph_owner_key = DataKey(symbol!("Gl_Owners"), count);
        let glyph_minter_key = DataKey(symbol!("Gl_Minters"), count);
        let glyph_miner_key = DataKey(symbol!("Gl_Miners"), count);

        env.storage().set(COUNTER, count);
        env.storage().set(glyph_key, glyph);
        env.storage().set(glyph_owner_key, env.invoker());
        env.storage().set(glyph_minter_key, env.invoker());
        env.storage().set(glyph_miner_key, map!(&env, (env.invoker(), 9)));
        
        env.crypto().sha256(&glyph.colors)
    }

    pub fn get(env: Env, count: u128) -> Vec<RawVal> {
        let glyph_key = DataKey(symbol!("Glyphs"), count);
        let glyph_owner_key = DataKey(symbol!("Gl_Owners"), count);
        let glyph_minter_key = DataKey(symbol!("Gl_Minters"), count);
        let glyph_miner_key = DataKey(symbol!("Gl_Miners"), count);

        vec![
            &env, 

            env
            .storage()
            .get(COUNTER)
            .unwrap_or_else(|| panic_with_error!(&env, ContractError::Generic))
            .unwrap(),

            env
            .storage()
            .get(glyph_key)
            .unwrap_or_else(|| panic_with_error!(&env, ContractError::Generic))
            .unwrap(),

            env
            .storage()
            .get(glyph_owner_key)
            .unwrap_or_else(|| panic_with_error!(&env, ContractError::Generic))
            .unwrap(),

            env
            .storage()
            .get(glyph_minter_key)
            .unwrap_or_else(|| panic_with_error!(&env, ContractError::Generic))
            .unwrap(),

            env
            .storage()
            .get(glyph_miner_key)
            .unwrap_or_else(|| panic_with_error!(&env, ContractError::Generic))
            .unwrap()
        ]
    }

}

mod error;
mod test;