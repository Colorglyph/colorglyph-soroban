#![cfg(test)]

use std::println;

use super::*;
use crate::{colors_contract};
use soroban_sdk::{Env, testutils::Accounts, map, vec, Vec, bytes, Bytes, bytesn, BytesN};

extern crate std;

// - CPU Instructions: 8456590
// - Memory Bytes: 1142057

#[test]
fn test() {
    let env = Env::default();
    let u1 = env.accounts().generate();
    let contract_id = env.register_contract(None, GlyphContract);
    let client = GlyphContractClient::new(&env, &contract_id);

    let color_contract_id = env.register_contract_wasm(None, colors_contract::WASM);

    let mut b_palette = Bytes::new(&env);
    let mut v_palette: Vec<(u32, BytesN<3>)> = Vec::new(&env);

    for i in 0..255 {
        b_palette.push(i);
        b_palette.push(i);
        b_palette.push(i);

        v_palette.push_back((i as u32, bytesn!(&env, [i, i, i])));
    }

    env.budget().reset();

    let palette = client
    .with_source_account(&u1)
    .mint(
        &Glyph{
            width: 4,
            colors: map![&env, 
                (1, v_palette)
                // (1, vec![&env, 
                //     (0, bytesn!(&env, [0, 0, 0])),
                //     (3, bytesn!(&env, [0, 0, 0])),
                // ]),
                // (2, vec![&env, 
                //     (1, bytesn!(&env, [255, 255, 255])),
                //     (2, bytesn!(&env, [255, 255, 255])),
                // ]),
            ]
        }, 
        &color_contract_id
    );

    println!("{}", env.budget());

    assert_eq!(
        palette,
        b_palette
    )

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
