#![no_std]

/* TODO
Likely including unnecessary auth or at least auth arguments (use `.require_auth_for_args`) (token interface includes its own auth)
Consider implementing fewer optional arguments at the cost of argument duplication?
Rethink bumps, many times (every time) these should be handled as separate ops vs within the executable
Ensure we're appropriately using the 3 different storage types
Implement events and ensure events aren't included in loops vs just single call events.
    They need to exist, they don't need to be overly verbose or granular
    Also make sure we're sending sufficient information. I think I saw a couple events not sending sufficient data to be useful to an indexer
Ensure fully verifying necessary ownerships
*/

// NOTE 20 storage writes is very limiting atm

mod contract;
mod events;
mod interface;
mod storage;
pub mod types;

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
