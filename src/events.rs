use soroban_sdk::{symbol_short, Address, Env, Map, Vec};

pub fn colors_mine(env: &Env, miner: &Address, to: &Address, colors: Map<u32, u32>) {
    env.events()
        .publish((symbol_short!("mine"), miner, to), colors);
}

pub fn colors_transfer(env: &Env, from: &Address, to: &Address, colors:  Vec<(Address, u32, u32)>) {
    env.events().publish(
        (symbol_short!("transfer"), from, to),
        colors,
    );
}
