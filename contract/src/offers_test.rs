#![cfg(test)]

use std::println;

use fixed_point_math::FixedPoint;
use soroban_auth::Identifier;
use soroban_sdk::{testutils::Logger, vec, Address, Env, Vec};
use stellar_xdr::Asset;

use crate::{
    colorglyph::{ColorGlyph, ColorGlyphClient},
    testutils::{generate_full_account, get_incr_allow_signature},
    token::Client as TokenClient,
    types::{AssetAmount, Error, MaybeAddress, MaybeSignature, OfferType, StorageKey},
};

extern crate std;

const ITERS: i128 = 10;

#[test]
fn test_buy_glyph() {
    let env = Env::default();

    // Contract
    let contract_id = env.register_contract(None, ColorGlyph);
    let contract_identifier = Identifier::Contract(contract_id.clone());
    let client = ColorGlyphClient::new(&env, &contract_id);

    // Accounts
    let (_, _, u1_account_id, u1_identifier, u1_address) = generate_full_account(&env);
    let (u2_keypair, _, u2_account_id, u2_identifier, _) = generate_full_account(&env);
    let (u3_keypair, _, u3_account_id, u3_identifier, u3_address) = generate_full_account(&env);

    let (_, _, _, fee_identifier, _) = generate_full_account(&env);

    // Token
    let token_id = env.register_stellar_asset_contract(Asset::Native);
    let token = TokenClient::new(&env, &token_id);

    client.init(&token_id, &fee_identifier);

    // Tests
    env.budget().reset();

    let mut colors_indexes: Vec<(u32, Vec<u32>)> = Vec::new(&env);
    let mut color_amount: Vec<(u32, u32)> = Vec::new(&env);
    let mut pay_amount: i128 = 0;

    for i in 0..ITERS {
        let hex = 16777215i128.fixed_div_floor(ITERS, i).unwrap(); // 0 - 16777215 (black to white)

        colors_indexes.push_back((hex as u32, vec![&env, i as u32]));
        color_amount.push_back((hex as u32, 1));
        pay_amount += 1;
    }

    let signature = get_incr_allow_signature(
        &env,
        &token_id,
        &u3_keypair,
        &token,
        &contract_identifier,
        &pay_amount,
    );

    client.with_source_account(&u3_account_id).mine(
        &signature,
        &color_amount,
        &MaybeAddress::Address(u1_address.clone()),
    );

    let hash = client
        .with_source_account(&u1_account_id)
        .make(&16, &vec![&env, (u3_address.clone(), colors_indexes)]);

    env.budget().reset();

    // Real Tests
    let amount: i128 = 16;
    let glyph = OfferType::Glyph(hash.clone());
    let asset = OfferType::Asset(AssetAmount(token_id.clone(), amount));

    let signature = get_incr_allow_signature(
        &env,
        &token_id,
        &u2_keypair,
        &token,
        &contract_identifier,
        &amount,
    );

    client.with_source_account(&u2_account_id).offer(
        &MaybeSignature::Signature(signature),
        &glyph,
        &asset,
    );

    client
        .with_source_account(&u1_account_id)
        .offer(&MaybeSignature::None, &asset, &glyph);

    env.as_contract(&contract_id, || {
        let res: Address = env
            .storage()
            .get(StorageKey::GlyphOwner(hash.clone()))
            .unwrap()
            .unwrap();

        assert_eq!(res, Address::Account(u2_account_id.clone()));
    });

    assert_eq!(
        client.try_get_offer(&glyph, &asset),
        Err(Ok(Error::NotFound))
    );

    assert_eq!(
        client.try_get_offer(&asset, &glyph),
        Err(Ok(Error::NotFound))
    );

    assert_eq!(token.balance(&contract_identifier), 0i128);
    assert_eq!(token.balance(&u1_identifier), 10_008i128);
    assert_eq!(token.balance(&u2_identifier), 9_984i128);
    assert_eq!(token.balance(&u3_identifier), 9_998i128);
}

