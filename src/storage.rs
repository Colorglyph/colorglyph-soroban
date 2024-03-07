use soroban_sdk::{Address, Env, panic_with_error};
use crate::types::{StorageKey, Error};


pub mod persistent {
    use super::*;

    pub fn write_color(env: &Env, miner: Address, to: Address, color: u32, amount: u32) {
        let miner_owner_color = StorageKey::Color(miner.clone(), to.clone(), color);
        
        env
            .storage()
            .persistent()
            .set::<StorageKey, u32>(&miner_owner_color, &amount);
    }

    pub fn read_color(env: &Env, miner: Address, to: Address, color: u32) -> u32 {
        let miner_owner_color = StorageKey::Color(miner.clone(), to.clone(), color);

        env
            .storage()
            .persistent()
            .get::<StorageKey, u32>(&miner_owner_color)
            .unwrap_or(0)
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
                .get(&StorageKey::OwnerAddress).unwrap_or_else(|| panic_with_error!(env, Error::NotInitialized))
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