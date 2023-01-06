#![cfg(test)]

use super::*;
use std::println;
use soroban_sdk::{Env, testutils::Accounts, map, Map, vec, Vec, bytes, Bytes, bytesn, BytesN};

extern crate std;

// - CPU Instructions: 42076990
// - Memory Bytes: 5921593

const ITER: u32 = 255;

#[test]
fn test() {
    let env = Env::default();
    let u1 = env.accounts().generate();
    let contract_id = env.register_contract(None, ColorGlyph);
    let client = ColorGlyphClient::new(&env, contract_id);

    let mut b_palette = Bytes::new(&env);
    let mut colors_indexes: Vec<(u32, Vec<u32>)> = Vec::new(&env);
    let mut color_amount: Vec<(u32, u32)> = Vec::new(&env);

    for i in 0..=ITER {
        let hex = 16777215 / ITER * i; // 0 - 16777215 (black to white)

        colors_indexes.push_back((hex, vec![&env, i]));
        color_amount.push_back((hex, 1));

        b_palette.insert_from_array(i * 4, &hex.to_le_bytes());
    }

    env.budget().reset();

    client
    .with_source_account(&u1)
    .mine(&color_amount, &SourceAccount::None);

    // let color = client
    //     .with_source_account(&u1)
    //     .get_color(&0, &u1);

    // assert_eq!(
    //     color,
    //     1
    // );

    let
        // _palette
        _hash
    = client
    .with_source_account(&u1)
    .mint(
        &Glyph{
            width: 16,
            colors: vec![&env,
                (1, colors_indexes)
            ]
        }
    );

    // let color = client
    //     .with_source_account(&u1)
    //     .get_color(&0, &u1);

    // assert_eq!(
    //     color,
    //     0
    // );

    println!("{}", env.budget());
    // println!("{:?}", 16777215u32.to_le_bytes());
    println!("{:?}", _hash);
    // println!("{:?}", _palette);
    // println!("{:?}", b_palette);

    // assert_eq!(
    //     _palette,
    //     b_palette
    // );

    // assert_eq!(
    //     hash,
    //     bytesn!(&env, 0xd545fce7b4ecb775d8dd3f1eebbb607c26d39699468cf12a973c00924f724be1)
    // );

    // let count: u128 = 1;
    // let glyph = &Glyph{width: 3, colors: bytes!(&env, 0x010001000100010001)};
    // let invoker = &Address::Account(u1.clone());
    // let miners = &map!(&env, (Address::Account(u1.clone()), 9));

    // assert_eq!(
    //     client.get(&1),
    //     vec![
    //         &env,
    //         count.into_val(&env),
    //         glyph.into_val(&env),
    //         invoker.into_val(&env),
    //         invoker.into_val(&env),
    //         miners.to_raw(),
    //     ]
    // );
}
