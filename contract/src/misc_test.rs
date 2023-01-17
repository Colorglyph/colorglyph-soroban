#![cfg(test)]

use std::println;

use soroban_auth::Identifier;
use soroban_sdk::{testutils::BytesN as UtilsBytesN, vec, Address, Bytes, BytesN, Env, Vec};
use stellar_xdr::Asset;

use crate::{
    colorglyph::{ColorGlyph, ColorGlyphClient},
    testutils::{generate_full_account, get_incr_allow_signature},
    token::Client as TokenClient,
    types::{AssetAmount, Error, Glyph, MaybeAccountId, MaybeSignature, OfferType, StorageKey},
};

extern crate std;

#[test]
fn test_binary_vs_index() {
  let env = Env::default();

  let item = AssetAmount(BytesN::random(&env), 10i128);
  let mut unsorted: Vec<AssetAmount> = Vec::new(&env);
  let mut binary_sorted: Vec<AssetAmount> = Vec::new(&env);
  let mut index_sorted: Vec<AssetAmount> = Vec::new(&env);

  for i in 0..10 {
      unsorted.push_back(AssetAmount(BytesN::random(&env), i));
  }

  unsorted.push_back(item.clone());

  env.budget().reset();

  for v in unsorted.clone().into_iter_unchecked() {
      match binary_sorted.binary_search(&v) {
          Result::Err(i) => binary_sorted.insert(i, v),
          _ => (),
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