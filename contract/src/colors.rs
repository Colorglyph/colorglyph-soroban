use soroban_sdk::{symbol, Env, Symbol, Vec, AccountId, Address, BytesN};

use crate::{
    types::{MaybeAccountId, Color, ColorOwned, ColorAmount, StorageKey}, 
    token::{Signature, Identifier, Client as TokenClient}
};

const ACC_IDX_I: Symbol = symbol!("ACC_IDX_I");
const COLORS: Symbol = symbol!("COLORS");

pub fn mine(env: &Env, signature: Signature, colors: Vec<(u32, u32)>, to: MaybeAccountId) {
    let miner_address = env.invoker();
    let miner_idx = get_account_idx(env, &miner_address);

    let to_address = get_source_account(env, to);
    let to_idx = get_account_idx(env, &to_address);

    let mut pay_amount: i128 = 0;

    for (hex, amount) in colors.iter_unchecked() {
        let color = ColorOwned(to_idx, hex, miner_idx);

        env.events().publish((COLORS, symbol!("mine")), (&to_address, hex, &miner_address));

        let current_amount: u32 = env
            .storage()
            .get(color)
            .unwrap_or(Ok(0))
            .unwrap();

        pay_amount += amount as i128;

        env
            .storage()
            .set(color, current_amount + amount);
    }

    let contract_id = Identifier::Contract(env.get_current_contract());
    let sig_id = signature.identifier(env);
    let token_id: BytesN<32> = env.storage().get(StorageKey::InitToken).unwrap().unwrap();
    let token = TokenClient::new(env, token_id);
    let fee_identifier: Identifier = env.storage().get(StorageKey::InitFeeId).unwrap().unwrap();
    let sender_nonce = token.nonce(&sig_id);

    token.incr_allow(
        &signature,
        &sender_nonce,
        &contract_id, 
        &pay_amount
    );

    token.xfer_from(
        &Signature::Invoker,
        &0,
        &sig_id,
        &fee_identifier,
        &pay_amount
    );
}

pub fn xfer(env: &Env, colors: Vec<ColorAmount>, to: MaybeAccountId) {
    let self_address = env.invoker();
    let self_idx = get_account_idx(env, &self_address);

    let to_address = get_source_account(env, to);
    let to_idx = get_account_idx(env, &to_address);

    // TODO: event

    for color in colors.iter_unchecked() {
        let ColorAmount(Color(hex, miner_idx), amount) = color;
        let from_color = ColorOwned(self_idx, hex, miner_idx);
        let current_from_amount: u32 = env
            .storage()
            .get(from_color)
            .unwrap_or(Ok(0))
            .unwrap();

        env
            .storage()
            .set(from_color, current_from_amount - amount);

        let to_color = ColorOwned(to_idx, hex, miner_idx);
        let current_to_amount: u32 = env
            .storage()
            .get(to_color)
            .unwrap_or(Ok(0))
            .unwrap();

        env
            .storage()
            .set(to_color, current_to_amount + amount);
    }
}

pub fn adjust(env: &Env, colors: &Vec<ColorAmount>, add: bool) {
    let self_address = env.invoker();
    let self_idx = get_account_idx(env, &self_address);

    // TODO: event

    for color in colors.iter_unchecked() {
        let ColorAmount(Color(hex, miner_idx), amount) = color;
        let from_color = ColorOwned(self_idx, hex, miner_idx);
        let current_from_amount = env
            .storage()
            .get(from_color)
            .unwrap_or(Ok(0))
            .unwrap();

        env
            .storage()
            .set(from_color,  if add { current_from_amount + amount } else { current_from_amount - amount });
    }
}

pub fn get_color(env: &Env, hex: u32, miner: AccountId) -> u32 {
    let self_address = env.invoker();
    let self_idx = get_account_idx(env, &self_address);

    let miner_address = Address::Account(miner);
    let miner_idx = get_account_idx(env, &miner_address);

    env
        .storage()
        .get(ColorOwned(self_idx, hex, miner_idx))
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
    let mut account_idx = env
        .storage()
        .get(source_account)
        .unwrap_or(Ok(0))
        .unwrap();

    if account_idx == 0 {
        account_idx = env
            .storage()
            .get(ACC_IDX_I)
            .unwrap_or(Ok(0))
            .unwrap();

        account_idx += 1;

        env
            .storage()
            .set(ACC_IDX_I, account_idx);

        env
            .storage()
            .set(source_account, account_idx);
    }

    account_idx
}