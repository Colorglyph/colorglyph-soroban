#![cfg(test)]

use std::println;
extern crate std;

use chrono::Utc;

use crate::{
    contract::{ColorGlyph, ColorGlyphClient},
    types::{HashId, GlyphType, Error, StorageKey},
};
use soroban_sdk::{
    map,
    testutils::{Address as _, Ledger, LedgerInfo},
    token, vec, Address, Env
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
fn test_quick_mint() {
    let env = Env::default();

    env.mock_all_auths();
    env.budget().reset_unlimited();

    let contract_address = env.register_contract(None, ColorGlyph);
    let client = ColorGlyphClient::new(&env, &contract_address);

    let token_admin = Address::random(&env);
    let token_id = env.register_stellar_asset_contract(token_admin.clone());
    let token_admin_client = token::AdminClient::new(&env, &token_id);

    let u1_address = Address::random(&env);
    let u2_address = Address::random(&env);
    let fee_address = Address::random(&env);

    token_admin_client.mint(&u1_address, &10_000);
    token_admin_client.mint(&u2_address, &10_000);

    client.initialize(&token_id, &fee_address);

    client.colors_mine(
        &u1_address,
        &None,
        &map![
            &env,
            (0, 100),
            (1, 100),
            (2, 100),
            (3, 100),
            (4, 100),
            (5, 100),
            (6, 100),
            (7, 100)
        ],
    );

    let id = client.glyph_mint(
        &u1_address,
        &None,
        &Some(map![
            &env,
            (
                u1_address.clone(),
                map![
                    &env,
                    (0, vec![&env, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9]),
                    (1, vec![&env, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]),
                    (2, vec![&env, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29]),
                    (3, vec![&env, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39]),
                ]
            )
        ]),
        &None,
        &None,
    );

    let id = match id {
        HashId::Id(id) => id,
        _ => panic!(),
    };

    let hash = client.glyph_mint(
        &u1_address,
        &None,
        &Some(map![
            &env,
            (
                u1_address.clone(),
                map![
                    &env,
                    (4, vec![&env, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49]),
                    (5, vec![&env, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59]),
                    (6, vec![&env, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69]),
                    (7, vec![&env, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79]),
                ]
            )
        ]),
        &Some(8),
        &Some(id),
    );

    println!("{:?}\n", hash);

    println!("{:?}\n", client.glyph_get(&hash));

    match hash {
        HashId::Hash(hash) => {
            env.as_contract(&contract_address, || {
                let res = env
                    .storage()
                    .persistent()
                    .get::<StorageKey, Address>(&StorageKey::GlyphOwner(hash.clone()))
                    .unwrap();
        
                assert_eq!(res, u1_address);
            });
        },
        _ => panic!(),
    };
}

#[test]
fn test() {
    let env = Env::default();

    env.mock_all_auths();
    env.budget().reset_unlimited();

    let contract_address = env.register_contract(None, ColorGlyph);
    let client = ColorGlyphClient::new(&env, &contract_address);

    let token_admin = Address::random(&env);
    let token_id = env.register_stellar_asset_contract(token_admin.clone());
    let token_admin_client = token::AdminClient::new(&env, &token_id);

    let u1_address = Address::random(&env);
    let u2_address = Address::random(&env);
    let fee_address = Address::random(&env);

    token_admin_client.mint(&u1_address, &10_000);
    token_admin_client.mint(&u2_address, &10_000);

    client.initialize(&token_id, &fee_address);

    client.colors_mine(
        &u1_address,
        &None,
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
        min_temp_entry_expiration: Default::default(),
        min_persistent_entry_expiration: Default::default(),
        max_entry_expiration: Default::default(),
    });

    let id = client.glyph_mint(
        &u1_address,
        &None,
        &Some(map![
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
        ]),
        &None,
        &None,
    );

    let id = match id {
        HashId::Id(id) => id,
        _ => panic!(),
    };

    println!("{:?}", id);

    client.glyph_mint(
        &u1_address,
        &None,
        &Some(map![
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
        ]),
        &None,
        &Some(id),
    );
    client.glyph_mint(
        &u1_address,
        &None,
        &Some(map![
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
        ]),
        &None,
        &Some(id),
    );
    client.glyph_mint(
        &u1_address,
        &None,
        &Some(map![
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
        ]),
        &None,
        &Some(id),
    );

    println!("{:?}\n", client.glyph_get(&HashId::Id(id.clone())));

    let hash = client.glyph_mint(&u1_address, &None, &None, &Some(14), &Some(id));

    let hash = match hash {
        HashId::Hash(hash) => hash,
        _ => panic!(),
    };

    println!("{:?}", hash);

    match client.glyph_get(&HashId::Hash(hash.clone())) {
        GlyphType::Glyph(glyph) => {
            println!("{:?}", glyph.length);
        },
        _ => panic!(),
    }

    let id = client.glyph_scrape(&u1_address, &None, &HashId::Hash(hash.clone()));
    
    assert_eq!(
        client.try_glyph_get(&HashId::Hash(hash.clone())), 
        Err(Ok(Error::NotFound))
    );

    assert_ne!(
        client.try_glyph_get(&HashId::Id(id.unwrap())), 
        Err(Ok(Error::NotFound))
    );

    assert_eq!(
        client.glyph_scrape(&u1_address, &None, &HashId::Id(id.unwrap())),
        None
    );

    assert_eq!(
        client.try_glyph_get(&HashId::Id(id.unwrap())), 
        Err(Ok(Error::NotFound))
    );

    // println!("{:?}", env.budget().print());
}
