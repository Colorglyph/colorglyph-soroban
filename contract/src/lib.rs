#![no_std]

mod token {
    soroban_sdk::contractimport!(file = "./soroban_token_spec.wasm");
}

pub mod colorglyph;
mod colors;
mod glyphs;
mod trades;
pub mod types;

mod testutils;
mod colors_test;
mod glyphs_test;
mod trades_test;