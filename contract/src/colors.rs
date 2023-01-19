use soroban_sdk::{panic_with_error, symbol, AccountId, Address, BytesN, Env, Symbol, Vec};

use crate::{
    token::{Identifier, Signature},
    types::{Error, MaybeAccountId, MinerColorAmount, MinerOwnerColor, StorageKey},
    utils::get_token_bits,
};

const COLORS: Symbol = symbol!("COLORS");

pub fn mine(env: &Env, signature: Signature, colors: Vec<(u32, u32)>, to: MaybeAccountId) {
    let miner_account_id = &invoker_account_id(env);

    let to_account_id = &match to {
        MaybeAccountId::None => match env.invoker() {
            Address::Account(account_id) => account_id,
            _ => panic_with_error!(env, Error::NotPermitted),
        },
        MaybeAccountId::AccountId(account_id) => account_id,
    };

    let mut pay_amount: i128 = 0;

    for (hex, amount) in colors.iter_unchecked() {
        let color = &MinerOwnerColor(miner_account_id.clone(), to_account_id.clone(), hex);

        env.events().publish(
            (COLORS, symbol!("mine")),
            (miner_account_id, to_account_id, hex),
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

pub fn xfer(env: &Env, colors: Vec<MinerColorAmount>, to: MaybeAccountId) {
    let self_account_id = &invoker_account_id(env);

    let to_account_id = &match to {
        MaybeAccountId::None => match env.invoker() {
            Address::Account(account_id) => account_id,
            _ => panic_with_error!(env, Error::NotPermitted),
        },
        MaybeAccountId::AccountId(account_id) => account_id,
    };

    // TODO: event

    for color in colors.iter_unchecked() {
        let MinerColorAmount(miner_account_id, hex, amount) = &color;
        let from_color = &MinerOwnerColor(miner_account_id.clone(), self_account_id.clone(), *hex);
        let current_from_amount: u32 = env.storage().get(from_color).unwrap_or(Ok(0)).unwrap();

        env.storage().set(from_color, current_from_amount - amount);

        let to_color = &MinerOwnerColor(miner_account_id.clone(), to_account_id.clone(), *hex);
        let current_to_amount: u32 = env.storage().get(to_color).unwrap_or(Ok(0)).unwrap();

        env.storage().set(to_color, current_to_amount + amount);
    }
}

pub fn adjust(env: &Env, colors: &Vec<MinerColorAmount>, add: bool) {
    let self_account_id = &invoker_account_id(env);

    // TODO: event

    for color in colors.iter_unchecked() {
        let MinerColorAmount(miner_account_id, hex, amount) = color;
        let from_color = &MinerOwnerColor(miner_account_id, self_account_id.clone(), hex);
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

pub fn get_color(env: &Env, hex: u32, miner_account_id: AccountId) -> u32 {
    let self_account_id = invoker_account_id(env);

    env.storage()
        .get(MinerOwnerColor(miner_account_id, self_account_id, hex))
        .unwrap_or(Ok(0))
        .unwrap()
}

fn invoker_account_id(env: &Env) -> AccountId {
    match env.invoker() {
        Address::Account(account_id) => account_id,
        _ => panic_with_error!(env, Error::NotPermitted),
    }
}
