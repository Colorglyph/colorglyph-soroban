#![cfg(test)]

use soroban_auth::{Signature, Identifier};
use soroban_sdk::{Env, Bytes, Vec, vec};
use stellar_xdr::{AlphaNum4, Asset, AssetCode4};

use crate::{
    colorglyph::{ColorGlyph, ColorGlyphClient}, 
    testutils::{generate_full_account, get_incr_allow_signature},
    token::Client as TokenClient, 
    types::{SourceAccount, Glyph, Error},
};

extern crate std;

const ITER: u32 = 255;

#[test]
fn test() {
    let env = Env::default();

    // Contract
    let contract_id = env.register_contract(None, ColorGlyph);
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
        issuer_xdr_account_id, 
        issuer_account_id,
        _,
    ) = generate_full_account(&env);

    let (
        _,
        _,
        _,
        fee_identifier
    ) = generate_full_account(&env);

    // Token
    let asset4 = Asset::CreditAlphanum4(AlphaNum4 {
        asset_code: AssetCode4([0u8; 4]),
        issuer: issuer_xdr_account_id.clone()
    });
    let token_id = env.register_stellar_asset_contract(asset4);
    let token = TokenClient::new(&env, &token_id);

    // Minting
    token
    .with_source_account(&issuer_account_id)
    .mint(
        &Signature::Invoker,
        &0,
        &u1_identifier,
        &10_000_000,
    );

    // Tests
    client.init(&token_id, &fee_identifier);

    let mut b_palette = Bytes::new(&env);
    let mut colors_indexes: Vec<(u32, Vec<u32>)> = Vec::new(&env);
    let mut color_amount: Vec<(u32, u32)> = Vec::new(&env);
    let mut pay_amount: i128 = 0;

    env.budget().reset();

    for i in 0..=ITER {
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
        &Identifier::Contract(contract_id.clone()),
        &pay_amount
    );

    env.budget().reset();

    client
        .with_source_account(&u1_account_id)
        .mine(&signature, &color_amount, &SourceAccount::None);

    let color = client
        .with_source_account(&u1_account_id)
        .get_color(&0, &u1_account_id);

    assert_eq!(color, 1);

    env.budget().reset();

    let hash = client
        .with_source_account(&u1_account_id)
        .mint(
            &Glyph{
                width: 16,
                colors: vec![&env,
                    (1, colors_indexes.clone())
                ]
            }
        );

    // - CPU Instructions: 23284840
    // - Memory Bytes: 3433113
    // println!("{}", env.budget());

    let color = client
        .with_source_account(&u1_account_id)
        .get_color(&0, &u1_account_id);

    assert_eq!(color, 0);

    env.budget().reset();

    client
        .with_source_account(&u1_account_id)
        .get_glyph(&hash);

    // - CPU Instructions: 1826840
    // - Memory Bytes: 224319
    // println!("{}", env.budget());

    env.budget().reset();

    client
        .with_source_account(&u1_account_id)
        .scrape(&hash);

    // - CPU Instructions: 17019100
    // - Memory Bytes: 2454954
    // println!("{}", env.budget());

    env.budget().reset();

    let res = client.try_get_glyph(&hash);

    assert_eq!(
        res,
        Err(Ok(Error::NotFound))
    );

    let color = client
        .with_source_account(&u1_account_id)
        .get_color(&0, &u1_account_id);

    assert_eq!(color, 1);

    let signature = get_incr_allow_signature(
        &env,
        &token_id, 
        &u1_keypair,
        &token,
        &Identifier::Contract(contract_id.clone()),
        &pay_amount
    );

    env.budget().reset();

    client
        .with_source_account(&u1_account_id)
        .mine(&signature, &color_amount, &SourceAccount::AccountId(u2_account_id.clone()));

    let hash = client
        .with_source_account(&u2_account_id)
        .mint(
            &Glyph{
                width: 16,
                colors: vec![&env,
                    (1, colors_indexes.clone())
                ]
            }
        );

    client
        .with_source_account(&u2_account_id)
        .get_glyph(&hash);

    env.budget().reset();

    let res = client
        .with_source_account(&u1_account_id)
        .try_scrape(&hash);

    assert_eq!(
        res,
        Err(Ok(Error::NotAuthorized.into()))
    );
}