#[test]
fn test_sell_glyph() {
    let env = Env::default();

    // Contract
    let contract_id = env.register_contract(None, ColorGlyph);
    let contract_identifier = Identifier::Contract(contract_id.clone());
    let client = ColorGlyphClient::new(&env, &contract_id);

    // Accounts
    let (_, _, u1_account_id, u1_identifier, u1_address) = generate_full_account(&env);
    let (u2_keypair, _, u2_account_id, u2_identifier, _) = generate_full_account(&env);
    let (u3_keypair, _, u3_account_id, u3_identifier, u3_address) = generate_full_account(&env);

    let (_, _, _, fee_identifier, _) = generate_full_account(&env);

    // Token
    let token_id = env.register_stellar_asset_contract(Asset::Native);
    let token = TokenClient::new(&env, &token_id);

    client.init(&token_id, &fee_identifier);

    // Tests
    env.budget().reset();

    let mut colors_indexes: Vec<(u32, Vec<u32>)> = Vec::new(&env);
    let mut color_amount: Vec<(u32, u32)> = Vec::new(&env);
    let mut pay_amount: i128 = 0;

    for i in 0..ITERS {
        let hex = 16777215i128.fixed_div_floor(ITERS, i).unwrap(); // 0 - 16777215 (black to white)

        colors_indexes.push_back((hex as u32, vec![&env, i as u32]));
        color_amount.push_back((hex as u32, 1));
        pay_amount += 1;
    }

    let signature = get_incr_allow_signature(
        &env,
        &token_id,
        &u3_keypair,
        &token,
        &contract_identifier,
        &pay_amount,
    );

    client.with_source_account(&u3_account_id).mine(
        &signature,
        &color_amount,
        &MaybeAddress::Address(u1_address.clone()),
    );

    let hash = client
        .with_source_account(&u1_account_id)
        .make(&16, &vec![&env, (u3_address.clone(), colors_indexes)]);

    env.budget().reset();

    // Real Tests
    let amount: i128 = 16;
    let glyph = OfferType::Glyph(hash.clone());
    let asset = OfferType::Asset(AssetAmount(token_id.clone(), amount));

    client
        .with_source_account(&u1_account_id)
        .offer(&MaybeSignature::None, &asset, &glyph);

    let signature = get_incr_allow_signature(
        &env,
        &token_id,
        &u2_keypair,
        &token,
        &contract_identifier,
        &amount,
    );

    client.with_source_account(&u2_account_id).offer(
        &MaybeSignature::Signature(signature),
        &glyph,
        &asset,
    );

    env.as_contract(&contract_id, || {
        let res: Address = env
            .storage()
            .get(StorageKey::GlyphOwner(hash.clone()))
            .unwrap()
            .unwrap();

        assert_eq!(res, Address::Account(u2_account_id.clone()));
    });

    assert_eq!(
        client.try_get_offer(&glyph, &asset),
        Err(Ok(Error::NotFound))
    );

    assert_eq!(
        client.try_get_offer(&asset, &glyph),
        Err(Ok(Error::NotFound))
    );

    println!("{:?}", env.logger().print());

    assert_eq!(token.balance(&contract_identifier), 0i128);
    assert_eq!(token.balance(&u1_identifier), 10_008i128);
    assert_eq!(token.balance(&u2_identifier), 9_984i128);
    assert_eq!(token.balance(&u3_identifier), 9_998i128);
}

