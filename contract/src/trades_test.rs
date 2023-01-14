#![cfg(test)]

use std::println;

use soroban_auth::{Identifier};
use soroban_sdk::{Env, Bytes, Vec, vec, symbol, Symbol};
use stellar_xdr::{Asset};

use crate::{
    colorglyph::{ColorGlyph, ColorGlyphClient},
    types::{AssetType, SourceAccount, Glyph, AssetAmount, DataKey, Error}, 
    testutils::{generate_full_account, get_incr_allow_signature},
    token::Client as TokenClient, 
};

extern crate std;

const ITER: u32 = 10;

#[test]
fn test() {
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
        u2_account_id, 
        _
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
        .mine(&signature, &color_amount, &SourceAccount::None);

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
    let asset_1 = AssetType::Asset(AssetAmount(token_id.clone(), 1i128));
    let asset_2 = AssetType::Glyph(hash.clone());
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
        .trade(&signature, &asset_2, &asset_1);

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
    let side = DataKey::SideBuy;

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
fn test_2() {
    // TODO: test this again https://github.com/stellar/rs-soroban-sdk/pull/827

    let env = Env::default();

    let unsorted = vec!(&env, 
        symbol!("hello"), 
        symbol!("world"),
        symbol!("tyler"), 
        symbol!("pizza"),
        symbol!("party"), 
        symbol!("groot"),
        symbol!("house"), 
        symbol!("mouse"),
        symbol!("trick"), 
        symbol!("juice"),
    );

    let mut sorted: Vec<Symbol> = Vec::new(&env);

    for v in unsorted.into_iter_unchecked() {
        let res = sorted.binary_search(v);

        match res {
            Result::Ok(_) => panic!("item exists"),
            Result::Err(i) => {
                println!("{}", i);
                if i == 0 {
                    sorted.push_front(v);
                } else if i == sorted.len() {
                    sorted.push_back(v);
                } else {
                    sorted.insert(i, v);
                }
            },
        }
    }

    // let res = v.binary_search(symbol!("pizza"));

    println!("{:?}", sorted);
}