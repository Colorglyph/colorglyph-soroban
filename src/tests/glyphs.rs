#![cfg(test)]

use std::println;
extern crate std;

use chrono::Utc;

use crate::{
    contract::{ColorGlyph, ColorGlyphClient},
    types::{Glyph, GlyphTypeArg, StorageKey},
};
use soroban_sdk::{
    map,
    testutils::{Address as _, Ledger, LedgerInfo},
    token, vec, Address, Env, Map, Vec,
};

// TODO
// test scrape of a never minted glyph
// test partial scrape to mint
// test partial scrape to build then remint
// test full scrape to remint
// test mint to `to` account
// test scrape to `to` account
// test glyph transfer
// test glyphbox transfer

#[test]
fn test() {
    let env = Env::default();

    env.mock_all_auths();

    let contract_address = env.register_contract(None, ColorGlyph);
    let client = ColorGlyphClient::new(&env, &contract_address);

    let token_admin = Address::random(&env);
    let token_id = env.register_stellar_asset_contract(token_admin.clone());
    let token = token::Client::new(&env, &token_id);

    let u1_address = Address::random(&env);
    let u2_address = Address::random(&env);
    let fee_address = Address::random(&env);

    token.mint(&u1_address, &10_000);
    token.mint(&u2_address, &10_000);

    client.initialize(&token_id, &fee_address);

    client.colors_mine(
        &u1_address,
        &Option::None,
        &map![
            &env,
            (0, 100),
            (1, 100),
            (2, 100),
            (3, 100),
            (4, 100),
            (5, 100),
            (6, 100),
            (7, 100),
            (8, 100),
            (9, 100)
        ],
    );
    client.colors_mine(
        &u2_address,
        &Some(u1_address.clone()),
        &map![
            &env,
            (10, 100),
            (11, 100),
            (12, 100),
            (13, 100),
            (14, 100),
            (15, 100),
            (16, 100),
            (17, 100),
            (18, 100),
            (19, 100)
        ],
    );

    env.ledger().set(LedgerInfo {
        timestamp: Utc::now().timestamp() as u64,
        protocol_version: Default::default(),
        sequence_number: Default::default(),
        network_id: Default::default(),
        base_reserve: Default::default(),
    });

    env.budget().reset_default();
    let id = client.glyph_build(
        &u1_address,
        &map![
            &env,
            (
                u1_address.clone(),
                map![
                    &env,
                    (0, vec![&env, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9]),
                    (1, vec![&env, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]),
                    (2, vec![&env, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29]),
                    (3, vec![&env, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39]),
                    (4, vec![&env, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49]),
                ]
            )
        ],
        &Option::None,
    );

    // println!("{:?}", id);

    client.glyph_build(
        &u1_address,
        &map![
            &env,
            (
                u1_address.clone(),
                map![
                    &env,
                    (5, vec![&env, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59]),
                    (6, vec![&env, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69]),
                    (7, vec![&env, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79]),
                    (8, vec![&env, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89]),
                    (9, vec![&env, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99]),
                ]
            )
        ],
        &Some(id),
    );

    println!("{:?}\n", client.glyph_get(&GlyphTypeArg::Id(id.clone())));

    client.glyph_build(
        &u1_address,
        &map![
            &env,
            (
                u2_address.clone(),
                map![
                    &env,
                    (
                        10,
                        vec![&env, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109]
                    ),
                    (
                        11,
                        vec![&env, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119]
                    ),
                    (
                        12,
                        vec![&env, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129]
                    ),
                    (
                        13,
                        vec![&env, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139]
                    ),
                    (
                        14,
                        vec![&env, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149]
                    ),
                ]
            )
        ],
        &Some(id),
    );
    client.glyph_build(
        &u1_address,
        &map![
            &env,
            (
                u2_address.clone(),
                map![
                    &env,
                    (
                        15,
                        vec![&env, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159]
                    ),
                    (
                        16,
                        vec![&env, 160, 161, 162, 163, 164, 165, 166, 167, 168, 169]
                    ),
                    (
                        17,
                        vec![&env, 170, 171, 172, 173, 174, 175, 176, 177, 178, 179]
                    ),
                    (
                        18,
                        vec![&env, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189]
                    ),
                    (
                        19,
                        vec![&env, 190, 191, 192, 193, 194, 195, 196, 197, 198, 199]
                    ),
                ]
            )
        ],
        &Some(id),
    );

    env.as_contract(&contract_address, || {
        let res = env
            .storage()
            .get::<StorageKey, Map<Address, Map<u32, Vec<u32>>>>(&StorageKey::GlyphBox(id));

        // println!("{:?}", res);
    });

    let hash = client.glyph_mint(&u1_address, &Option::None, &14, &id);

    // println!("{:?}", hash);

    env.as_contract(&contract_address, || {
        let res = env
            .storage()
            .get::<StorageKey, Glyph>(&StorageKey::Glyph(hash.clone()));

        println!("{:?}", res.unwrap().unwrap().length);
    });

    let id = client.glyph_scrape(
        &u1_address,
        &Option::None,
        &GlyphTypeArg::Hash(hash.clone()),
    );

    env.as_contract(&contract_address, || {
        let res1 = env
            .storage()
            .get::<StorageKey, Glyph>(&StorageKey::Glyph(hash.clone()));

        let res2 = env
            .storage()
            .get::<StorageKey, Map<Address, Map<u32, Vec<u32>>>>(&StorageKey::GlyphBox(
                id.unwrap(),
            ));

        assert_eq!(res1, None);
        assert_ne!(res2, None);
    });

    // println!("{:?}", id);

    assert_eq!(
        client.glyph_scrape(&u1_address, &Option::None, &GlyphTypeArg::Id(id.unwrap())),
        None
    );

    env.as_contract(&contract_address, || {
        let res2 = env
            .storage()
            .get::<StorageKey, Map<Address, Map<u32, Vec<u32>>>>(&StorageKey::GlyphBox(
                id.unwrap(),
            ));

        assert_eq!(res2, None);
    });

    // println!("{:?}", env.budget().print());
}
