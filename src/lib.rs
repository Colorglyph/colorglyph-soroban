#![no_std]

// TODO
// Likely including unneccesary auth or at least auth arguments (use `.require_auth_for_args`) (token interface includes its own auth)
// Consider implementing fewer optional arguments at the cost of argument duplication?
// Rethink bumps, many times (every time) these should be handled as separate ops vs within the executible
// Ensure we're appropriately using the 3 different storage types
// Implement events
// Create an admin upgrade function
// Ensure fully verifying necesary ownerships

// NOTE
// 20 storage writes is very limiting atm

mod contract;
mod interface;
mod types;

mod colors;
mod glyphs;
mod offers;

#[path = "./tests/colors.rs"]
mod colors_test;
#[path = "./tests/glyphs.rs"]
mod glyphs_test;
// #[path = "./tests/misc.rs"]
// mod misc_test;
#[path = "./tests/offers.rs"]
mod offers_test;
