use crate::types::{Error, StorageKey};
use soroban_sdk::{panic_with_error, Address, Env};

pub mod persistent {
    use soroban_sdk::{vec, BytesN, Map, Vec};

    use crate::types::{Glyph, Offer};

    use super::*;

    pub fn write_color(env: &Env, miner: &Address, to: &Address, color: u32, amount: u32) {
        let miner_owner_color = StorageKey::Color(miner.clone(), to.clone(), color);

        env.storage()
            .persistent()
            .set::<StorageKey, u32>(&miner_owner_color, &amount);
    }

    pub fn read_color(env: &Env, miner: &Address, to: &Address, color: u32) -> u32 {
        let miner_owner_color = StorageKey::Color(miner.clone(), to.clone(), color);

        env.storage()
            .persistent()
            .get::<StorageKey, u32>(&miner_owner_color)
            .unwrap_or(0)
    }

    pub fn read_glyph_or_default(env: &Env, hash: &BytesN<32>) -> Glyph {
        read_glyph(env, hash).unwrap_or(Glyph {
            width: 0,
            length: 0,
            colors: Map::new(env),
        })
    }

    pub fn read_glyph_or_error(env: &Env, hash: &BytesN<32>) -> Glyph {
        read_glyph(env, hash).unwrap_or_else(|| panic_with_error!(env, Error::NotFound))
    }

    pub fn read_glyph(env: &Env, hash: &BytesN<32>) -> Option<Glyph> {
        let glyph_key = StorageKey::Glyph(hash.clone());

        env.storage()
            .persistent()
            .get::<StorageKey, Glyph>(&glyph_key)
    }

    pub fn read_glyph_owner(env: &Env, hash: &BytesN<32>) -> Option<Address> {
        env.storage()
            .persistent()
            .get(&StorageKey::GlyphOwner(hash.clone()))
    }

    pub fn remove_glyph_owner(env: &Env, hash: &BytesN<32>) {
        let key = StorageKey::GlyphOwner(hash.clone());

        if env.storage().persistent().has(&key) {
            env.storage().persistent().remove(&key);
        }
    }

    pub fn write_glyph_owner(env: &Env, hash: &BytesN<32>, new_owner: &Address) {
        let key = StorageKey::GlyphOwner(hash.clone());

        env.storage().persistent().set(&key, &new_owner);
    }

    pub fn remove_glyph_offer(env: &Env, hash: &BytesN<32>) {
        let key = StorageKey::GlyphOffer(hash.clone());

        if env.storage().persistent().has(&key) {
            env.storage().persistent().remove(&key);
        }
    }

    pub fn read_glyph_minter(env: &Env, hash: &BytesN<32>) -> Option<Address> {
        let buy_glyph_minter_key = StorageKey::GlyphMinter(hash.clone());

        env.storage()
            .persistent()
            .get::<StorageKey, Address>(&buy_glyph_minter_key)
    }

    // Offers-related storage utils

    pub fn read_offers_by_glyph(env: &Env, hash: &BytesN<32>) -> Vec<Offer> {
        let buy_glyph_offer_key = StorageKey::GlyphOffer(hash.clone());
        env.storage()
            .persistent()
            .get::<StorageKey, Vec<Offer>>(&buy_glyph_offer_key)
            .unwrap_or(vec![&env])
    }

    pub fn write_offers_by_glyph(env: &Env, hash: &BytesN<32>, offers: Vec<Offer>) {
        let buy_glyph_offer_key = StorageKey::GlyphOffer(hash.clone());
        env.storage()
            .persistent()
            .set(&buy_glyph_offer_key, &offers);
    }

    pub fn read_asset_offers_by_asset(
        env: &Env,
        hash: &BytesN<32>,
        address: &Address,
        amount: i128,
    ) -> Option<Vec<Address>> {
        let key = StorageKey::AssetOffer(hash.clone(), address.clone(), amount);

        env.storage()
            .persistent()
            .get::<StorageKey, Vec<Address>>(&key)
    }

    pub fn remove_asset_offers_by_asset(
        env: &Env,
        hash: &BytesN<32>,
        address: &Address,
        amount: i128,
    ) {
        let key = StorageKey::AssetOffer(hash.clone(), address.clone(), amount);

        if env.storage().persistent().has(&key) {
            env.storage().persistent().remove(&key);
        }
    }

    pub fn has_asset_offers_by_asset(
        env: &Env,
        hash: &BytesN<32>,
        address: &Address,
        amount: i128,
    ) -> bool {
        let key = StorageKey::AssetOffer(hash.clone(), address.clone(), amount);

        env.storage().persistent().has(&key)
    }

    pub fn write_asset_offers_by_asset(
        env: &Env,
        hash: &BytesN<32>,
        address: &Address,
        amount: i128,
        offers: &Vec<Address>,
    ) {
        let key = StorageKey::AssetOffer(hash.clone(), address.clone(), amount);

        env.storage().persistent().set(&key, offers);
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

    pub fn write_mine_multiplier(env: &Env, mine_multiplier: &i128) {
        env.storage()
            .instance()
            .set(&StorageKey::MineMultiplier, mine_multiplier);
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

    pub fn read_mine_multiplier(env: &Env) -> i128 {
        env.storage()
            .instance()
            .get(&StorageKey::MineMultiplier)
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
