#![cfg(test)]

use std::println;

use soroban_sdk::{
    testutils::{Accounts, BytesN as UtilsBytesN},
    Address, BytesN, Env, Vec,
};

use crate::types::AssetAmount;

extern crate std;

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
