#![cfg(test)]

use super::*;
use std::println;
use ed25519_dalek::Keypair;
use rand::thread_rng;
use soroban_auth::{Ed25519Signature, SignaturePayload, SignaturePayloadV0};
use soroban_sdk::{BytesN, Env, testutils::Accounts, AccountId, vec, IntoVal, TryIntoVal, symbol, map, testutils::ed25519::Sign, xdr::Asset};
use stellar_xdr::{AlphaNum4, AssetCode4};
use crate::{types::Color, token::Client};

extern crate std;

fn generate_keypair() -> Keypair {
    Keypair::generate(&mut thread_rng())
}

fn to_ed25519(e: &Env, kp: &Keypair) -> Identifier {
    Identifier::Ed25519(kp.public.to_bytes().into_val(e))
}

// TODO: can we simplify this with tdep's sign testutil?
fn get_auth(
    env: &Env, 
    token_id: &BytesN<32>, 
    from_keypair: &Keypair, 
    from_identifier: &Identifier, 
    token: &Client,
    to_identifier: &Identifier, 
    amount: &i128
) -> Signature {
    let msg = SignaturePayload::V0(SignaturePayloadV0 {
        name: symbol!("incr_allow"),
        contract: token_id.clone(),
        network: env.ledger().network_passphrase(),
        args: (
            from_identifier, 
            token.nonce(from_identifier),
            to_identifier,
            amount
        ).into_val(env),
    });

    let auth = Signature::Ed25519(Ed25519Signature {
        public_key: from_keypair.public.to_bytes().into_val(env),
        signature: from_keypair.sign(msg).unwrap().into_val(env),
    });

    auth
}

#[test]
fn mine_test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, ColorGlyph);
    let client = ColorGlyphClient::new(&env, &contract_id);

    let u1_keypair = generate_keypair();
    let u1_account_id = stellar_xdr::AccountId(
        stellar_xdr::PublicKey::PublicKeyTypeEd25519(
            stellar_xdr::Uint256(
                *u1_keypair.public.as_bytes()
            )
        )
    )
    .try_into_val(&env)
    .unwrap();
    let u1_identifier = to_ed25519(&env, &u1_keypair);

    let u2_keypair = generate_keypair();
    let u2_account_id = stellar_xdr::AccountId(
        stellar_xdr::PublicKey::PublicKeyTypeEd25519(
            stellar_xdr::Uint256(
                *u2_keypair.public.as_bytes()
            )
        )
    )
    .try_into_val(&env)
    .unwrap();
    let u2_identifier = to_ed25519(&env, &u2_keypair);
    
    let u3 = env.accounts().generate();

    // let token_admin = env.accounts().generate();

    let issuer_keypair = generate_keypair();
    let issuer_xdr_account_id = stellar_xdr::AccountId(
        stellar_xdr::PublicKey::PublicKeyTypeEd25519(
            stellar_xdr::Uint256(
                *issuer_keypair.public.as_bytes()
            )
        )
    );
    let issuer_account_id = issuer_xdr_account_id.clone().try_into_val(&env).unwrap();

    let fee_keypair = generate_keypair();
    let fee_identifier = to_ed25519(&env, &fee_keypair);
    
    let asset4 = Asset::CreditAlphanum4(AlphaNum4 {
        asset_code: AssetCode4([66u8; 4]),
        issuer: issuer_xdr_account_id.clone()
    });
    let token_id = env.register_stellar_asset_contract(asset4);
    let token = token::Client::new(&env, &token_id);

    token
    .with_source_account(&issuer_account_id)
    .mint(
        &Signature::Invoker,
        &0,
        &u1_identifier,
        &10_000_000,
    );

    let mut colors: Vec<(u32, u32)> = Vec::new(&env);
    let mut pay_amount: i128 = 0;

    for i in 0..10 {
        pay_amount += 1 as i128;
        colors.push_back((i, 1));
    }

    client.init(&token_id, &fee_identifier);

    let auth = get_auth(
        &env, 
        &token_id, 
        &u1_keypair,
        &u1_identifier, 
        &token,
        &Identifier::Contract(contract_id.clone()),
        &pay_amount
    );

    // env.budget().reset();

    client
        .with_source_account(&u1_account_id)
        .mine(&auth, &colors, &SourceAccount::None);
    
    let color = client
        .with_source_account(&u1_account_id)
        .get_color(&0, &u1_account_id);

    // println!("{}", env.budget());

    assert_eq!(color, 1);

    token
    .with_source_account(&issuer_account_id)
    .mint(
        &Signature::Invoker,
        &0,
        &u2_identifier,
        &10_000_000,
    );

    let auth = get_auth(
        &env, 
        &token_id, 
        &u2_keypair,
        &u2_identifier, 
        &token,
        &Identifier::Contract(contract_id.clone()),
        &pay_amount
    );

    client
        .with_source_account(&u2_account_id)
        .mine(&auth, &colors, &SourceAccount::AccountId(u1_account_id.clone()));

    let color1 = client
        .with_source_account(&u1_account_id)
        .get_color(&0, &u1_account_id);
    let color2 = client
        .with_source_account(&u1_account_id)
        .get_color(&0, &u2_account_id);

    assert_eq!(color1 + color2, 2);

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

    assert_eq!(token.balance(&u1_identifier), 10_000_000 - 10);
    assert_eq!(token.balance(&u1_identifier), 10_000_000 - 10);
    assert_eq!(token.balance(&fee_identifier), 20);
}