#![cfg(test)]

use super::*;
use soroban_sdk::{Env, bytesn, testutils::Accounts, IntoVal};

extern crate std;

#[test]
fn test() {
    let env = Env::default();
    let u1 = env.accounts().generate();
    let contract_id = env.register_contract(None, ColorGlyphContract);
    let client = ColorGlyphContractClient::new(&env, &contract_id);

    let hash = client
    .with_source_account(&u1)
    .mint();

    assert_eq!(
        hash, 
        bytesn!(&env, 0xa6188710c09cfbc77383ee0588dec2f7affa6e03e78aa900e9ae597a8d8faba3)
    );

    let count: u128 = 1;
    let glyph = &Glyph{width: 3, colors: bytes!(&env, 0x010001000100010001)};
    let invoker = &Address::Account(u1.clone());
    let miners = &map!(&env, (Address::Account(u1.clone()), 9));

    assert_eq!(
        client.get(&1),
        vec![
            &env,
            count.into_val(&env),
            glyph.into_val(&env),
            invoker.into_val(&env),
            invoker.into_val(&env),
            miners.to_raw(),
        ]
    );
}
