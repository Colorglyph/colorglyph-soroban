#![cfg(test)]

// use std::println;
// extern crate std;

use crate::{
    contract::{ColorGlyph, ColorGlyphClient},
    misc_test::color_balance,
    types::MinerColorAmount,
};
use soroban_sdk::{testutils::Address as _, token, vec, Address, Env, Vec};

#[test]
fn test() {
    let env = Env::default();

    env.mock_all_auths();

    // Contract
    let contract_address = env.register_contract(None, ColorGlyph);
    let client = ColorGlyphClient::new(&env, &contract_address);

    // Token
    let token_admin = Address::random(&env);
    let token_id = env.register_stellar_asset_contract(token_admin.clone());
    let token = token::Client::new(&env, &token_id);

    // Accounts
    let u1_address = Address::random(&env);
    let u2_address = Address::random(&env);
    let u3_address = Address::random(&env);
    let fee_address = Address::random(&env);

    token.mint(&u1_address, &10_000);
    token.mint(&u2_address, &10_000);
    token.mint(&u3_address, &10_000);

    client.initialize(&token_id, &fee_address);

    // Tests
    let mut colors: Vec<(u32, u32)> = Vec::new(&env);

    for i in 0..=255 {
        colors.push_back((i, 1));
    }

    env.budget().reset_default();
    client.colors_mine(&u1_address, &None, &colors);

    let color = color_balance(&env, &contract_address, u1_address.clone(), Option::None, 0);

    assert_eq!(color, 1);

    env.budget().reset_default();
    client.colors_mine(&u2_address, &Some(u1_address.clone()), &colors);

    let color1 = color_balance(&env, &contract_address, u1_address.clone(), Option::None, 0);
    let color2 = color_balance(
        &env,
        &contract_address,
        u1_address.clone(),
        Option::Some(u2_address.clone()),
        0,
    );

    assert_eq!(color1 + color2, 2);

    client.colors_transfer(
        &u1_address,
        &u3_address,
        &vec![
            &env,
            MinerColorAmount(u1_address.clone(), 0, 1),
            MinerColorAmount(u2_address.clone(), 0, 1),
        ],
    );

    let color1 = color_balance(
        &env,
        &contract_address,
        u3_address.clone(),
        Option::Some(u1_address.clone()),
        0,
    );
    let color2 = color_balance(
        &env,
        &contract_address,
        u3_address.clone(),
        Option::Some(u2_address.clone()),
        0,
    );

    assert_eq!(color1 + color2, 2);

    assert_eq!(token.balance(&u1_address), 10_000 - 256);
    assert_eq!(token.balance(&u2_address), 10_000 - 256);
    assert_eq!(token.balance(&fee_address), 512);
}
