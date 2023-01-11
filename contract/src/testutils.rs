#![cfg(any(test, feature = "testutils"))]

use ed25519_dalek::Keypair;
use rand::thread_rng;
use soroban_auth::{Identifier, Signature, testutils::ed25519::{signer, sign}};
use soroban_sdk::{Env, IntoVal, BytesN, symbol, AccountId, TryIntoVal};

use crate::token::Client;

pub fn generate_keypair() -> Keypair {
  Keypair::generate(&mut thread_rng())
}

pub fn to_ed25519(e: &Env, kp: &Keypair) -> Identifier {
  Identifier::Ed25519(kp.public.to_bytes().into_val(e))
}

pub fn get_incr_allow_signature(
  env: &Env, 
  token_id: &BytesN<32>, 
  from_keypair: &Keypair, 
  token: &Client,
  to_identifier: &Identifier, 
  amount: &i128
) -> Signature {
  let (
      from_identifier, 
      from_signer
  ) = signer(&env, &from_keypair.secret.to_bytes());
  
  sign(
      env,
      &from_signer,
      &token_id.clone(),
      symbol!("incr_allow"),
      (
          &from_identifier,
          token.nonce(&from_identifier),
          to_identifier,
          amount
      ),
  )
}

pub fn generate_full_account(env: &Env) -> (Keypair, stellar_xdr::AccountId, AccountId, Identifier) {
  let keypair = generate_keypair();
  let account_xdr_id = stellar_xdr::AccountId(
      stellar_xdr::PublicKey::PublicKeyTypeEd25519(
          stellar_xdr::Uint256(
              *keypair.public.as_bytes()
          )
      )
  );
  let account_id = account_xdr_id.clone().try_into_val(env).unwrap();
  let identifier = to_ed25519(env, &keypair);

  (keypair, account_xdr_id, account_id, identifier)
}