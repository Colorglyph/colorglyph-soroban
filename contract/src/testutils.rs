#![cfg(any(test, feature = "testutils"))]

use std::{print, println};

use rand::thread_rng;
use ed25519_dalek::Keypair;
use soroban_auth::{Identifier, Signature, testutils::ed25519::{signer, sign}, AccountSignatures};
use soroban_sdk::{Env, BytesN, symbol, AccountId, TryIntoVal, testutils::Accounts, vec};
use stellar_xdr::{AccountId as XdrAccountId, PublicKey, Uint256};

use crate::token::Client;

extern crate std;

pub fn get_incr_allow_signature(
  env: &Env, 
  token_id: &BytesN<32>,
  from_keypair: &Keypair, 
  token: &Client,
  to_identifier: &Identifier,
  amount: &i128
) -> Signature {
  let from_public = from_keypair.public.to_bytes();
  let from_secret = from_keypair.secret.to_bytes();
  let from_xdr_id = XdrAccountId(PublicKey::PublicKeyTypeEd25519(Uint256(from_public)));
  let from_id = from_xdr_id.clone().try_into_val(env).unwrap();
  let from_identifier = Identifier::Account(from_id.clone());

  let (
    _, 
    from_signer
  ) = signer(env, &from_secret);

  let signature = sign(
      env,
      &from_signer,
      &token_id,
      symbol!("incr_allow"),
      (
          &from_identifier,
          token.nonce(&from_identifier),
          to_identifier,
          amount
      ),
  );

  match signature {
    Signature::Ed25519(s) => {
      Signature::Account(AccountSignatures{
        account_id: from_id,
        signatures: vec!(env, s),
      })
    },
    _ => panic!("unexpected signature type"),
  }
}

pub fn generate_full_account(env: &Env) -> (Keypair, XdrAccountId, AccountId, Identifier) {
  let keypair = Keypair::generate(&mut thread_rng());
  let public = keypair.public.to_bytes();
  // let secret = keypair.secret.to_bytes();
  let account_xdr_id = XdrAccountId(PublicKey::PublicKeyTypeEd25519(Uint256(public)));
  let account_id = account_xdr_id.clone().try_into_val(env).unwrap();
  let identifier = Identifier::from(account_id.clone());

  env.accounts().create(&account_id);
  env.accounts().update_balance(&account_id, 10_000i64);

  (keypair, account_xdr_id, account_id, identifier)
}