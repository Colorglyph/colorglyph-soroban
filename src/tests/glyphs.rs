#![cfg(test)]

use std::println;
extern crate std;

use crate::{
    contract::{ColorGlyph, ColorGlyphClient},
    types::Error,
};
use fixed_point_math::FixedPoint;
use soroban_sdk::{testutils::Address as _, token, vec, Address, Env, Vec};

const ITERS: i128 = 16i128.pow(2);

// TODO
// re-add the tests to ensure minting and scraping send the colors to the right places. Maintain the test on a 16x16 glyph though

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
    let fee_address = Address::random(&env);

    token.mint(&u1_address, &10_000);
    token.mint(&u2_address, &10_000);

    client.initialize(&token_id, &fee_address);

    // Tests
    let mut colors_indexes: Vec<(u32, Vec<u32>)> = Vec::new(&env);
    let mut color_amount: Vec<(u32, u32)> = Vec::new(&env);

    for i in 0..ITERS {
        let hex = 16777215i128.fixed_div_floor(ITERS, i).unwrap(); // 0 - 16777215 (black to white)

        colors_indexes.push_back((hex as u32, vec![&env, i as u32]));
        color_amount.push_back((hex as u32, 1));
    }

    // colors_indexes.push_back((0 as u32, vec![&env, 0, 2]));
    // colors_indexes.push_back((1 as u32, vec![&env, 1, 3]));
    // color_amount.push_back((0 as u32, 2));
    // color_amount.push_back((1 as u32, 2));

    env.budget().reset_default();
    client.colors_mine(&u1_address, &None, &color_amount);

    // let color = client.color_balance(&u1_address, &0, &u1_address);

    // assert_eq!(color, 1);
    // assert_eq!(color, 2);

    // Real Test
    env.budget().reset_default();
    let hash = client.glyph_mint(
        &u1_address,
        &Option::None,
        &vec![&env, (u1_address.clone(), colors_indexes.clone())],
        &16,
    );

    // env.budget().reset_default();
    // let color = client.color_balance(&u1_address, &0, &u1_address);

    // assert_eq!(color, 0);

    println!("{:?}", client.glyph_get(&hash));

    env.budget().reset_default();
    client.glyph_scrape(&u1_address, &Option::None, &hash);

    env.budget().reset_default();
    assert_eq!(client.try_glyph_get(&hash), Err(Ok(Error::NotFound)));

    // env.budget().reset_default();
    // let color = client.color_balance(&u1_address, &0, &u1_address);

    // assert_eq!(color, 1);
    // assert_eq!(color, 2);

    env.budget().reset_default();
    client.colors_mine(&u1_address, &Some(u2_address.clone()), &color_amount);

    env.budget().reset_default();
    let hash = client.glyph_mint(
        &u2_address,
        &Option::None,
        &vec![&env, (u1_address.clone(), colors_indexes.clone())],
        &16,
    );

    env.budget().reset_default();
    client.glyph_get(&hash);

    env.budget().reset_default();
    assert_eq!(
        client.try_glyph_scrape(&u1_address, &Option::None, &hash),
        Err(Ok(Error::NotAuthorized.into()))
    );
}