#[test]
fn test_swap_glyph() {
    let env = Env::default();

    // Contract
    let contract_id = env.register_contract(None, ColorGlyph);
    let contract_identifier = Identifier::Contract(contract_id.clone());
    let client = ColorGlyphClient::new(&env, &contract_id);

    // Accounts
    let (u1_keypair, _, u1_account_id, _, u1_address) = generate_full_account(&env);
    let (u2_keypair, _, u2_account_id, _, u2_address) = generate_full_account(&env);

    let (_, _, _, fee_identifier, _) = generate_full_account(&env);

    // Token
    let token_id = env.register_stellar_asset_contract(Asset::Native);
    let token = TokenClient::new(&env, &token_id);

    client.init(&token_id, &fee_identifier);

    // Tests
    env.budget().reset();

    let mut colors_a_indexes: Vec<(u32, Vec<u32>)> = Vec::new(&env);
    let mut colors_b_indexes: Vec<(u32, Vec<u32>)> = Vec::new(&env);
    let mut color_amount: Vec<(u32, u32)> = Vec::new(&env);
    let mut pay_amount: i128 = 0;

    for i in 0..ITERS {
        let hex = 16777215i128.fixed_div_floor(ITERS, i).unwrap(); // 0 - 16777215 (black to white)

        colors_a_indexes.push_back((hex as u32, vec![&env, i as u32]));
        colors_b_indexes.push_front((hex as u32, vec![&env, i as u32]));
        color_amount.push_back((hex as u32, 1));
        pay_amount += 1;
    }

    let signature = get_incr_allow_signature(
        &env,
        &token_id,
        &u1_keypair,
        &token,
        &contract_identifier,
        &pay_amount,
    );

    client
        .with_source_account(&u1_account_id)
        .mine(&signature, &color_amount, &MaybeAddress::None);

    let hash_a = client
        .with_source_account(&u1_account_id)
        .make(&16, &vec![&env, (u1_address.clone(), colors_a_indexes)]);

    let signature = get_incr_allow_signature(
        &env,
        &token_id,
        &u2_keypair,
        &token,
        &contract_identifier,
        &pay_amount,
    );

    client
        .with_source_account(&u2_account_id)
        .mine(&signature, &color_amount, &MaybeAddress::None);

    let hash_b = client
        .with_source_account(&u2_account_id)
        .make(&16, &vec![&env, (u2_address.clone(), colors_b_indexes)]);

    env.budget().reset();

    // Real Tests
    let glyph_1 = OfferType::Glyph(hash_a.clone());
    let glyph_2 = OfferType::Glyph(hash_b.clone());

    client
        .with_source_account(&u1_account_id)
        .offer(&MaybeSignature::None, &glyph_2, &glyph_1);

    client
        .with_source_account(&u2_account_id)
        .offer(&MaybeSignature::None, &glyph_1, &glyph_2);

    env.as_contract(&contract_id, || {
        let res_a: Address = env
            .storage()
            .get(StorageKey::GlyphOwner(hash_a.clone()))
            .unwrap()
            .unwrap();

        assert_eq!(res_a, Address::Account(u2_account_id.clone()));

        let res_b: Address = env
            .storage()
            .get(StorageKey::GlyphOwner(hash_b.clone()))
            .unwrap()
            .unwrap();

        assert_eq!(res_b, Address::Account(u1_account_id.clone()));
    });

    assert_eq!(
        client.try_get_offer(&glyph_2, &glyph_1),
        Err(Ok(Error::NotFound))
    );

    assert_eq!(
        client.try_get_offer(&glyph_1, &glyph_2),
        Err(Ok(Error::NotFound))
    );
}

