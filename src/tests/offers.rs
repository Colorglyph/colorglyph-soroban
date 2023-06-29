#![cfg(test)]

// use std::println;
// extern crate std;

use fixed_point_math::FixedPoint;
use soroban_sdk::{testutils::Address as _, token, vec, Address, Env, Vec};

use crate::{
    contract::{ColorGlyph, ColorGlyphClient},
    types::{AssetAmount, Error, OfferType, StorageKey},
};

const ITERS: i128 = 10i128;

#[test]
fn test_buy_glyph() {
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
    env.budget().reset_default();

    let mut colors_indexes: Vec<(u32, Vec<u32>)> = Vec::new(&env);
    let mut color_amount: Vec<(u32, u32)> = Vec::new(&env);

    for i in 0..ITERS {
        let hex = 16777215i128.fixed_div_floor(ITERS, i).unwrap(); // 0 - 16777215 (black to white)

        colors_indexes.push_back((hex as u32, vec![&env, i as u32]));
        color_amount.push_back((hex as u32, 1));
    }

    client.colors_mine(&u3_address, &Some(u1_address.clone()), &color_amount);

    let hash = client.glyph_mint(
        &u1_address,
        &Option::None,
        &vec![&env, (u3_address.clone(), colors_indexes)],
        &16,
    );

    env.budget().reset_default();

    // Real Tests
    let amount: i128 = 100;
    let glyph = OfferType::Glyph(hash.clone());
    let asset = OfferType::Asset(AssetAmount(token_id.clone(), amount));

    env.budget().reset_default();

    client.offer_post(&u2_address, &asset, &glyph);

    client.offer_post(&u1_address, &glyph, &asset);

    // env.budget().print();

    env.as_contract(&contract_address, || {
        let res: Address = env
            .storage()
            .get(&StorageKey::GlyphOwner(hash.clone()))
            .unwrap()
            .unwrap();

        assert_eq!(res, u2_address);
    });

    assert_eq!(
        client.try_offers_get(&asset, &glyph),
        Err(Ok(Error::NotFound))
    );

    assert_eq!(
        client.try_offers_get(&glyph, &asset),
        Err(Ok(Error::NotFound))
    );

    assert_eq!(token.balance(&contract_address), 0i128);
    assert_eq!(token.balance(&u1_address), 10_098i128);
    assert_eq!(token.balance(&u2_address), 9_900i128);
    assert_eq!(token.balance(&u3_address), 9_992i128);
}

#[test]
fn test_sell_glyph() {
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
    env.budget().reset_default();

    let mut colors_indexes: Vec<(u32, Vec<u32>)> = Vec::new(&env);
    let mut color_amount: Vec<(u32, u32)> = Vec::new(&env);

    for i in 0..ITERS {
        let hex = 16777215i128.fixed_div_floor(ITERS, i).unwrap(); // 0 - 16777215 (black to white)

        colors_indexes.push_back((hex as u32, vec![&env, i as u32]));
        color_amount.push_back((hex as u32, 1));
    }

    client.colors_mine(&u3_address, &Some(u1_address.clone()), &color_amount);

    let hash = client.glyph_mint(
        &u1_address,
        &Option::None,
        &vec![&env, (u3_address.clone(), colors_indexes)],
        &16,
    );

    env.budget().reset_default();

    // Real Tests
    let amount: i128 = 100;
    let glyph = OfferType::Glyph(hash.clone());
    let asset = OfferType::Asset(AssetAmount(token_id.clone(), amount));

    client.offer_post(&u1_address, &glyph, &asset);

    client.offer_post(&u2_address, &asset, &glyph);

    env.as_contract(&contract_address, || {
        let res: Address = env
            .storage()
            .get(&StorageKey::GlyphOwner(hash.clone()))
            .unwrap()
            .unwrap();

        assert_eq!(res, u2_address);
    });

    assert_eq!(
        client.try_offers_get(&asset, &glyph),
        Err(Ok(Error::NotFound))
    );

    assert_eq!(
        client.try_offers_get(&glyph, &asset),
        Err(Ok(Error::NotFound))
    );

    // env.logger().print();

    assert_eq!(token.balance(&contract_address), 0i128);
    assert_eq!(token.balance(&u1_address), 10_098i128);
    assert_eq!(token.balance(&u2_address), 9_900i128);
    assert_eq!(token.balance(&u3_address), 9_992i128);
}

