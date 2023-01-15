#![cfg(test)]

use soroban_auth::{Identifier};
use soroban_sdk::{Env, Bytes, Vec, vec};
use stellar_xdr::{Asset};

use crate::{
    colorglyph::{ColorGlyph, ColorGlyphClient}, 
    testutils::{generate_full_account, get_incr_allow_signature},
    token::Client as TokenClient, 
    types::{MaybeAccountId, Glyph, Error},
};

extern crate std;

const ITER: u32 = 256;

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
        _,
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
    let mut color_amount: Vec<(u32, i128)> = Vec::new(&env);
    let mut pay_amount: i128 = 0;

    for i in 0..ITER {
        let hex = 16777215 / ITER * i; // 0 - 16777215 (black to white)

        colors_indexes.push_back((hex, vec![&env, i]));
        color_amount.push_back((hex, 1));
        pay_amount += 1;

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

    let color = client
        .with_source_account(&u1_account_id)
        .get_color(&0, &u1_account_id);

    assert_eq!(color, 1);

    env.budget().reset();

    // Real Test
    let hash = client
        .with_source_account(&u1_account_id)
        .make(
            &Glyph{
                width: 16,
                colors: vec![&env,
                    (1, colors_indexes.clone())
                ]
            }
        );

    let color = client
        .with_source_account(&u1_account_id)
        .get_color(&0, &u1_account_id);

    assert_eq!(color, 0);

    client
        .with_source_account(&u1_account_id)
        .get_glyph(&hash);

    client
        .with_source_account(&u1_account_id)
        .scrape(&hash);

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
        &contract_identifier,
        &pay_amount
    );

    client
        .with_source_account(&u1_account_id)
        .mine(&signature, &color_amount, &MaybeAccountId::AccountId(u2_account_id.clone()));

    let hash = client
        .with_source_account(&u2_account_id)
        .make(
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

    let res = client
        .with_source_account(&u1_account_id)
        .try_scrape(&hash);

    assert_eq!(
        res,
        Err(Ok(Error::NotAuthorized.into()))
    );
}