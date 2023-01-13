#![cfg(test)]

use soroban_auth::{Signature, Identifier};
use soroban_sdk::{Env, Vec, testutils::Accounts, vec};
use stellar_xdr::{AlphaNum4, Asset, AssetCode4};

use crate::{
    colorglyph::{ColorGlyph, ColorGlyphClient}, 
    testutils::{generate_full_account, get_incr_allow_signature}, 
    token::Client as TokenClient, 
    types::{SourceAccount, ColorAmount, Color}
};

extern crate std;

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
        u1_identifier
    ) = generate_full_account(&env);

    let (
        u2_keypair, 
        _, 
        u2_account_id, 
        u2_identifier
    ) = generate_full_account(&env);

    let (
        _,
        issuer_xdr_account_id, 
        issuer_account_id, 
        _
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

    token
    .with_source_account(&issuer_account_id)
    .mint(
        &Signature::Invoker,
        &0,
        &u2_identifier,
        &10_000_000,
    );

    // Tests
    client.init(&token_id, &fee_identifier);
    
    let mut colors: Vec<(u32, u32)> = Vec::new(&env);
    let mut pay_amount: i128 = 0;

    for i in 0..10 {
        pay_amount += 1 as i128;
        colors.push_back((i, 1));
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
        .mine(&signature, &colors, &SourceAccount::None);
    
    let color = client
        .with_source_account(&u1_account_id)
        .get_color(&0, &u1_account_id);

    // println!("{}", env.budget());

    assert_eq!(color, 1);

    let signature = get_incr_allow_signature(
        &env, 
        &token_id, 
        &u2_keypair,
        &token,
        &Identifier::Contract(contract_id.clone()),
        &pay_amount
    );

    env.budget().reset();

    client
        .with_source_account(&u2_account_id)
        .mine(&signature, &colors, &SourceAccount::AccountId(u1_account_id.clone()));

    let color1 = client
        .with_source_account(&u1_account_id)
        .get_color(&0, &u1_account_id);
    let color2 = client
        .with_source_account(&u1_account_id)
        .get_color(&0, &u2_account_id);

    assert_eq!(color1 + color2, 2);

    let u3 = env.accounts().generate();

    env.budget().reset();

    client
        .with_source_account(&u1_account_id)
        .xfer(
            &vec![&env, 
                ColorAmount(Color(0, 1), 1), 
                ColorAmount(Color(0, 2), 1)
            ], 
            &SourceAccount::AccountId(u3.clone())
        );
        
    let color1 = client
        .with_source_account(&u3)
        .get_color(&0, &u1_account_id);
    let color2 = client
        .with_source_account(&u3)
        .get_color(&0, &u2_account_id);

    assert_eq!(color1 + color2, 2);

    // assert_eq!(token.balance(&u1_identifier), 10_000_000 - 10);
    // assert_eq!(token.balance(&u1_identifier), 10_000_000 - 10);
    // assert_eq!(token.balance(&fee_identifier), 20);
}