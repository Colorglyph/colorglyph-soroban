use soroban_sdk::{symbol_short, Address, BytesN, Env, Map, Symbol, Vec};

use crate::types::Offer;

pub fn colors_mine(env: &Env, miner: &Address, to: &Address, colors: Map<u32, u32>) {
    env.events()
        .publish((symbol_short!("mine"), miner, to), colors);
}

pub fn colors_transfer(env: &Env, from: &Address, to: &Address, colors: Vec<(Address, u32, u32)>) {
    env.events()
        .publish((symbol_short!("transfer"), from, to), colors);
}

pub fn colors_out(env: &Env, miner: &Address, minter: &Address, color: u32, indexes_length: u32) {
    env.events().publish(
        (symbol_short!("color_out"), miner.clone(), minter.clone()),
        (color, indexes_length),
    );
}

pub fn minted_event(env: &Env, minter: &Address, to: Option<Address>, hash: &BytesN<32>) {
    env.events().publish(
        (symbol_short!("minted"), minter.clone(), to.clone()),
        hash.clone(),
    );
}

pub fn minting_event(env: &Env, minter: &Address) {
    env.events()
        .publish((symbol_short!("minting"), minter.clone()), ());
}

pub fn transfer_colors_event(env: &Env, from: &Address, to: &Address) {
    env.events().publish(
        (
            Symbol::new(&env, "transfer_colors"),
            from.clone(),
            to.clone(),
        ),
        (),
    );
}

// Note this event has changed to avoid unnecessary enlargement of soroban meta.
pub fn transfer_glyph_event(env: &Env, to: &Address, glyph_hash: &BytesN<32>) {
    env.events().publish(
        (
            Symbol::new(&env, "transfer_glyph"),
            glyph_hash.clone(),
            to.clone(),
        ),
        (),
    );
}

pub fn scrape_colors_event(env: &Env, colors_owner: &Address, to: Option<Address>) {
    env.events().publish(
        (
            Symbol::new(&env, "scrape_colors"),
            colors_owner.clone(),
            to.clone(),
        ),
        (),
    );
}

pub fn scrape_glyph_event(
    env: &Env,
    owner: &Address,
    to: Option<Address>,
    glyph_hash: &BytesN<32>,
) {
    env.events().publish(
        (Symbol::new(&env, "scrape_glyph"), owner.clone(), to.clone()),
        glyph_hash.clone(),
    );
}

pub fn color_in_event(
    env: &Env,
    miner: &Address,
    to_address: &Address,
    color: u32,
    indexes_length: u32,
) {
    env.events().publish(
        (symbol_short!("color_in"), miner.clone(), to_address.clone()),
        (color, indexes_length),
    );
}

pub fn offer_match(env: &Env, sell_hash: &BytesN<32>, sell_owner: &Address, buy_hash: &BytesN<32>, buy_owner: &Address) {
    env.events().publish(
        (
            Symbol::new(&env, "offer_match"),
            sell_hash.clone(),
            sell_owner,
        ),
        (buy_hash.clone(), buy_owner),
    );
}

pub fn offer_match_sell_asset(env: &Env, sell_asset: &Address, sell_owner: &Address, buy_hash: &BytesN<32>, offer_index: u32) {
    env.events().publish(
        (
            Symbol::new(&env, "offer_match_sell_asset"),
            sell_asset,
            sell_owner,
        ),
        (buy_hash.clone(), offer_index),
    );
}

pub fn asset_offer_post(env: &Env, buy_asset: &Address, buy_asset_owner: &Address, hash: &BytesN<32>, amount: i128) {
    env.events().publish(
        (
            Symbol::new(&env, "asset_offer_post"),
            buy_asset,
            buy_asset_owner,
        ),
        (amount, hash.clone()),
    );
}

pub fn glyph_offer_post(env: &Env, sell_hash: &BytesN<32>, sell_owner: &Address, offer: Offer) {
    env.events().publish(
        (
            Symbol::new(&env, "glyph_offer_post"),
            sell_hash.clone(),
            sell_owner,
        ),
        (offer, ),
    );
}

pub fn glyph_offer_delete(env: &Env, hash: &BytesN<32>, owner: &Address, offer: Offer, idx: u32) {
    env.events().publish(
        (
            Symbol::new(&env, "glyph_offer_delete"),
            hash.clone(),
            owner,
        ),
        (offer, idx),
    );
}

pub fn glyph_offer_delete_all(env: &Env, hash: &BytesN<32>, owner: &Address) {
    env.events().publish(
        (
            Symbol::new(&env, "glyph_offer_delete_all"),
            hash.clone(),
            owner,
        ),
        ((), ),
    );
}

pub fn asset_offer_delete(env: &Env, asset: &Address, asset_owner: &Address, hash: &BytesN<32>, amount: i128, idx: u32) {
    env.events().publish(
        (
            Symbol::new(&env, "asset_offer_delete"),
            asset,
            asset_owner,
        ),
        (amount, hash.clone(), idx),
    );
}