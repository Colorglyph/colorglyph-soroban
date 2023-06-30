use crate::types::{Error, MinerColorAmount, MinerOwnerColor, StorageKey};
use soroban_sdk::{panic_with_error, Address, BytesN, Env, Vec};

pub fn color_to_rgb(color: u32) -> [u8; 3] {
    let a = color.to_le_bytes();

    [a[0], a[1], a[2]]
}

pub fn colors_mint_or_burn(env: &Env, from: &Address, colors: &Vec<MinerColorAmount>, mint: bool) {
    for miner_color_amount in colors.iter_unchecked() {
        let MinerColorAmount(miner, color, amount) = miner_color_amount;
        let miner_owner_color = MinerOwnerColor(miner, from.clone(), color);
        let current_from_amount = env.storage().get(&miner_owner_color).unwrap_or(Ok(0)).unwrap();

        env.storage().set(
            &miner_owner_color,
            &if mint {
                current_from_amount + amount
            } else {
                current_from_amount - amount
            }
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
