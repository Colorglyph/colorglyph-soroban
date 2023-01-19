#![cfg(test)]

use std::println;

use soroban_auth::Identifier;
use soroban_sdk::{testutils::Accounts, vec, Env, Vec};
use stellar_xdr::Asset;

use crate::{
    colorglyph::{ColorGlyph, ColorGlyphClient},
    testutils::{generate_full_account, get_incr_allow_signature},
    token::Client as TokenClient,
    types::{MinerColorAmount, MaybeAccountId},
};

extern crate std;

#[test]
fn test() {
    let env = Env::default();

    // Contract
    let contract_id = env.register_contract(None, ColorGlyph);
    let contract_identifier = Identifier::Contract(contract_id.clone());
    let client = ColorGlyphClient::new(&env, &contract_id);

    // Accounts
    let (u1_keypair, _, u1_account_id, u1_identifier) = generate_full_account(&env);
    let (u2_keypair, _, u2_account_id, _) = generate_full_account(&env);
    let (_, _, u3_account_id, _) = generate_full_account(&env);

    let (_, _, _, fee_identifier) = generate_full_account(&env);

    // Token
    let token_id = env.register_stellar_asset_contract(Asset::Native);
    let token = TokenClient::new(&env, &token_id);

    client.init(&token_id, &fee_identifier);

    // Tests
    env.budget().reset();

    let mut colors: Vec<(u32, u32)> = Vec::new(&env);
    let mut pay_amount: i128 = 0;

    for i in 0..10 {
        pay_amount += 1;
        colors.push_back((i, 1));
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
        .mine(&signature, &colors, &MaybeAccountId::None);

    let color = client
        .with_source_account(&u1_account_id)
        .get_color(&0, &u1_account_id);

    assert_eq!(color, 1);

    let signature = get_incr_allow_signature(
        &env,
        &token_id,
        &u2_keypair,
        &token,
        &contract_identifier,
        &pay_amount,
    );

    client.with_source_account(&u2_account_id).mine(
        &signature,
        &colors,
        &MaybeAccountId::AccountId(u1_account_id.clone()),
    );

    let color1 = client
        .with_source_account(&u1_account_id)
        .get_color(&0, &u1_account_id);
    let color2 = client
        .with_source_account(&u1_account_id)
        .get_color(&0, &u2_account_id);

    assert_eq!(color1 + color2, 2);

    client.with_source_account(&u1_account_id).xfer(
        &vec![&env, MinerColorAmount(u1_account_id.clone(), 0, 1), MinerColorAmount(u2_account_id.clone(), 0, 1)],
        &MaybeAccountId::AccountId(u3_account_id.clone()),
    );

    let color1 = client
        .with_source_account(&u3_account_id)
        .get_color(&0, &u1_account_id);
    let color2 = client
        .with_source_account(&u3_account_id)
        .get_color(&0, &u2_account_id);

    assert_eq!(color1 + color2, 2);

    assert_eq!(token.balance(&u1_identifier), 10_000 - 10);
    assert_eq!(token.balance(&u1_identifier), 10_000 - 10);
    assert_eq!(token.balance(&fee_identifier), 10_020);
}
