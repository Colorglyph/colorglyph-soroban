#![cfg(test)]

// use std::println;
// extern crate std;

use crate::contract::{ColorGlyph, ColorGlyphClient};
use soroban_sdk::{testutils::Address as _, token, vec, Address, Env, Map};

#[test]
fn test() {
    let env = Env::default();

    env.mock_all_auths();
    env.budget().reset_unlimited();

    // Contract
    let contract_address = env.register_contract(None, ColorGlyph);
    let client = ColorGlyphClient::new(&env, &contract_address);

    // Token
    let token_admin = Address::generate(&env);
    let token_address = env.register_stellar_asset_contract(token_admin.clone());
    let token_admin_client = token::StellarAssetClient::new(&env, &token_address);
    let token_client = token::Client::new(&env, &token_address);

    // Accounts
    let u1_address = Address::generate(&env);
    let u2_address = Address::generate(&env);
    let u3_address = Address::generate(&env);
    let fee_address = Address::generate(&env);

    token_admin_client.mint(&u1_address, &10_000);
    token_admin_client.mint(&u2_address, &10_000);
    token_admin_client.mint(&u3_address, &10_000);

    client.initialize(&u1_address, &token_address, &fee_address, &1);

    // Tests
    let mut colors: Map<u32, u32> = Map::new(&env);

    for i in 0..=255 {
        colors.set(i, 1);
    }

    client.colors_mine(&u1_address, &colors, &None, &None);

    let color = client.color_balance(&u1_address.clone(), &0, &None);

    assert_eq!(color, 1);

    client.colors_mine(&u2_address, &colors, &None, &Some(u1_address.clone()));

    let color1 = client.color_balance(&u1_address.clone(), &0, &None);
    let color2 = client.color_balance(&u1_address.clone(), &0, &Option::Some(u2_address.clone()));

    assert_eq!(color1 + color2, 2);

    client.colors_transfer(
        &u1_address,
        &u3_address,
        &vec![&env, (u1_address.clone(), 0, 1), (u2_address.clone(), 0, 1)],
    );

    let color0 = client.color_balance(
        &u3_address.clone(),
        &u32::MAX,
        &Option::Some(u1_address.clone()),
    );
    let color1 = client.color_balance(&u3_address.clone(), &0, &Option::Some(u1_address.clone()));
    let color2 = client.color_balance(&u3_address.clone(), &0, &Option::Some(u2_address.clone()));

    assert_eq!(color0, 0); // ensure we test for colors that don't exist (getting and bumping non-existent values)
    assert_eq!(color1 + color2, 2);

    assert_eq!(token_client.balance(&u1_address), 10_000 - 256);
    assert_eq!(token_client.balance(&u2_address), 10_000 - 256);
    assert_eq!(token_client.balance(&fee_address), 512);

    // println!("{:?}", env.budget().print());
}
