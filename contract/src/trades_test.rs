#![cfg(test)]

use std::{println};

use soroban_auth::{Identifier};
use soroban_sdk::{Env, Bytes, Vec, vec, BytesN, testutils::BytesN as UtilsBytesN};
use stellar_xdr::{Asset};

use crate::{
    colorglyph::{ColorGlyph, ColorGlyphClient},
    types::{AssetType, MaybeAccountId, Glyph, AssetAmount, Error, MaybeSignature, Side}, 
    testutils::{generate_full_account, get_incr_allow_signature},
    token::Client as TokenClient, 
};

extern crate std;

const ITER: u32 = 10;

#[test]
fn test_trade_buy_glyph() {
    let env = Env::default();

    // Contract
    let contract_id = env.register_contract(None, ColorGlyph);
    let contract_identifier = Identifier::Contract(contract_id.clone());
    let client = ColorGlyphClient::new(&env, &contract_id);

    // Accounts
    let (
        u1_keypair, 
        _, 
        u1_account_id, 
        u1_identifier,
    ) = generate_full_account(&env);

    let (
        u2_keypair, 
        _, 
        u2_account_id, 
        u2_identifier,
    ) = generate_full_account(&env);

    let (
        _,
        _,
        _,
        fee_identifier
    ) = generate_full_account(&env);

    // Token
    let token_id = env.register_stellar_asset_contract(Asset::Native);
    let token = TokenClient::new(&env, &token_id);

    client.init(&token_id, &fee_identifier);

    // Tests
    env.budget().reset();

    let mut b_palette = Bytes::new(&env);
    let mut colors_indexes: Vec<(u32, Vec<u32>)> = Vec::new(&env);
    let mut color_amount: Vec<(u32, u32)> = Vec::new(&env);
    let mut pay_amount: i128 = 0;

    for i in 0..ITER {
        let hex = 16777215 / ITER * i; // 0 - 16777215 (black to white)

        colors_indexes.push_back((hex, vec![&env, i]));
        color_amount.push_back((hex, 1));
        pay_amount += 1 as i128;

        b_palette.insert_from_array(i * 4, &hex.to_le_bytes());
    }

    let signature = get_incr_allow_signature(
        &env, 
        &token_id, 
        &u1_keypair,
        &token,
        &contract_identifier,
        &pay_amount
    );

    client
        .with_source_account(&u1_account_id)
        .mine(&signature, &color_amount, &MaybeAccountId::None);

    let hash = client
        .with_source_account(&u1_account_id)
        .mint(
            &Glyph{
                width: 16,
                colors: vec![&env,
                    (1, colors_indexes)
                ]
            }
        );

    env.budget().reset();

    // Real Tests
    let asset_1 = AssetType::Glyph(hash.clone());
    let asset_2 = AssetType::Asset(AssetAmount(token_id.clone(), 1i128));
    let amount: i128 = 1;

    client
        .with_source_account(&u1_account_id)
        .trade(&MaybeSignature::None, &asset_2, &asset_1);

    let signature = get_incr_allow_signature(
        &env, 
        &token_id, 
        &u2_keypair,
        &token,
        &contract_identifier,
        &amount
    );

    client
        .with_source_account(&u2_account_id)
        .trade(&MaybeSignature::Signature(signature), &asset_1, &asset_2);
}

#[test]
fn test_trade_sell_glyph() {
    let env = Env::default();

    // Contract
    let contract_id = env.register_contract(None, ColorGlyph);
    let contract_identifier = Identifier::Contract(contract_id.clone());
    let client = ColorGlyphClient::new(&env, &contract_id);

    // Accounts
    let (
        u1_keypair, 
        _, 
        u1_account_id, 
        u1_identifier,
    ) = generate_full_account(&env);

    let (
        u2_keypair, 
        _, 
        u2_account_id, 
        u2_identifier,
    ) = generate_full_account(&env);

    let (
        _,
        _,
        _,
        fee_identifier
    ) = generate_full_account(&env);

    // Token
    let token_id = env.register_stellar_asset_contract(Asset::Native);
    let token = TokenClient::new(&env, &token_id);

    client.init(&token_id, &fee_identifier);

    // Tests
    env.budget().reset();

    let mut b_palette = Bytes::new(&env);
    let mut colors_indexes: Vec<(u32, Vec<u32>)> = Vec::new(&env);
    let mut color_amount: Vec<(u32, u32)> = Vec::new(&env);
    let mut pay_amount: i128 = 0;

    for i in 0..ITER {
        let hex = 16777215 / ITER * i; // 0 - 16777215 (black to white)

        colors_indexes.push_back((hex, vec![&env, i]));
        color_amount.push_back((hex, 1));
        pay_amount += 1 as i128;

        b_palette.insert_from_array(i * 4, &hex.to_le_bytes());
    }

    let signature = get_incr_allow_signature(
        &env, 
        &token_id, 
        &u1_keypair,
        &token,
        &contract_identifier,
        &pay_amount
    );

    client
        .with_source_account(&u1_account_id)
        .mine(&signature, &color_amount, &MaybeAccountId::None);

    let hash = client
        .with_source_account(&u1_account_id)
        .mint(
            &Glyph{
                width: 16,
                colors: vec![&env,
                    (1, colors_indexes)
                ]
            }
        );

    env.budget().reset();

    // Real Tests
    let asset_1 = AssetType::Glyph(hash.clone());
    let asset_2 = AssetType::Asset(AssetAmount(token_id.clone(), 1i128));
    let amount: i128 = 1;

    let signature = get_incr_allow_signature(
        &env, 
        &token_id, 
        &u2_keypair,
        &token,
        &contract_identifier,
        &amount
    );

    client
        .with_source_account(&u2_account_id)
        .trade(&MaybeSignature::Signature(signature), &asset_1, &asset_2);

    client
    .with_source_account(&u1_account_id)
    .trade(&MaybeSignature::None, &asset_2, &asset_1);
}