#[test]
fn test_swap_glyph() {
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
    env.budget().reset_default();

    let mut colors_a_indexes: Vec<(u32, Vec<u32>)> = Vec::new(&env);
    let mut colors_b_indexes: Vec<(u32, Vec<u32>)> = Vec::new(&env);
    let mut colors_a_amount: Vec<(u32, u32)> = Vec::new(&env);
    let mut colors_b_amount: Vec<(u32, u32)> = Vec::new(&env);

    for i in 0..ITERS {
        let hex_a = 16777215i128.fixed_div_floor(ITERS, i).unwrap(); // 0 - 16777215 (black to white)
        let hex_b = 16777215i128.fixed_div_floor(ITERS, i + 1).unwrap();

        colors_a_indexes.push_back((hex_a as u32, vec![&env, i as u32]));
        colors_b_indexes.push_back((hex_b as u32, vec![&env, i as u32]));
        colors_a_amount.push_back((hex_a as u32, 1));
        colors_b_amount.push_back((hex_b as u32, 1));
    }

    client.colors_mine(&u1_address, &None, &colors_a_amount);

    let hash_a = client.glyph_mint(
        &u1_address,
        &Option::None,
        &vec![&env, (u1_address.clone(), colors_a_indexes)],
        &16,
    );

    client.colors_mine(&u2_address, &None, &colors_b_amount);

    let hash_b = client.glyph_mint(
        &u2_address,
        &Option::None,
        &vec![&env, (u2_address.clone(), colors_b_indexes)],
        &16,
    );

    env.budget().reset_default();

    // Real Tests
    let glyph_1 = OfferType::Glyph(hash_a.clone());
    let glyph_2 = OfferType::Glyph(hash_b.clone());

    client.offer_post(&u1_address, &glyph_1, &glyph_2);

    client.offer_post(&u2_address, &glyph_2, &glyph_1);

    env.as_contract(&contract_id, || {
        let res_a: Address = env
            .storage()
            .get(&StorageKey::GlyphOwner(hash_a.clone()))
            .unwrap()
            .unwrap();

        assert_eq!(res_a, u2_address);

        let res_b: Address = env
            .storage()
            .get(&StorageKey::GlyphOwner(hash_b.clone()))
            .unwrap()
            .unwrap();

        assert_eq!(res_b, u1_address);
    });

    assert_eq!(
        client.try_offers_get(&glyph_1, &glyph_2),
        Err(Ok(Error::NotFound))
    );

    assert_eq!(
        client.try_offers_get(&glyph_2, &glyph_1),
        Err(Ok(Error::NotFound))
    );
}

#[test]
fn test_rm_glyph_buy() {
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
    let fee_address = Address::random(&env);

    token.mint(&u1_address, &10_000);

    client.initialize(&token_id, &fee_address);

    // Tests
    env.budget().reset_default();

    let mut colors_indexes: Vec<(u32, Vec<u32>)> = Vec::new(&env);
    let mut color_amount: Vec<(u32, u32)> = Vec::new(&env);

    for i in 0..ITERS {
        let hex = 16777215i128.fixed_div_floor(ITERS, i).unwrap(); // 0 - 16777215 (black to white)

        colors_indexes.push_back((hex as u32, vec![&env, i as u32]));
        color_amount.push_back((hex as u32, 1));
    }

    client.colors_mine(&u1_address, &None, &color_amount);

    let hash = client.glyph_mint(
        &u1_address,
        &Option::None,
        &vec![&env, (u1_address.clone(), colors_indexes)],
        &16,
    );

    env.budget().reset_default();

    // Real Tests
    let amount: i128 = 1;
    let glyph = OfferType::Glyph(hash.clone());
    let asset = OfferType::Asset(AssetAmount(token_id.clone(), amount));

    client.offer_post(&u1_address, &asset, &glyph);

    assert_eq!(token.balance(&contract_address), 1i128);

    assert_eq!(token.balance(&u1_address), 9_989i128);

    client.offers_get(&asset, &glyph);

    client.offer_delete(&u1_address, &asset, &glyph);

    assert_eq!(
        client.try_offers_get(&asset, &glyph),
        Err(Ok(Error::NotFound))
    );

    assert_eq!(token.balance(&contract_address), 0i128);

    assert_eq!(token.balance(&u1_address), 9990i128);
}

