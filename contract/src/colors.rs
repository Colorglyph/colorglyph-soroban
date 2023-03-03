use soroban_sdk::{symbol, Address, BytesN, Env, Symbol, Vec};

use crate::{
    token,
    types::{MinerColorAmount, MinerOwnerColor, StorageKey},
};

const COLORS: Symbol = symbol!("COLORS");

pub fn mine(env: &Env, from: Address, colors: Vec<(u32, u32)>, to: Option<Address>) {
    from.require_auth();

    let to = match to {
        None => from.clone(),
        Some(address) => address,
    };

    let mut pay_amount: i128 = 0;

    for (hex, amount) in colors.iter_unchecked() {
        let color = MinerOwnerColor(from.clone(), to.clone(), hex);

        env.events()
            .publish((COLORS, symbol!("mine")), color.clone());

        let current_amount: u32 = env.storage().get(&color).unwrap_or(Ok(0)).unwrap();

        pay_amount += i128::from(amount);

        env.storage().set(&color, &(current_amount + amount));
    }

    let token_id: BytesN<32> = env.storage().get(&StorageKey::InitToken).unwrap().unwrap();
    let token = token::Client::new(env, &token_id);
    let fee_address: Address = env.storage().get(&StorageKey::InitFee).unwrap().unwrap();

    token.xfer(&from, &fee_address, &pay_amount);
}

pub fn xfer(env: &Env, from: Address, colors: Vec<MinerColorAmount>, to: Option<Address>) {
    from.require_auth();

    let to = match to {
        None => from.clone(),
        Some(address) => address,
    };

    // TODO: event

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

pub fn adjust(env: &Env, from: &Address, colors: &Vec<MinerColorAmount>, add: bool) {
    // TODO: event

    for color in colors.iter_unchecked() {
        let MinerColorAmount(miner_address, hex, amount) = color;
        let from_color = MinerOwnerColor(miner_address, from.clone(), hex);
        let current_from_amount = env.storage().get(&from_color).unwrap_or(Ok(0)).unwrap();

        env.storage().set(
            &from_color,
            &if add {
                current_from_amount + amount
            } else {
                current_from_amount - amount
            },
        );
    }
}

pub fn get_color(env: &Env, from: Address, hex: u32, miner_address: Address) -> u32 {
    env.storage()
        .get(&MinerOwnerColor(miner_address, from, hex))
        .unwrap_or(Ok(0))
        .unwrap()
}
