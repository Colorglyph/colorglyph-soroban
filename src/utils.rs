use crate::types::{Error, StorageKey, MinerOwnerColor, MinerColorAmount};
use soroban_sdk::{panic_with_error, Address, BytesN, Env, Vec};

pub fn hex_to_rgb(hex: u32) -> [u8; 3] {
    let a: [u8; 4] = hex.to_le_bytes();
    let mut b = [0; 3];

    b.copy_from_slice(&a[..3]);

    b
}

pub fn colors_mint_or_burn(env: &Env, from: &Address, colors: &Vec<MinerColorAmount>, mint: bool) {
    for color in colors.iter_unchecked() {
        let MinerColorAmount(miner_address, hex, amount) = color;
        let from_color = MinerOwnerColor(miner_address, from.clone(), hex);
        let current_from_amount = env.storage().get(&from_color).unwrap_or(Ok(0)).unwrap();

        env.storage().set(
            &from_color,
            &if mint {
                current_from_amount + amount
            } else {
                current_from_amount - amount
            },
        );
    }
}

pub fn glyph_verify_ownership(env: &Env, from: Address, glyph_hash: BytesN<32>) {
    let glyph_owner: Address = env
        .storage()
        .get(&StorageKey::GlyphOwner(glyph_hash))
        .unwrap_or_else(|| panic_with_error!(env, Error::NotAuthorized))
        .unwrap();

    if glyph_owner != from {
        panic_with_error!(env, Error::NotAuthorized);
    }
}
