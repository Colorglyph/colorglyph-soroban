#![cfg(test)]

// use std::println;
// extern crate std;

use crate::{
    colorglyph::{ColorGlyph, ColorGlyphClient},
    types::MinerColorAmount,
};
use soroban_sdk::{testutils::Address as _, token, vec, Address, Env, Vec};

#[test]
fn test() {
    let env = Env::default();

    env.mock_all_auths();

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

    token.mint(&u1_address, &10_000);
    token.mint(&u2_address, &10_000);
    token.mint(&u3_address, &10_000);

    client.init(&token_id, &fee_address);

    // Tests
    let mut colors: Vec<(u32, u32)> = Vec::new(&env);

    for i in 0..=255 {
        colors.push_back((i, 1));
    }

    env.budget().reset_default();
    client.mine(&u1_address, &colors, &None);

    let color = client.get_color(&u1_address, &0, &u1_address);

    assert_eq!(color, 1);

    env.budget().reset_default();
    client.mine(&u2_address, &colors, &Some(u1_address.clone()));

    let color1 = client.get_color(&u1_address, &0, &u1_address);
    let color2 = client.get_color(&u1_address, &0, &u2_address);

    assert_eq!(color1 + color2, 2);

    client.transfer(
        &u1_address,
        &vec![
            &env,
            MinerColorAmount(u1_address.clone(), 0, 1),
            MinerColorAmount(u2_address.clone(), 0, 1),
        ],
        &Some(u3_address.clone()),
    );

    let color1 = client.get_color(&u3_address, &0, &u1_address);
    let color2 = client.get_color(&u3_address, &0, &u2_address);

    assert_eq!(color1 + color2, 2);

    assert_eq!(token.balance(&u1_address), 10_000 - 256);
    assert_eq!(token.balance(&u2_address), 10_000 - 256);
    assert_eq!(token.balance(&fee_address), 512);
}
