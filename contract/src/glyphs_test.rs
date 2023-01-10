#![cfg(test)]

use super::*;
use std::println;
use soroban_sdk::{Env, testutils::Accounts, map, Map, vec, Vec, bytes, Bytes, bytesn, BytesN};

extern crate std;

const ITER: u32 = 255;

#[test]
fn test() {
    let env = Env::default();
    let u1 = env.accounts().generate();
    let u2 = env.accounts().generate();
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

    client
        .with_source_account(&u1)
        .mine(&color_amount, &SourceAccount::None);

    let color = client
        .with_source_account(&u1)
        .get_color(&0, &u1);

    assert_eq!(color, 1);

    ////

    env.budget().reset();

    let hash = client
        .with_source_account(&u1)
        .mint(
            &Glyph{
                width: 16,
                colors: vec![&env,
                    (1, colors_indexes.clone())
                ]
            }
        );

    // - CPU Instructions: 23284840
    // - Memory Bytes: 3433113
    // println!("{}", env.budget());

    let color = client
        .with_source_account(&u1)
        .get_color(&0, &u1);

    assert_eq!(color, 0);

    env.budget().reset();

    let glyph = client
        .with_source_account(&u1)
        .get_glyph(&hash);

    // - CPU Instructions: 1826840
    // - Memory Bytes: 224319
    // println!("{}", env.budget());

    // println!("{:?}", glyph);

    env.budget().reset();

    client
        .with_source_account(&u1)
        .scrape(&hash);

    // - CPU Instructions: 17019100
    // - Memory Bytes: 2454954
    // println!("{}", env.budget());

    // let hash = client
    //     .with_source_account(&u1)
    //     .mint(
    //         &Glyph{
    //             width: 16,
    //             colors: vec![&env,
    //                 (1, colors_indexes.clone())
    //             ]
    //         }
    //     );

    // let glyph = client
    //     .with_source_account(&u1)
    //     .get_glyph(&hash);

    // assert_eq!(glyph.scraped, true);

    // let color = client
    //     .with_source_account(&u1)
    //     .get_color(&0, &u1);

    // assert_eq!(color, 1);
}
