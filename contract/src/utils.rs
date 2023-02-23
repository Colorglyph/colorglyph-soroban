use crate::types::{Error, StorageKey};
use soroban_sdk::{panic_with_error, Address, BytesN, Env};

pub fn verify_glyph_ownership(env: &Env, from: Address, glyph_hash: BytesN<32>) {
    let glyph_owner: Address = env
        .storage()
        .get(&StorageKey::GlyphOwner(glyph_hash))
        .unwrap_or_else(|| panic_with_error!(env, Error::NotAuthorized))
        .unwrap();

    if glyph_owner != from {
        panic_with_error!(env, Error::NotAuthorized);
    }
}
