use soroban_sdk::{token, Address, Env, Symbol, Vec};

use crate::types::{MinerColorAmount, MinerOwnerColor, StorageKey};

const COLORS: Symbol = Symbol::short("COLORS");

pub fn colors_mine(env: &Env, miner: Address, to: Option<Address>, colors: Vec<(u32, u32)>) {
    miner.require_auth();

    let to = match to {
        None => miner.clone(),
        Some(address) => address,
    };

    let mut pay_amount: i128 = 0;

    for (hex, amount) in colors.iter_unchecked() {
        let color = MinerOwnerColor(miner.clone(), to.clone(), hex);

        env.events()
            .publish((COLORS, Symbol::short("mine")), color.clone());

        let current_amount: u32 = env.storage().get(&color).unwrap_or(Ok(0)).unwrap();

        pay_amount += i128::from(amount);

        env.storage().set(&color, &(current_amount + amount));
    }

    let token_id = env
        .storage()
        .get::<StorageKey, Address>(&StorageKey::InitToken)
        .unwrap()
        .unwrap();
    let token = token::Client::new(env, &token_id);
    let fee_address = env
        .storage()
        .get::<StorageKey, Address>(&StorageKey::InitFee)
        .unwrap()
        .unwrap();

    token.transfer(&miner, &fee_address, &pay_amount);
}

pub fn colors_transfer(env: &Env, from: Address, to: Address, colors: Vec<MinerColorAmount>) {
    from.require_auth();

    for color in colors.iter_unchecked() {
        let MinerColorAmount(miner_address, hex, amount) = color;
        let from_color = MinerOwnerColor(miner_address.clone(), from.clone(), hex);
        let current_from_amount: u32 = env.storage().get(&from_color).unwrap_or(Ok(0)).unwrap();

        env.storage()
            .set(&from_color, &(current_from_amount - amount));

        let to_color = MinerOwnerColor(miner_address, to.clone(), hex);
        let current_to_amount: u32 = env.storage().get(&to_color).unwrap_or(Ok(0)).unwrap();

        env.storage().set(&to_color, &(current_to_amount + amount));
    }
}

pub fn color_balance(env: &Env, owner: Address, miner: Option<Address>, color: u32) -> u32 {
    let miner = match miner {
        None => owner.clone(),
        Some(address) => address,
    };

    env.storage()
        .get(&MinerOwnerColor(miner, owner, color))
        .unwrap_or(Ok(0))
        .unwrap()
}