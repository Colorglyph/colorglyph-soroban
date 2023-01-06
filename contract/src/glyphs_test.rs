#![cfg(test)]

use super::*;
use std::println;
use soroban_sdk::{Env, testutils::Accounts, map, Map, Vec, Bytes, BytesN};

extern crate std;

// Bytes
// - CPU Instructions: 8456590
// - Memory Bytes: 1,142,057

// u32
// - CPU Instructions: 9673250
// - Memory Bytes: 1,242,093

// burning
// - CPU Instructions: 48206590
// - Memory Bytes: 9,091,362

#[test]
fn test() {
    let env = Env::default();
    let u1 = env.accounts().generate();
    let contract_id = env.register_contract(None, ColorGlyph);
    let client = ColorGlyphClient::new(&env, contract_id);

    let mut b_palette = Bytes::new(&env);
    let mut v_palette: Vec<(u32, u32)> = Vec::new(&env);
    let mut m_palette: Map<u32, u32> = Map::new(&env);

    for hex in 0..=255 {
        b_palette.insert_from_array(hex * 4, &hex.to_le_bytes());
        v_palette.push_back((hex, hex));

        let current_m_palette_amount = m_palette
            .get(hex)
            .unwrap_or(Ok(0))
            .unwrap();

        m_palette.set(hex, current_m_palette_amount + 1);
    }

    env.budget().reset();

    client
    .with_source_account(&u1)
    .mine(&m_palette, &SourceAccount::None);

    // let color = client
    //     .with_source_account(&u1)
    //     .get_color(&0, &u1);

    // assert_eq!(
    //     color,
    //     1
    // );

    let palette = client
    .with_source_account(&u1)
    .mint(
        &Glyph{
            width: 16,
            colors: map![&env, 
                (1, v_palette)
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

    // println!("{:?}", palette);
    // println!("{:?}", b_palette);

    // assert_eq!(
    //     palette,
    //     b_palette
    // )

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
