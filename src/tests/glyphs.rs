#![cfg(test)]

use std::println;
extern crate std;

use crate::{
    contract::{ColorGlyph, ColorGlyphClient},
    types::{Error, StorageKey},
};
use soroban_fixed_point_math::FixedPoint;
use soroban_sdk::{
    map,
    testutils::{Address as _, BytesN as _},
    token, vec, Address, BytesN, Env,
};

mod colorglyph {
    soroban_sdk::contractimport!(
        file = "./target/wasm32-unknown-unknown/release/colorglyph.optimized.wasm"
    );
}

/* TODO
test scrape of a never minted glyph
test partial scrape to mint
test partial scrape to build then remint
test full scrape to remint
test scrape to `to` account
test Glyph transfer
test Colors transfer
test to ensure hash gen is consistent when duping indexes or mixing in white/missing pixels
test scraping a glyph when there's already a Dust glyph in Storage
ensure color spends are working
*/

#[test]
fn big_mint() {
    let env = Env::default();

    env.mock_all_auths();
    env.budget().reset_unlimited();

    // let contract_address = env.register_contract_wasm(None, colorglyph::WASM);
    let contract_address = env.register_contract(None, ColorGlyph);
    let client = ColorGlyphClient::new(&env, &contract_address);

    let token_admin = Address::generate(&env);
    let token_address = env.register_stellar_asset_contract(token_admin.clone());
    let token_admin_client = token::StellarAssetClient::new(&env, &token_address);

    let u1_address = Address::generate(&env);
    let fee_address = Address::generate(&env);

    token_admin_client.mint(&u1_address, &10_000);

    client.initialize(&u1_address, &token_address, &fee_address, &1);

    let width: u64 = 16; // 40;
    let mut index = 0;
    let mut mine_colors = map![&env];
    let mut mint_colors = map![&env];

    for i in 0..width {
        for j in 0..width {
            let red = 255 - 1.fixed_div_floor(width, i * 255).unwrap();
            let green = 255 - 1.fixed_div_floor(width, j * 255).unwrap();
            let blue = 1.fixed_div_floor(width * width, i * j * 255).unwrap();

            let color = red * 256u64.pow(2) + green * 256 + blue;

            mine_colors.set(color as u32, 1);
            mint_colors.set(color as u32, vec![&env, index as u32]);
            index += 1;
        }
    }

    client.colors_mine(&u1_address, &mine_colors, &None, &None);

    let hash = BytesN::from_array(
        &env,
        &[
            // 171, 144, 28, 55, 171, 87, 24, 54, 102, 181, 50, 141, 120, 172, 192, 50, 179, 102, 40,
            // 61, 163, 28, 127, 243, 243, 34, 99, 105, 98, 143, 227, 46,
            158, 185, 37, 209, 254, 153, 112, 252, 14, 46, 147, 173, 27, 76, 140, 30, 146, 19, 102, 0, 249, 170, 200, 75, 137, 221, 164, 72, 20, 209, 136, 203
        ],
    );

    client.glyph_mint(
        &hash,
        &u1_address,
        &None,
        &map![&env, (u1_address.clone(), mint_colors)],
        &None,
    );

    // env.budget().reset_default();
    let map = map![&env];
    env.budget().reset_unlimited();

    client.glyph_mint(&hash, &u1_address, &None, &map, &Some(width as u32));

    let glyph = client.glyph_get(&hash);

    println!("{:?}", glyph.colors.get(u1_address.clone()).unwrap().len());

    client.glyph_scrape(&None, &hash);

    let glyph = client.glyph_get(&hash);

    println!("{:?}", glyph.colors.get(u1_address.clone()).unwrap().len());

    // 40
    // "cost": {
    //     "cpuInsns": "77759603",
    //     "memBytes": "38784019"
    // },
    // Cpu limit: 18446744073709551615; used: 28017220
    // Mem limit: 18446744073709551615; used: 4439162

    // env.budget().print();

    // 40
    // 171, 144, 28, 55, 171, 87, 24, 54, 102, 181, 50, 141, 120, 172, 192, 50, 179, 102, 40, 61, 163, 28, 127, 243, 243, 34, 99, 105, 98, 143, 227, 46
    // 16
    // 158, 185, 37, 209, 254, 153, 112, 252, 14, 46, 147, 173, 27, 76, 140, 30, 146, 19, 102, 0, 249, 170, 200, 75, 137, 221, 164, 72, 20, 209, 136, 203
    // println!("{:?}", hash);
}

