#![cfg(test)]

// use std::println;

use soroban_sdk::{testutils::Address as _, vec, Address, Env, Vec};

use crate::{
    token,
    colorglyph::{ColorGlyph, ColorGlyphClient},
    types::{MaybeAddress, MinerColorAmount},
};

// extern crate std;

#[test]
fn test() {
    let env = Env::default();

    // Contract
    let contract_id = env.register_contract(None, ColorGlyph);
    let client = ColorGlyphClient::new(&env, &contract_id);

    // Token
    let token_admin = Address::random(&env);
    let token_id = env.register_stellar_asset_contract(token_admin.clone());
    let token = token::Client::new(&env, &token_id);

    // Accounts
    let u1_address = Address::random(&env);
    let u2_address = Address::random(&env);
    let u3_address = Address::random(&env);
    let fee_address = Address::random(&env);

    token.mint(&token_admin, &u1_address, &10_000);
    token.mint(&token_admin, &u2_address, &10_000);
    token.mint(&token_admin, &u3_address, &10_000);

    client.init(&token_id, &fee_address);

    // Tests
    env.budget().reset();

    let mut colors: Vec<(u32, u32)> = Vec::new(&env);

    for i in 0..10 {
        colors.push_back((i, 1));
    }

    client.mine(&u1_address, &colors, &MaybeAddress::None);

    let color = client.get_color(&u1_address, &0, &u1_address);

    assert_eq!(color, 1);

    client.mine(
        &u2_address,
        &colors,
        &MaybeAddress::Address(u1_address.clone()),
    );

    let color1 = client.get_color(&u1_address, &0, &u1_address);
    let color2 = client.get_color(&u1_address, &0, &u2_address);

    assert_eq!(color1 + color2, 2);

    client.xfer(
        &u1_address,
        &vec![
            &env,
            MinerColorAmount(u1_address.clone(), 0, 1),
            MinerColorAmount(u2_address.clone(), 0, 1),
        ],
        &MaybeAddress::Address(u3_address.clone()),
    );

    let color1 = client.get_color(&u3_address, &0, &u1_address);
    let color2 = client.get_color(&u3_address, &0, &u2_address);

    assert_eq!(color1 + color2, 2);

    assert_eq!(token.balance(&u1_address), 10_000 - 10);
    assert_eq!(token.balance(&u2_address), 10_000 - 10);
    assert_eq!(token.balance(&fee_address), 20);
}