#[test]
fn test_rm_glyph_buy() {
    let env = Env::default();

    // Contract
    let contract_id = env.register_contract(None, ColorGlyph);
    let contract_identifier = Identifier::Contract(contract_id.clone());
    let client = ColorGlyphClient::new(&env, &contract_id);

    // Accounts
    let (u1_keypair, _, u1_account_id, u1_identifier, u1_address) = generate_full_account(&env);

    let (_, _, _, fee_identifier, _) = generate_full_account(&env);

    // Token
    let token_id = env.register_stellar_asset_contract(Asset::Native);
    let token = TokenClient::new(&env, &token_id);

    client.init(&token_id, &fee_identifier);

    // Tests
    env.budget().reset();

    let mut colors_indexes: Vec<(u32, Vec<u32>)> = Vec::new(&env);
    let mut color_amount: Vec<(u32, u32)> = Vec::new(&env);
    let mut pay_amount: i128 = 0;

    for i in 0..ITERS {
        let hex = 16777215i128.fixed_div_floor(ITERS, i).unwrap(); // 0 - 16777215 (black to white)

        colors_indexes.push_back((hex as u32, vec![&env, i as u32]));
        color_amount.push_back((hex as u32, 1));
        pay_amount += 1;
    }

    let signature = get_incr_allow_signature(
        &env,
        &token_id,
        &u1_keypair,
        &token,
        &contract_identifier,
        &pay_amount,
    );

    client
        .with_source_account(&u1_account_id)
        .mine(&signature, &color_amount, &MaybeAddress::None);

    let hash = client
        .with_source_account(&u1_account_id)
        .make(&16, &vec![&env, (u1_address.clone(), colors_indexes)]);

    env.budget().reset();

    // Real Tests
    let amount: i128 = 1;
    let glyph = OfferType::Glyph(hash.clone());
    let asset = OfferType::Asset(AssetAmount(token_id.clone(), amount));

    let signature = get_incr_allow_signature(
        &env,
        &token_id,
        &u1_keypair,
        &token,
        &contract_identifier,
        &amount,
    );

    client.with_source_account(&u1_account_id).offer(
        &MaybeSignature::Signature(signature),
        &glyph,
        &asset,
    );

    assert_eq!(token.balance(&contract_identifier), 1i128);

    assert_eq!(token.balance(&u1_identifier), 9_989i128);

    client.get_offer(&glyph, &asset);

    client
        .with_source_account(&u1_account_id)
        .rm_offer(&glyph, &asset);

    assert_eq!(
        client.try_get_offer(&glyph, &asset),
        Err(Ok(Error::NotFound))
    );

    assert_eq!(token.balance(&contract_identifier), 0i128);

    assert_eq!(token.balance(&u1_identifier), 9990i128);
}

#[test]
fn test_rm_glyph_sell() {
    let env = Env::default();

    // Contract
    let contract_id = env.register_contract(None, ColorGlyph);
    let contract_identifier = Identifier::Contract(contract_id.clone());
    let client = ColorGlyphClient::new(&env, &contract_id);

    // Accounts
    let (u1_keypair, _, u1_account_id, u1_identifier, u1_address) = generate_full_account(&env);

    let (_, _, _, fee_identifier, _) = generate_full_account(&env);

    // Token
    let token_id = env.register_stellar_asset_contract(Asset::Native);
    let token = TokenClient::new(&env, &token_id);

    client.init(&token_id, &fee_identifier);

    // Tests
    env.budget().reset();

    let mut colors_indexes: Vec<(u32, Vec<u32>)> = Vec::new(&env);
    let mut color_amount: Vec<(u32, u32)> = Vec::new(&env);
    let mut pay_amount: i128 = 0;

    for i in 0..ITERS {
        let hex = 16777215i128.fixed_div_floor(ITERS, i).unwrap(); // 0 - 16777215 (black to white)

        colors_indexes.push_back((hex as u32, vec![&env, i as u32]));
        color_amount.push_back((hex as u32, 1));
        pay_amount += 1;
    }

    let signature = get_incr_allow_signature(
        &env,
        &token_id,
        &u1_keypair,
        &token,
        &contract_identifier,
        &pay_amount,
    );

    client
        .with_source_account(&u1_account_id)
        .mine(&signature, &color_amount, &MaybeAddress::None);

    let hash = client
        .with_source_account(&u1_account_id)
        .make(&16, &vec![&env, (u1_address.clone(), colors_indexes)]);

    env.budget().reset();

    // Real Tests
    let glyph = OfferType::Glyph(hash.clone());
    let asset = OfferType::Asset(AssetAmount(token_id.clone(), 1i128));

    client
        .with_source_account(&u1_account_id)
        .offer(&MaybeSignature::None, &asset, &glyph);

    client.get_offer(&asset, &glyph);

    client
        .with_source_account(&u1_account_id)
        .rm_offer(&asset, &glyph);

    assert_eq!(
        client.try_get_offer(&asset, &glyph),
        Err(Ok(Error::NotFound))
    );

    assert_eq!(token.balance(&contract_identifier), 0i128);

    assert_eq!(token.balance(&u1_identifier), 9990i128);
}