#[test]
fn test_rm_glyph_sell() {
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
    let fee_address = Address::random(&env);

    token.mint(&u1_address, &10_000);

    client.initialize(&token_id, &fee_address);

    // Tests
    env.budget().reset_default();

    let mut colors_indexes: Vec<(u32, Vec<u32>)> = Vec::new(&env);
    let mut color_amount: Vec<(u32, u32)> = Vec::new(&env);

    for i in 0..ITERS {
        let hex = 16777215i128.fixed_div_floor(ITERS, i).unwrap(); // 0 - 16777215 (black to white)

        colors_indexes.push_back((hex as u32, vec![&env, i as u32]));
        color_amount.push_back((hex as u32, 1));
    }

    client.colors_mine(&u1_address, &None, &color_amount);

    let hash = client.glyph_mint(
        &u1_address,
        &Option::None,
        &vec![&env, (u1_address.clone(), colors_indexes)],
        &16,
    );

    env.budget().reset_default();

    // Real Tests
    let glyph = OfferType::Glyph(hash.clone());
    let asset = OfferType::Asset(AssetAmount(token_id.clone(), 1i128));

    client.offer_post(&u1_address, &glyph, &asset);

    client.offers_get(&glyph, &asset);

    client.offer_delete(&u1_address, &glyph, &asset);

    assert_eq!(
        client.try_offers_get(&glyph, &asset),
        Err(Ok(Error::NotFound))
    );

    assert_eq!(token.balance(&contract_address), 0i128);

    assert_eq!(token.balance(&u1_address), 9990i128);
}

#[test]
fn test_rm_glyph_swap() {
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
    let fee_address = Address::random(&env);

    token.mint(&u1_address, &10_000);
    token.mint(&u2_address, &10_000);

    client.initialize(&token_id, &fee_address);

    // Tests
    env.budget().reset_default();

    let mut colors_a_indexes: Vec<(u32, Vec<u32>)> = Vec::new(&env);
    let mut colors_b_indexes: Vec<(u32, Vec<u32>)> = Vec::new(&env);
    let mut colors_a_amount: Vec<(u32, u32)> = Vec::new(&env);
    let mut colors_b_amount: Vec<(u32, u32)> = Vec::new(&env);

    for i in 0..ITERS {
        let hex_a = 16777215i128.fixed_div_floor(ITERS, i).unwrap(); // 0 - 16777215 (black to white)
        let hex_b = 16777215i128.fixed_div_floor(ITERS, i + 1).unwrap();

        colors_a_indexes.push_back((hex_a as u32, vec![&env, i as u32]));
        colors_b_indexes.push_back((hex_b as u32, vec![&env, i as u32]));
        colors_a_amount.push_back((hex_a as u32, 1));
        colors_b_amount.push_back((hex_b as u32, 1));
    }

    client.colors_mine(&u1_address, &None, &colors_a_amount);

    let hash_a = client.glyph_mint(
        &u1_address,
        &Option::None,
        &vec![&env, (u1_address.clone(), colors_a_indexes)],
        &16,
    );

    client.colors_mine(&u1_address, &Some(u2_address.clone()), &colors_b_amount);

    let hash_b = client.glyph_mint(
        &u2_address,
        &Option::None,
        &vec![&env, (u1_address.clone(), colors_b_indexes)],
        &16,
    );

    env.budget().reset_default();

    // Real Tests
    let glyph_a = OfferType::Glyph(hash_a.clone());
    let glyph_b = OfferType::Glyph(hash_b.clone());

    client.offer_post(&u1_address, &glyph_a, &glyph_b);

    client.offers_get(&glyph_a, &glyph_b);

    client.offer_delete(&u1_address, &glyph_a, &glyph_b);

    assert_eq!(
        client.try_offers_get(&glyph_a, &glyph_b),
        Err(Ok(Error::NotFound))
    );

    assert_eq!(token.balance(&contract_address), 0i128);

    assert_eq!(token.balance(&u1_address), 9980i128);
}
