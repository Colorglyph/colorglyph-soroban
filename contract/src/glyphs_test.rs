#![cfg(test)]

use std::println;

use fixed_point_math::FixedPoint;
use soroban_sdk::{testutils::Address as _, vec, Address, Env, Vec};

use crate::{
    colorglyph::{ColorGlyph, ColorGlyphClient},
    token,
    types::{Error, MaybeAddress},
};

extern crate std;

const ITERS: i128 = 256;

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
    let fee_address = Address::random(&env);

    token.mint(&token_admin, &u1_address, &10_000);
    token.mint(&token_admin, &u2_address, &10_000);

    client.init(&token_id, &fee_address);

    // Tests
    env.budget().reset();

    let mut colors_indexes: Vec<(u32, Vec<u32>)> = Vec::new(&env);
    let mut color_amount: Vec<(u32, u32)> = Vec::new(&env);

    // for i in 0..ITERS {
    //     let hex = 16777215i128.fixed_div_floor(ITERS, i).unwrap(); // 0 - 16777215 (black to white)

    //     colors_indexes.push_back((hex as u32, vec![&env, i as u32]));
    //     color_amount.push_back((hex as u32, 1));
    // }

    colors_indexes.push_back((0 as u32, vec![&env, 0, 2]));
    colors_indexes.push_back((1 as u32, vec![&env, 1, 3]));
    color_amount.push_back((0 as u32, 2));
    color_amount.push_back((1 as u32, 2));

    client.mine(&u1_address, &color_amount, &MaybeAddress::None);

    let color = client.get_color(&u1_address, &0, &u1_address);

    // assert_eq!(color, 1);
    assert_eq!(color, 2);

    env.budget().reset();

    // Real Test
    let hash = client.make(
        &u1_address,
        &16,
        &vec![&env, (u1_address.clone(), colors_indexes.clone())],
    );

    let color = client.get_color(&u1_address, &0, &u1_address);

    assert_eq!(color, 0);

    println!("{:?}", client.get_glyph(&hash));

    client.scrape(&u1_address, &hash);

    let res = client.try_get_glyph(&hash);

    assert_eq!(res, Err(Ok(Error::NotFound)));

    let color = client.get_color(&u1_address, &0, &u1_address);

    // assert_eq!(color, 1);
    assert_eq!(color, 2);

    client.mine(
        &u1_address,
        &color_amount,
        &MaybeAddress::Address(u2_address.clone()),
    );

    let hash = client.make(
        &u2_address,
        &16,
        &vec![&env, (u1_address.clone(), colors_indexes.clone())],
    );

    client.get_glyph(&hash);

    let res = client.try_scrape(&u1_address, &hash);

    assert_eq!(res, Err(Ok(Error::NotAuthorized.into())));
}
