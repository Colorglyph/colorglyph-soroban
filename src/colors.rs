use soroban_sdk::{
    token,
    Address,
    Env,
    Map,
    // Symbol,
    Vec,
};

use crate::types::StorageKey;

// const COLORS: Symbol = Symbol::short("COLORS");

pub fn colors_mine(env: &Env, miner: Address, to: Option<Address>, colors: Map<u32, u32>) {
    miner.require_auth();

    // TODO this seems slightly inneficient atm as you can only mine ~15 different colors per invocation

    let to = match to {
        Some(address) => address,
        None => miner.clone(),
    };

    let mut pay_amount: u32 = 0;

    for (color, amount) in colors.iter() {
        let miner_owner_color = StorageKey::Color(miner.clone(), to.clone(), color);
        // env.events()
        //     .publish((COLORS, Symbol::short("mine")), miner_owner_color.clone());

        let current_amount = env
            .storage()
            .persistent()
            .get::<StorageKey, u32>(&miner_owner_color)
            .unwrap_or(0);

        pay_amount += amount;

        env.storage()
            .persistent()
            .set(&miner_owner_color, &(current_amount + amount));
    }

    let token_id = env
        .storage()
        .persistent()
        .get::<StorageKey, Address>(&StorageKey::TokenAddress)
        .unwrap();
    let token = token::Client::new(env, &token_id);
    let fee_address = env
        .storage()
        .persistent()
        .get::<StorageKey, Address>(&StorageKey::FeeAddress)
        .unwrap();

    token.transfer(&miner, &fee_address, &i128::from(pay_amount));
}

pub fn colors_transfer(env: &Env, from: Address, to: Address, colors: Vec<(Address, u32, u32)>) {
    from.require_auth();

    // TODO
    // Consider allowing the miner Address in `colors` to be Option<Address> and assume the from address in order to reduce the size of the argument being sent

    for miner_color_amount in colors.iter() {
        let (miner_address, color, amount) = miner_color_amount;

        let from_miner_owner_color = StorageKey::Color(miner_address.clone(), from.clone(), color);
        let to_miner_owner_color = StorageKey::Color(miner_address, to.clone(), color);

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
    }
}

pub fn color_balance(env: &Env, owner: Address, miner: Option<Address>, color: u32) -> u32 {
    let miner = match miner {
        None => owner.clone(),
        Some(address) => address,
    };

    env.storage()
        .persistent()
        .get::<StorageKey, u32>(&StorageKey::Color(miner, owner, color))
        .unwrap_or(0)
}