#[test]
fn test_trade_rm() {
    let env = Env::default();

    // Contract
    let contract_id = env.register_contract(None, ColorGlyph);
    let contract_identifier = Identifier::Contract(contract_id.clone());
    let client = ColorGlyphClient::new(&env, &contract_id);

    // Accounts
    let (
        u1_keypair, 
        _, 
        u1_account_id, 
        u1_identifier,
    ) = generate_full_account(&env);

    let (
        _,
        _,
        _,
        fee_identifier
    ) = generate_full_account(&env);

    // Token
    let token_id = env.register_stellar_asset_contract(Asset::Native);
    let token = TokenClient::new(&env, &token_id);

    client.init(&token_id, &fee_identifier);

    // Tests
    env.budget().reset();

    let mut b_palette = Bytes::new(&env);
    let mut colors_indexes: Vec<(u32, Vec<u32>)> = Vec::new(&env);
    let mut color_amount: Vec<(u32, u32)> = Vec::new(&env);
    let mut pay_amount: i128 = 0;

    for i in 0..ITER {
        let hex = 16777215 / ITER * i; // 0 - 16777215 (black to white)

        colors_indexes.push_back((hex, vec![&env, i]));
        color_amount.push_back((hex, 1));
        pay_amount += 1 as i128;

        b_palette.insert_from_array(i * 4, &hex.to_le_bytes());
    }

    let signature = get_incr_allow_signature(
        &env, 
        &token_id, 
        &u1_keypair,
        &token,
        &contract_identifier,
        &pay_amount
    );

    client
        .with_source_account(&u1_account_id)
        .mine(&signature, &color_amount, &MaybeAccountId::None);

    let hash = client
        .with_source_account(&u1_account_id)
        .mint(
            &Glyph{
                width: 16,
                colors: vec![&env,
                    (1, colors_indexes)
                ]
            }
        );

    env.budget().reset();

    // Real Tests
    let asset_1 = AssetType::Glyph(hash.clone());
    let asset_2 = AssetType::Asset(AssetAmount(token_id.clone(), 1i128));
    let amount: i128 = 1;

    let signature = get_incr_allow_signature(
        &env, 
        &token_id, 
        &u1_keypair,
        &token,
        &contract_identifier,
        &amount
    );

    client
        .with_source_account(&u1_account_id)
        .trade(&MaybeSignature::Signature(signature), &asset_1, &asset_2);

    assert_eq!(
        token
            .balance(&contract_identifier), 
        1i128
    );

    assert_eq!(
        token
            .balance(&u1_identifier), 
        9_989i128
    );

    let buy_hash = hash;
    let sell_hash = token_id;
    let side = Side::Buy;

    client.get_trade(&buy_hash, &sell_hash, &1i128, &side);

    client
        .with_source_account(&u1_account_id)
        .rm_trade(&buy_hash, &sell_hash, &1i128, &side);

    let res = client.try_get_trade(&buy_hash, &sell_hash, &1i128, &side);

    assert_eq!(
        res,
        Err(Ok(Error::NotFound.into()))
    );

    assert_eq!(
        token
            .balance(&contract_identifier), 
        0i128
    );

    assert_eq!(
        token
            .balance(&u1_identifier), 
        9990i128
    );
}

#[test]
fn test_binary_vs_index() {
    let env = Env::default();

    let item = AssetAmount(BytesN::random(&env), 10i128);
    let mut unsorted: Vec<AssetAmount> = Vec::new(&env);
    let mut binary_sorted: Vec<AssetAmount> = Vec::new(&env);
    let mut index_sorted: Vec<AssetAmount> = Vec::new(&env);

    for i in 0..10 {
        unsorted.push_back(AssetAmount(BytesN::random(&env), i as i128));
    }

    unsorted.push_back(item.clone());

    env.budget().reset();

    for v in unsorted.clone().into_iter_unchecked() {
        match binary_sorted.binary_search(&v) {
            Result::Err(i) => binary_sorted.insert(i, v),
            _ => ()
        }
    }

    // - CPU Instructions: 563584
    // - Memory Bytes: 44879
    println!("binary build {:?}", env.budget().print());

    env.budget().reset();

    for v in unsorted.clone().into_iter_unchecked() {
        index_sorted.push_back(v);
    }

    // - CPU Instructions: 413640
    // - Memory Bytes: 25818
    println!("index build {:?}", env.budget().print());

    env.budget().reset();

    let index = binary_sorted.binary_search(&item).unwrap();
    let res = binary_sorted.get(index).unwrap().unwrap();

    // - CPU Instructions: 17458
    // - Memory Bytes: 3551
    println!("binary get {:?}", env.budget().print());

    env.budget().reset();

    let index = index_sorted.first_index_of(&item).unwrap();
    let res = index_sorted.get(index).unwrap().unwrap();

    // - CPU Instructions: 24582
    // - Memory Bytes: 6351
    println!("index get {:?}", env.budget().print());
}