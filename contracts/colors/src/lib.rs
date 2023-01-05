#![no_std]

use soroban_sdk::{contracttype, contractimpl, symbol, Env, Symbol, Map, AccountId, Address};

#[contracttype]
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub enum SourceAccount {
    #[default]
    None,
    AccountId(AccountId)
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, Copy)]
pub struct OwnedColor (
    pub u32, // owner
    pub u32, // color hex
    pub u32 // miner
);

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, Copy)]
pub struct Color (
    pub u32, // color hex
    pub u32 // miner
);

const COUNTER: Symbol = symbol!("COUNTER");
const COLORS: Symbol = symbol!("COLORS");

pub struct ColorsContract;

#[contractimpl]
impl ColorsContract {

    pub fn mine(env: Env, colors: Map<u32, u32>, to: SourceAccount) {
        let miner_address = env.invoker();
        let miner_idx = get_account_idx(&env, &miner_address);

        let to_address = get_source_account(&env, to);
        let to_idx = get_account_idx(&env, &to_address);

        for (hex, amount) in colors.iter_unchecked() {
            let color = OwnedColor(to_idx, hex, miner_idx);

            env.events().publish((COLORS, symbol!("mine")), (&to_address, hex, &miner_address));

            let current_amount: u32 = env
                .storage()
                .get(&color)
                .unwrap_or(Ok(0))
                .unwrap();

            env
                .storage()
                .set(&color, current_amount + amount);
        }
    }

    pub fn xfer(env: Env, colors: Map<Color, u32>, to: SourceAccount) {
        let self_address = env.invoker();
        let self_idx = get_account_idx(&env, &self_address);

        let to_address = get_source_account(&env, to);
        let to_idx = get_account_idx(&env, &to_address);

        // TODO probably need an event here to help track what colors and account has

        for color in colors.iter_unchecked() {
            let (Color(hex, miner_idx), amount) = color;
            let from_color = OwnedColor(self_idx, hex, miner_idx);
            let current_from_amount: u32 = env
                .storage()
                .get(from_color)
                .unwrap_or(Ok(0))
                .unwrap();

            env
                .storage()
                .set(from_color, current_from_amount - amount);

            let to_color = OwnedColor(to_idx, hex, miner_idx);
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

    pub fn get_color(env: Env, hex: u32, miner: AccountId) -> u32 {
        let self_address = env.invoker();
        let self_idx = get_account_idx(&env, &self_address);

        let miner_address = Address::Account(miner);
        let miner_idx = get_account_idx(&env, &miner_address);

        env
            .storage()
            .get(OwnedColor(self_idx, hex, miner_idx))
            .unwrap_or(Ok(0))
            .unwrap()
    }
}

fn get_source_account(env: &Env, to: SourceAccount) -> Address {
    let source_account: Address;

    match to {
        SourceAccount::None => source_account = env.invoker(),
        SourceAccount::AccountId(account_id) => source_account = Address::Account(account_id),
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
            .get(COUNTER)
            .unwrap_or(Ok(0))
            .unwrap();

        account_idx += 1;

        env
            .storage()
            .set(COUNTER, account_idx);

        env
            .storage()
            .set(source_account, account_idx);
    }

    account_idx
}

mod test;