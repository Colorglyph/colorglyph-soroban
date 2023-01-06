#![cfg(test)]

use crate::types::Color;

use super::*;
use soroban_sdk::{Env, testutils::Accounts, vec};

extern crate std;

#[test]
fn mine_test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, ColorGlyph);
    let client = ColorGlyphClient::new(&env, &contract_id);

    let user_1 = env.accounts().generate();
    let user_2 = env.accounts().generate();
    let user_3 = env.accounts().generate();

    let mut colors: Vec<(u32, u32)> = Vec::new(&env);

    for i in 0..10 {
        colors.push_back((i, 1));
    }

    // env.budget().reset();

    client
        .with_source_account(&user_1)
        .mine(&colors, &SourceAccount::None);
    
    let color = client
        .with_source_account(&user_1)
        .get_color(&0, &user_1);

    // println!("{}", env.budget());

    assert_eq!(color, 1);

    client
        .with_source_account(&user_2)
        .mine(&colors, &SourceAccount::AccountId(user_1.clone()));

    let color1 = client
        .with_source_account(&user_1)
        .get_color(&0, &user_1);
    let color2 = client
        .with_source_account(&user_1)
        .get_color(&0, &user_2);

    assert_eq!(color1 + color2, 2);

    client
        .with_source_account(&user_1)
        .xfer(
            &vec![&env, 
                ColorAmount(Color(0, 1), 1), 
                ColorAmount(Color(0, 2), 1)
            ], 
            &SourceAccount::AccountId(user_3.clone())
        );
        
    let color1 = client
        .with_source_account(&user_3)
        .get_color(&0, &user_1);
    let color2 = client
        .with_source_account(&user_3)
        .get_color(&0, &user_2);

    assert_eq!(color1 + color2, 2);

    // client
    //     .with_source_account(&user_3)
    //     .burn(&map![&env, (Color(0, 1), 1)]);

    // client
    //     .with_source_account(&user_3)
    //     .burn(&map![&env, (Color(0, 2), 1)]);

    // let color1 = client
    //     .with_source_account(&user_3)
    //     .get_color(&0, &user_1);
    // let color2 = client
    //     .with_source_account(&user_3)
    //     .get_color(&0, &user_2);

    // assert_eq!(color1 + color2, 0);
}