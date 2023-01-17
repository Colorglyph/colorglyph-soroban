use soroban_sdk::{symbol, AccountId, Address, BytesN, Env, Symbol, Vec};

use crate::{
    token::{Client as TokenClient, Identifier, Signature},
    types::{ColorAmount, ColorOwner, MaybeAccountId, StorageKey},
    utils::get_token_bits,
};

const ACC_IDX_I: Symbol = symbol!("ACC_IDX_I");
const COLORS: Symbol = symbol!("COLORS");

pub fn mine(env: &Env, signature: Signature, colors: Vec<(u32, i128)>, to: MaybeAccountId) {
    let miner_address = env.invoker();
    let miner_idx = get_account_idx(env, &miner_address);

    let to_address = get_source_account(env, to);
    let to_idx = get_account_idx(env, &to_address);

    let mut pay_amount: i128 = 0;

    for (hex, amount) in colors.iter_unchecked() {
        let color = ColorOwner(hex, miner_idx, to_idx);

        env.events().publish(
            (COLORS, symbol!("mine")),
            (&to_address, hex, &miner_address),
        );

        let current_amount: i128 = env.storage().get(color).unwrap_or(Ok(0)).unwrap();

        pay_amount += amount;

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

pub fn xfer(env: &Env, colors: Vec<ColorAmount>, to: MaybeAccountId) {
    let self_address = env.invoker();
    let self_idx = get_account_idx(env, &self_address);

    let to_address = get_source_account(env, to);
    let to_idx = get_account_idx(env, &to_address);

    // TODO: event

    for color in colors.iter_unchecked() {
        let ColorAmount(hex, miner_idx, amount) = color;
        let from_color = ColorOwner(hex, miner_idx, self_idx);
        let current_from_amount: i128 = env.storage().get(from_color).unwrap_or(Ok(0)).unwrap();

        env.storage().set(from_color, current_from_amount - amount);

        let to_color = ColorOwner(hex, miner_idx, to_idx);
        let current_to_amount: i128 = env.storage().get(to_color).unwrap_or(Ok(0)).unwrap();

        env.storage().set(to_color, current_to_amount + amount);
    }
}

pub fn adjust(env: &Env, colors: &Vec<ColorAmount>, add: bool) {
    let self_address = env.invoker();
    let self_idx = get_account_idx(env, &self_address);

    // TODO: event

    for color in colors.iter_unchecked() {
        let ColorAmount(hex, miner_idx, amount) = color;
        let from_color = ColorOwner(hex, miner_idx, self_idx);
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

pub fn get_color(env: &Env, hex: u32, miner: AccountId) -> i128 {
    let self_address = env.invoker();
    let self_idx = get_account_idx(env, &self_address);

    let miner_address = Address::Account(miner);
    let miner_idx = get_account_idx(env, &miner_address);

    env.storage()
        .get(ColorOwner(hex, miner_idx, self_idx))
        .unwrap_or(Ok(0))
        .unwrap()
}

fn get_source_account(env: &Env, to: MaybeAccountId) -> Address {
    let source_account: Address;

    match to {
        MaybeAccountId::None => source_account = env.invoker(),
        MaybeAccountId::AccountId(account_id) => source_account = Address::Account(account_id),
    }

    source_account
}

fn get_account_idx(env: &Env, source_account: &Address) -> u32 {
    let mut account_idx = env.storage().get(source_account).unwrap_or(Ok(0)).unwrap();

    if account_idx == 0 {
        account_idx = env.storage().get(ACC_IDX_I).unwrap_or(Ok(0)).unwrap();

        account_idx += 1;

        env.storage().set(ACC_IDX_I, account_idx);

        env.storage().set(source_account, account_idx);
    }

    account_idx
}
