use crate::types::{Error, StorageKey};
use soroban_sdk::{panic_with_error, Address, Env};

pub mod persistent {
    use soroban_sdk::{BytesN, Map, Vec};

    use crate::types::Glyph;

    use super::*;

    pub fn write_color(env: &Env, miner: Address, to: Address, color: u32, amount: u32) {
        let miner_owner_color = StorageKey::Color(miner.clone(), to.clone(), color);

        env.storage()
            .persistent()
            .set::<StorageKey, u32>(&miner_owner_color, &amount);
    }

    pub fn write_colors(env: &Env, minter: Address, colors: &Map<Address, Map<u32, Vec<u32>>>) {
        let glyph_colors_key = StorageKey::Colors(minter.clone());

        env.storage()
            .persistent()
            .set::<StorageKey, Map<Address, Map<u32, Vec<u32>>>>(&glyph_colors_key, colors);
    }

    pub fn read_color(env: &Env, miner: Address, to: Address, color: u32) -> u32 {
        let miner_owner_color = StorageKey::Color(miner.clone(), to.clone(), color);

        env.storage()
            .persistent()
            .get::<StorageKey, u32>(&miner_owner_color)
            .unwrap_or(0)
    }

    pub fn read_colors_or_map(env: &Env, minter: Address) -> Map<Address, Map<u32, Vec<u32>>> {
        read_colors(env, minter).unwrap_or(Map::new(&env))
    }

    pub fn read_colors_or_error(env: &Env, minter: Address) -> Map<Address, Map<u32, Vec<u32>>> {
        read_colors(env, minter).unwrap_or_else(|| panic_with_error!(env, Error::NotFound))
    }

    fn read_colors(env: &Env, minter: Address) -> Option<Map<Address, Map<u32, Vec<u32>>>> {
        let glyph_colors_key = StorageKey::Colors(minter.clone());

        env.storage()
            .persistent()
            .get::<StorageKey, Map<Address, Map<u32, Vec<u32>>>>(&glyph_colors_key)
    }

    pub fn read_glyph(env: &Env, hash: BytesN<32>) -> Result<Glyph, Error> {
        let glyph_key = StorageKey::Glyph(hash.clone());

        env.storage()
            .persistent()
            .get::<StorageKey, Glyph>(&glyph_key)
            .ok_or(Error::NotFound)
    }

    pub fn remove_glyph_owner(env: &Env, hash: BytesN<32>) {
        env.storage()
            .persistent()
            .remove(&StorageKey::GlyphOwner(hash));
    }

    pub fn remove_glyph_offer(env: &Env, hash: BytesN<32>) {
        env.storage()
            .persistent()
            .remove(&StorageKey::GlyphOffer(hash));
    }

    pub fn remove_colors(env: &Env, owner: Address) {
        let colors_key = StorageKey::Colors(owner);
        env.storage().persistent().remove(&colors_key);
    }

    pub fn has_colors(env: &Env, owner: Address) -> bool {
        env.storage().persistent().has(&StorageKey::Colors(owner))
    }
}

pub mod instance {
    use super::*;

    pub fn write_owner_address(env: &Env, owner: &Address) {
        env.storage()
            .instance()
            .set(&StorageKey::OwnerAddress, owner);
    }

    pub fn write_token_address(env: &Env, token: &Address) {
        env.storage()
            .instance()
            .set(&StorageKey::TokenAddress, token);
    }

    pub fn write_fee_address(env: &Env, fee_address: &Address) {
        env.storage()
            .instance()
            .set(&StorageKey::FeeAddress, fee_address);
    }

    pub fn write_max_entry_lifetime(env: &Env, max_entry_lifetime: &u32) {
        env.storage()
            .instance()
            .set(&StorageKey::MaxEntryLifetime, max_entry_lifetime);
    }

    pub fn write_max_payment_count(env: &Env, max_payment_count: &u32) {
        env.storage()
            .instance()
            .set(&StorageKey::MaxPaymentCount, max_payment_count);
    }

    pub fn write_minter_royalty_rate(env: &Env, minter_royalty_rate: &i128) {
        env.storage()
            .instance()
            .set(&StorageKey::MinterRoyaltyRate, minter_royalty_rate);
    }

    pub fn write_miner_royalty_rate(env: &Env, miner_royalty_rate: &i128) {
        env.storage()
            .instance()
            .set(&StorageKey::MinerRoyaltyRate, miner_royalty_rate);
    }

    pub fn read_owner_address(env: &Env) -> Address {
        env.storage()
            .instance()
            .get(&StorageKey::OwnerAddress)
            .unwrap_or_else(|| panic_with_error!(env, Error::NotInitialized))
    }

    pub fn read_token_address(env: &Env) -> Address {
        env.storage()
            .instance()
            .get(&StorageKey::TokenAddress)
            .unwrap_or_else(|| panic_with_error!(env, Error::NotInitialized))
    }

    pub fn read_fee_address(env: &Env) -> Address {
        env.storage()
            .instance()
            .get(&StorageKey::FeeAddress)
            .unwrap_or_else(|| panic_with_error!(env, Error::NotInitialized))
    }

    pub fn read_max_entry_lifetime(env: &Env) -> u32 {
        env.storage()
            .instance()
            .get(&StorageKey::MaxEntryLifetime)
            .unwrap_or_else(|| panic_with_error!(env, Error::NotInitialized))
    }

    pub fn read_max_payment_count(env: &Env) -> u32 {
        env.storage()
            .instance()
            .get(&StorageKey::MaxPaymentCount)
            .unwrap_or_else(|| panic_with_error!(env, Error::NotInitialized))
    }

    pub fn read_minter_royalty_rate(env: &Env) -> i128 {
        env.storage()
            .instance()
            .get(&StorageKey::MinterRoyaltyRate)
            .unwrap_or_else(|| panic_with_error!(env, Error::NotInitialized))
    }

    pub fn read_miner_royalty_rate(env: &Env) -> i128 {
        env.storage()
            .instance()
            .get(&StorageKey::MinerRoyaltyRate)
            .unwrap_or_else(|| panic_with_error!(env, Error::NotInitialized))
    }
}