#[test]
fn toolbox_test() {
    let env = Env::default();

    env.mock_all_auths();
    env.budget().reset_unlimited();

    let contract_address = env.register_contract(None, ColorGlyph);
    let client = ColorGlyphClient::new(&env, &contract_address);

    let token_admin = Address::generate(&env);
    let token_address = env.register_stellar_asset_contract(token_admin.clone());
    let token_admin_client = token::StellarAssetClient::new(&env, &token_address);

    let u1_address = Address::generate(&env);
    let fee_address = Address::generate(&env);

    token_admin_client.mint(&u1_address, &10_000);

    client.initialize(&u1_address, &token_address, &fee_address, &1);

    client.colors_mine(
        &u1_address,
        &map![&env, (0, 100), (16777215, 100),],
        &None,
        &None,
    );

    let hash = BytesN::random(&env);

    let id = client.glyph_mint(
        &hash,
        &u1_address,
        &None,
        &map![
            &env,
            (
                u1_address.clone(),
                map![&env, (0, vec![&env, 0, 1]), (16777215, vec![&env, 2, 3]),]
            )
        ],
        &None,
    );

    println!("{:?}", id);
}

#[test]
fn test_dupe_mint() {
    let env = Env::default();

    env.mock_all_auths();
    env.budget().reset_unlimited();

    let contract_address = env.register_contract(None, ColorGlyph);
    let client = ColorGlyphClient::new(&env, &contract_address);

    let token_admin = Address::generate(&env);
    let token_address = env.register_stellar_asset_contract(token_admin.clone());
    let token_admin_client = token::StellarAssetClient::new(&env, &token_address);

    let u1_address = Address::generate(&env);
    let u2_address = Address::generate(&env);
    let fee_address = Address::generate(&env);

    token_admin_client.mint(&u1_address, &10_000);
    token_admin_client.mint(&u2_address, &10_000);

    client.initialize(&u1_address, &token_address, &fee_address, &1);

    client.colors_mine(
        &u1_address,
        &map![&env, (0, 100), (16777215, 100),],
        &None,
        &None,
    );

    let hash = BytesN::from_array(
        &env,
        &[
            146, 244, 196, 178, 69, 175, 195, 226, 252, 79, 5, 122, 242, 142, 128, 55, 167, 30,
            183, 95, 130, 159, 120, 66, 91, 161, 26, 127, 31, 119, 35, 249,
        ],
    );

    client.glyph_mint(
        &hash,
        &u1_address,
        &None,
        &map![
            &env,
            (
                u1_address.clone(),
                map![&env, (0, vec![&env, 3, 1]), (16777215, vec![&env, 2, 0]),]
            )
        ],
        &Some(2),
    );

    println!("{:?}\n", hash);

    assert_eq!(
        client.try_glyph_mint(
            &hash,
            &u1_address,
            &None,
            &map![
                &env,
                (
                    u1_address.clone(),
                    map![&env, (16777215, vec![&env, 0, 2]), (0, vec![&env, 1, 3]),]
                )
            ],
            &Some(2),
        ),
        Err(Ok(soroban_sdk::Error::from(Error::NotEmpty)))
    );
}

#[test]
fn test_to_mint() {
    let env = Env::default();

    env.mock_all_auths();
    env.budget().reset_unlimited();

    let contract_address = env.register_contract(None, ColorGlyph);
    let client = ColorGlyphClient::new(&env, &contract_address);

    let token_admin = Address::generate(&env);
    let token_address = env.register_stellar_asset_contract(token_admin.clone());
    let token_admin_client = token::StellarAssetClient::new(&env, &token_address);

    let u1_address = Address::generate(&env);
    let u2_address = Address::generate(&env);
    let fee_address = Address::generate(&env);

    token_admin_client.mint(&u1_address, &10_000);
    token_admin_client.mint(&u2_address, &10_000);

    client.initialize(&u1_address, &token_address, &fee_address, &1);

    client.colors_mine(
        &u1_address,
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
        &None,
        &None,
    );

    let hash = BytesN::from_array(
        &env,
        &[
            225, 229, 239, 100, 89, 99, 62, 200, 24, 206, 221, 76, 251, 184, 5, 232, 50, 233, 128,
            82, 192, 51, 181, 139, 201, 132, 70, 58, 41, 111, 133, 31,
        ],
    );

    client.glyph_mint(
        &hash,
        &u1_address,
        &None,
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
                ]
            )
        ],
        &None,
    );

    client.glyph_mint(
        &hash,
        &u1_address,
        &Some(u2_address.clone()),
        &map![
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
        ],
        &Some(8),
    );

    println!("{:?}\n", hash);

    println!("{:?}\n", client.glyph_get(&hash.clone()));

    env.as_contract(&contract_address, || {
        let res = env
            .storage()
            .persistent()
            .get::<StorageKey, Address>(&StorageKey::GlyphOwner(hash.clone()))
            .unwrap();

        assert_eq!(res, u2_address);
    });
}

