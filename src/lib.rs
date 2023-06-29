#![no_std]

mod contract;
mod interface;
mod storage;
mod types;

mod colors;
mod glyphs;
mod offers;
mod utils;

#[path = "./tests/colors.rs"]
mod colors_test;
#[path = "./tests/glyphs.rs"]
mod glyphs_test;
#[path = "./tests/misc.rs"]
mod misc_test;
#[path = "./tests/offers.rs"]
mod offers_test;