#[test]
fn test_rm_glyph_swap() {
    let env = Env::default();

    // Contract
    let contract_id = env.register_contract(None, ColorGlyph);
    let contract_identifier = Identifier::Contract(contract_id.clone());
    let client = ColorGlyphClient::new(&env, &contract_id);

    // Accounts
    let (u1_keypair, _, u1_account_id, u1_identifier, u1_address) = generate_full_account(&env);
    let (_, _, u2_account_id, _, u2_address) = generate_full_account(&env);

    let (_, _, _, fee_identifier, _) = generate_full_account(&env);

    // Token
    let token_id = env.register_stellar_asset_contract(Asset::Native);
    let token = TokenClient::new(&env, &token_id);

    client.init(&token_id, &fee_identifier);

    // Tests
    env.budget().reset();

    let mut colors_a_indexes: Vec<(u32, Vec<u32>)> = Vec::new(&env);
    let mut colors_b_indexes: Vec<(u32, Vec<u32>)> = Vec::new(&env);
    let mut color_amount: Vec<(u32, u32)> = Vec::new(&env);
    let mut pay_amount: i128 = 0;

    for i in 0..ITERS {
        let hex = 16777215i128.fixed_div_floor(ITERS, i).unwrap(); // 0 - 16777215 (black to white)

        colors_a_indexes.push_back((hex as u32, vec![&env, i as u32]));
        colors_b_indexes.push_front((hex as u32, vec![&env, i as u32]));
        color_amount.push_back((hex as u32, 1));
        pay_amount += 1;
    }

    let signature = get_incr_allow_signature(
        &env,
        &token_id,
        &u1_keypair,
        &token,
        &contract_identifier,
        &pay_amount,
    );

    client
        .with_source_account(&u1_account_id)
        .mine(&signature, &color_amount, &MaybeAddress::None);

    let hash_a = client
        .with_source_account(&u1_account_id)
        .make(&16, &vec![&env, (u1_address.clone(), colors_a_indexes)]);

    let signature = get_incr_allow_signature(
        &env,
        &token_id,
        &u1_keypair,
        &token,
        &contract_identifier,
        &pay_amount,
    );

    client.with_source_account(&u1_account_id).mine(
        &signature,
        &color_amount,
        &MaybeAddress::Address(u2_address),
    );

    let hash_b = client
        .with_source_account(&u2_account_id)
        .make(&16, &vec![&env, (u1_address.clone(), colors_b_indexes)]);

    env.budget().reset();

    // Real Tests
    let glyph_a = OfferType::Glyph(hash_a.clone());
    let glyph_b = OfferType::Glyph(hash_b.clone());

    client
        .with_source_account(&u1_account_id)
        .offer(&MaybeSignature::None, &glyph_b, &glyph_a);

    client.get_offer(&glyph_b, &glyph_a);

    client
        .with_source_account(&u1_account_id)
        .rm_offer(&glyph_b, &glyph_a);

    assert_eq!(
        client.try_get_offer(&glyph_b, &glyph_a),
        Err(Ok(Error::NotFound))
    );

    assert_eq!(token.balance(&contract_identifier), 0i128);

    assert_eq!(token.balance(&u1_identifier), 9980i128);
}