#[test]
fn test_partial_mint() {
    let env = Env::default();

    env.mock_all_auths();
    env.budget().reset_unlimited();

    let contract_address = env.register_contract(None, ColorGlyph);
    let client = ColorGlyphClient::new(&env, &contract_address);

    let token_admin = Address::generate(&env);
    let token_address = env.register_stellar_asset_contract(token_admin.clone());
    let token_admin_client = token::StellarAssetClient::new(&env, &token_address);

    let u1_address = Address::generate(&env);
    let u2_address = Address::generate(&env);
    let fee_address = Address::generate(&env);

    token_admin_client.mint(&u1_address, &10_000);
    token_admin_client.mint(&u2_address, &10_000);

    client.initialize(&u1_address, &token_address, &fee_address, &1);

    client.colors_mine(
        &u1_address,
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
        &None,
        &None,
    );
    client.colors_mine(
        &u2_address,
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
        &None,
        &Some(u1_address.clone()),
    );
    client.colors_mine(
        &u1_address,
        &map![
            &env,
            (20, 100),
            (21, 100),
            (22, 100),
            (23, 100),
            (24, 100),
            (25, 100),
            (26, 100),
            (27, 100),
            (28, 100),
            (29, 100)
        ],
        &None,
        &None,
    );

    let hash = BytesN::from_array(
        &env,
        &[
            10, 15, 138, 151, 38, 207, 168, 1, 210, 138, 63, 67, 145, 229, 44, 3, 88, 227, 87, 209, 37, 146, 156, 96, 58, 95, 0, 108, 46, 42, 224, 120
        ],
    );

    client.glyph_mint(
        &hash,
        &u1_address,
        &None,
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
        &None,
    );

    client.glyph_mint(
        &hash,
        &u1_address,
        &None,
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
        &None,
    );
    client.glyph_mint(
        &hash,
        &u1_address,
        &None,
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
        &None,
    );
    client.glyph_mint(
        &hash,
        &u1_address,
        &None,
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
        &None,
    );
    client.glyph_mint(
        &hash,
        &u1_address,
        &None,
        &map![
            &env,
            (
                u1_address.clone(),
                map![
                    &env,
                    (
                        20,
                        vec![&env, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209]
                    ),
                    (
                        21,
                        vec![&env, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219]
                    ),
                    (
                        22,
                        vec![&env, 220, 221, 222, 223, 224, 225, 226, 227, 228, 229]
                    ),
                    (
                        23,
                        vec![&env, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239]
                    ),
                    (
                        24,
                        vec![&env, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249]
                    ),
                ]
            )
        ],
        &None,
    );
    client.glyph_mint(
        &hash,
        &u1_address,
        &None,
        &map![
            &env,
            (
                u1_address.clone(),
                map![
                    &env,
                    (
                        25,
                        vec![&env, 250, 251, 252, 253, 254, 255, 256, 257, 258, 259]
                    ),
                    (
                        26,
                        vec![&env, 260, 261, 262, 263, 264, 265, 266, 267, 268, 269]
                    ),
                    (
                        27,
                        vec![&env, 270, 271, 272, 273, 274, 275, 276, 277, 278, 279]
                    ),
                    (
                        28,
                        vec![&env, 280, 281, 282, 283, 284, 285, 286, 287, 288, 289]
                    ),
                    (
                        29,
                        vec![&env, 290, 291, 292, 293, 294, 295, 296, 297, 298, 299]
                    ),
                ]
            )
        ],
        &None,
    );

    client.glyph_mint(&hash, &u1_address, &None, &map![&env], &Some(14));

    let glyph = client.glyph_get(&hash.clone());

    assert_eq!(glyph.length, 300);

    client.glyph_scrape(&None, &hash.clone());

    assert_eq!(client.glyph_get(&hash.clone()).colors.len(), 1);

    client.glyph_scrape(&None, &hash.clone());

    assert_eq!(client.glyph_get(&hash.clone()).colors.len(), 0);

    // println!("{:?}", env.budget().print());
}
