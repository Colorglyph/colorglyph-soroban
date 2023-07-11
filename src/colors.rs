use soroban_sdk::{
    token,
    Address,
    Env,
    // Symbol,
    Vec,
};

use crate::types::{MinerColorAmount, MinerOwnerColor, StorageKey};

// const COLORS: Symbol = Symbol::short("COLORS");

pub fn colors_mine(env: &Env, miner: Address, to: Option<Address>, colors: Vec<(u32, u32)>) {
    miner.require_auth();

    // TODO this seems slightly inneficient atm as you can only mine ~15 different colors per invocation

    let to = match to {
        None => miner.clone(),
        Some(address) => address,
    };

    let mut pay_amount: u32 = 0;

    for (color, amount) in colors.iter_unchecked() {
        let miner_owner_color = MinerOwnerColor(miner.clone(), to.clone(), color);

        // env.events()
        //     .publish((COLORS, Symbol::short("mine")), miner_owner_color.clone());

        let current_amount = env
            .storage()
            .get::<MinerOwnerColor, u32>(&miner_owner_color)
            .unwrap_or(Ok(0))
            .unwrap();

        pay_amount += amount;

        env.storage()
            .set(&miner_owner_color, &(current_amount + amount));
    }

    let token_id = env
        .storage()
        .get::<StorageKey, Address>(&StorageKey::TokenAddress)
        .unwrap()
        .unwrap();
    let token = token::Client::new(env, &token_id);
    let fee_address = env
        .storage()
        .get::<StorageKey, Address>(&StorageKey::FeeAddress)
        .unwrap()
        .unwrap();

    token.transfer(&miner, &fee_address, &i128::from(pay_amount));
}

pub fn colors_transfer(env: &Env, from: Address, to: Address, colors: Vec<MinerColorAmount>) {
    from.require_auth();

    // TODO
    // Consider allowing Miner in MinerColorAmount to be Option<Address> and assume the from address in order to reduce the size of the argument being sent

    for miner_color_amount in colors.iter_unchecked() {
        let MinerColorAmount(miner_address, color, amount) = miner_color_amount;
        let miner_owner_color = MinerOwnerColor(miner_address.clone(), from.clone(), color);
        let current_from_amount = env
            .storage()
            .get::<MinerOwnerColor, u32>(&miner_owner_color)
            .unwrap_or(Ok(0))
            .unwrap();

        env.storage()
            .set(&miner_owner_color, &(current_from_amount - amount));

        let to_miner_owner_color = MinerOwnerColor(miner_address, to.clone(), color);
        let current_to_amount = env
            .storage()
            .get::<MinerOwnerColor, u32>(&to_miner_owner_color)
            .unwrap_or(Ok(0))
            .unwrap();

        env.storage()
            .set(&to_miner_owner_color, &(current_to_amount + amount));
    }
}

pub fn colors_mint_or_burn(env: &Env, from: &Address, colors: &Vec<MinerColorAmount>, mint: bool) {
    for miner_color_amount in colors.iter_unchecked() {
        let MinerColorAmount(miner, color, amount) = miner_color_amount;
        let miner_owner_color = MinerOwnerColor(miner, from.clone(), color);
        let current_from_amount = env
            .storage()
            .get::<MinerOwnerColor, u32>(&miner_owner_color)
            .unwrap_or(Ok(0))
            .unwrap();

        env.storage().set(
            &miner_owner_color,
            &if mint {
                current_from_amount + amount
            } else {
                current_from_amount - amount
            },
        );
    }
}
