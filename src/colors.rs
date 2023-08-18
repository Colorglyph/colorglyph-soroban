use soroban_sdk::{symbol_short, token, Address, Env, Map, Vec};

use crate::{contract::MAX_ENTRY_LIFETIME, types::StorageKey};

pub fn colors_mine(env: &Env, miner: Address, to: Option<Address>, colors: Map<u32, u32>) {
    miner.require_auth();

    let to = match to {
        Some(address) => address,
        None => miner.clone(),
    };

    let mut pay_amount: u32 = 0;

    for (color, amount) in colors.iter() {
        let miner_owner_color = StorageKey::Color(miner.clone(), to.clone(), color);

        let current_amount = env
            .storage()
            .persistent()
            .get::<StorageKey, u32>(&miner_owner_color)
            .unwrap_or(0);

        pay_amount += amount;

        env.storage()
            .persistent()
            .set(&miner_owner_color, &(current_amount + amount));

        env.storage()
            .persistent()
            .bump(&miner_owner_color, MAX_ENTRY_LIFETIME);

        env.events().publish(
            (symbol_short!("mine"), miner.clone(), to.clone()),
            (color, amount),
        );
    }

    let token_address = env
        .storage()
        .instance()
        .get::<StorageKey, Address>(&StorageKey::TokenAddress)
        .unwrap();
    let fee_address = env
        .storage()
        .instance()
        .get::<StorageKey, Address>(&StorageKey::FeeAddress)
        .unwrap();
    let token = token::Client::new(env, &token_address);

    env.storage().instance().bump(MAX_ENTRY_LIFETIME);

    // TODO this is just a stroop fee so not sufficient. This will need to be adjusted before going live
    token.transfer(&miner, &fee_address, &(pay_amount as i128));
}

pub fn colors_transfer(env: &Env, from: Address, to: Address, colors: Vec<(Address, u32, u32)>) {
    from.require_auth();

    for (miner_address, color, amount) in colors.iter() {
        let from_miner_owner_color = StorageKey::Color(miner_address.clone(), from.clone(), color);
        let to_miner_owner_color = StorageKey::Color(miner_address.clone(), to.clone(), color);

        let current_from_amount = env
            .storage()
            .persistent()
            .get::<StorageKey, u32>(&from_miner_owner_color)
            .unwrap_or(0);
        let current_to_amount = env
            .storage()
            .persistent()
            .get::<StorageKey, u32>(&to_miner_owner_color)
            .unwrap_or(0);

        env.storage()
            .persistent()
            .set(&from_miner_owner_color, &(current_from_amount - amount));
        env.storage()
            .persistent()
            .set(&to_miner_owner_color, &(current_to_amount + amount));

        env.storage()
            .persistent()
            .bump(&from_miner_owner_color, MAX_ENTRY_LIFETIME);
        env.storage()
            .persistent()
            .bump(&to_miner_owner_color, MAX_ENTRY_LIFETIME);

        env.events().publish(
            (symbol_short!("transfer"), from.clone(), to.clone()),
            (miner_address, color, amount),
        );
    }
}

pub fn color_balance(env: &Env, owner: Address, miner: Option<Address>, color: u32) -> u32 {
    let miner = match miner {
        None => owner.clone(),
        Some(address) => address,
    };
    let color_key = StorageKey::Color(miner, owner, color);

    let color = env
        .storage()
        .persistent()
        .get::<StorageKey, u32>(&color_key)
        .unwrap_or(0);

    env.storage()
        .persistent()
        .bump(&color_key, MAX_ENTRY_LIFETIME);

    color
}
