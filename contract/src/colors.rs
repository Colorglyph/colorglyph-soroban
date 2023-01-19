use soroban_sdk::{panic_with_error, symbol, AccountId, Address, BytesN, Env, Symbol, Vec};

use crate::{
    token::{Identifier, Signature},
    types::{Error, MaybeAddress, MinerColorAmount, MinerOwnerColor, StorageKey},
    utils::get_token_bits,
};

const COLORS: Symbol = symbol!("COLORS");

pub fn mine(env: &Env, signature: Signature, colors: Vec<(u32, u32)>, to: MaybeAddress) {
    let self_address = &env.invoker();

    let to_address = &match to {
        MaybeAddress::None => self_address.clone(),
        MaybeAddress::Address(address) => address,
    };

    let mut pay_amount: i128 = 0;

    for (hex, amount) in colors.iter_unchecked() {
        let color = &MinerOwnerColor(self_address.clone(), to_address.clone(), hex);

        env.events().publish(
            (COLORS, symbol!("mine")),
            color,
        );

        let current_amount: u32 = env.storage().get(color).unwrap_or(Ok(0)).unwrap();

        pay_amount += i128::from(amount);

        env.storage().set(color, current_amount + amount);
    }

    let token_id: BytesN<32> = env.storage().get(StorageKey::InitToken).unwrap().unwrap();
    let fee_identifier: Identifier = env.storage().get(StorageKey::InitFeeId).unwrap().unwrap();

    let (contract_identifier, signature_identifier, token, sender_nonce) =
        get_token_bits(env, &token_id, &signature);

    token.incr_allow(&signature, &sender_nonce, &contract_identifier, &pay_amount);

    token.xfer_from(
        &Signature::Invoker,
        &0,
        &signature_identifier,
        &fee_identifier,
        &pay_amount,
    );
}

pub fn xfer(env: &Env, colors: Vec<MinerColorAmount>, to: MaybeAddress) {
    let self_address = &env.invoker();

    let to_address = &match to {
        MaybeAddress::None => self_address.clone(),
        MaybeAddress::Address(account_id) => account_id,
    };

    // TODO: event

    for color in colors.iter_unchecked() {
        let MinerColorAmount(miner_address, hex, amount) = &color;
        let from_color = &MinerOwnerColor(miner_address.clone(), self_address.clone(), *hex);
        let current_from_amount: u32 = env.storage().get(from_color).unwrap_or(Ok(0)).unwrap();

        env.storage().set(from_color, current_from_amount - amount);

        let to_color = &MinerOwnerColor(miner_address.clone(), to_address.clone(), *hex);
        let current_to_amount: u32 = env.storage().get(to_color).unwrap_or(Ok(0)).unwrap();

        env.storage().set(to_color, current_to_amount + amount);
    }
}

pub fn adjust(env: &Env, colors: &Vec<MinerColorAmount>, add: bool) {
    let self_address = env.invoker();

    // TODO: event

    for color in colors.iter_unchecked() {
        let MinerColorAmount(miner_address, hex, amount) = color;
        let from_color = &MinerOwnerColor(miner_address, self_address.clone(), hex);
        let current_from_amount = env.storage().get(from_color).unwrap_or(Ok(0)).unwrap();

        env.storage().set(
            from_color,
            if add {
                current_from_amount + amount
            } else {
                current_from_amount - amount
            },
        );
    }
}

pub fn get_color(env: &Env, hex: u32, miner_address: Address) -> u32 {
    let self_address = env.invoker();

    env.storage()
        .get(MinerOwnerColor(miner_address, self_address, hex))
        .unwrap_or(Ok(0))
        .unwrap()
}
