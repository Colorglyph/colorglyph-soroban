#![cfg(test)]

use std::println;

use soroban_sdk::{
    testutils::{Accounts, BytesN as UtilsBytesN},
    Address, BytesN, Env, TryIntoVal, Vec,
};
use stellar_strkey::ed25519;
use stellar_xdr::Uint256;

use crate::types::AssetAmount;
use fixed_point_math::FixedPoint;

extern crate std;

#[test]
fn test_mootz_math() {
    const MAKER_ROYALTY_RATE: i128 = 3;
    const MINER_ROYALTY_RATE: i128 = 2;

    let amount = 16i128;
    let total_pixels = 10u32;
    let miner_pixels = 10u32;

    let res1 = MAKER_ROYALTY_RATE.fixed_mul_ceil(amount, 100).unwrap();

    println!("{}", res1);

    let res2 = MINER_ROYALTY_RATE
        .fixed_mul_ceil(amount, 100)
        .unwrap()
        .fixed_mul_ceil(i128::from(miner_pixels), i128::from(total_pixels))
        .unwrap();

    println!("{}", res2);
}

#[test]
fn gen_address_from_string() {
    let env = Env::default();

    let ed25519_pubkey =
        ed25519::PublicKey::from_string("GDPUT6M7JFHRXYUJILRY6FK7763GQDUBIFQAAGSCMALAJ65DZATW3ZKQ")
            .unwrap();
    let xdr_account_id = stellar_xdr::AccountId(stellar_xdr::PublicKey::PublicKeyTypeEd25519(
        Uint256(ed25519_pubkey.0),
    ));
    let account_id = xdr_account_id.clone().try_into_val(&env).unwrap();
    let address = Address::Account(account_id);

    println!("{:?}", address);
}

#[test]
fn test_vec_pop() {
    let env = Env::default();

    let mut items_front: Vec<Address> = Vec::new(&env);
    let mut items_back: Vec<Address> = Vec::new(&env);

    env.budget().reset();

    for _ in 0..10 {
        items_front.push_front(Address::Account(env.accounts().generate()));
    }

    println!("{:?}", items_front.len());

    let test = items_front.pop_front();

    println!("{:?}", test);
    println!("{:?}", items_front.len());
    // - CPU Instructions: 258920
    // - Memory Bytes: 11437
    println!("items front {:?}", env.budget().print());

    env.budget().reset();

    for _ in 0..10 {
        items_back.push_back(Address::Account(env.accounts().generate()));
    }

    println!("{:?}", items_back.len());

    let test = items_back.pop_back();

    println!("{:?}", test);
    println!("{:?}", items_back.len());
    // - CPU Instructions: 258920
    // - Memory Bytes: 11437
    println!("items back {:?}", env.budget().print());
}

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
    binary_sorted.get(index).unwrap().unwrap();

    // - CPU Instructions: 17458
    // - Memory Bytes: 3551
    println!("binary get {:?}", env.budget().print());

    env.budget().reset();

    let index = index_sorted.first_index_of(&item).unwrap();
    index_sorted.get(index).unwrap().unwrap();

    // - CPU Instructions: 24582
    // - Memory Bytes: 6351
    println!("index get {:?}", env.budget().print());
}
