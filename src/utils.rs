use crate::types::{Error, StorageKey};
use soroban_sdk::{panic_with_error, Address, BytesN, Env};

pub fn hex_to_rgb(hex: u32) -> [u8; 3] {
    let a: [u8; 4] = hex.to_le_bytes();
    let mut b = [0; 3];

    b.copy_from_slice(&a[..3]);

    b
}

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
