use soroban_sdk::{panic_with_error, Address, Bytes, BytesN, Env, Vec};

use crate::{
    types::{Error, Glyph, MinerColorAmount, StorageKey},
    utils::{colors_mint_or_burn, color_to_rgb},
};

// TODO
// Limit number of unique miner addresses in a mint `colors` Vec

pub fn glyph_mint(
    env: &Env,
    minter: Address,
    to: Option<Address>,
    colors: Vec<(Address, Vec<(u32, Vec<u32>)>)>,
    width: u32,
) -> BytesN<32> {
    minter.require_auth();

    let mut b_palette = Bytes::new(&env);
    let mut m_palette: Vec<MinerColorAmount> = Vec::new(&env);

    // TODO
    // better error for not enough colors
    // should we error if there's a dupe index? (will result in burned colors)
    // Need to ensure hash gen is consistent when duping indexes or mixing in white/missing pixels

    for (miner_address, color_indexes) in colors.iter_unchecked() {
        for (color, indexes) in color_indexes.iter_unchecked() {
            m_palette.push_back(MinerColorAmount(miner_address.clone(), color, indexes.len()));

            // TODO
            // This is expensive and it's only for getting the sha256 hash. We should find a cheaper way to derive a hash from the Glyph colors themselves.
            // RawVal maybe?
            // Ordering is important so you can't just hash the arg directly
            // May be able to improve perf by ordering indexes (and maybe reversing them so we extend and then insert vs lots of inserts?)

            for index in indexes.iter_unchecked() {
                // We need to extend the length of the palette
                if (b_palette.len() / 3) <= index {
                    // Start wherever we have data .. wherever we need data
                    for i in (b_palette.len() / 3)..=index {
                        // If this is the section we're interested in filling, just fill
                        if i == index {
                            b_palette.insert_from_slice(index * 3, &color_to_rgb(color));
                        }
                        // Push empty white pixels
                        // NOTE: this is a "free" way to use white pixels atm
                        else {
                            b_palette.extend_from_slice(&[255; 3]);
                        }
                    }
                }
                // If the bytes already exist just fill them in
                else {
                    b_palette.copy_from_slice(index, &color_to_rgb(color));
                }
            }
        }
    }

    // TODO
    // should the hash also include something with the width? Otherwise two identical palettes with different widths would clash

    let hash = env.crypto().sha256(&b_palette);

    let is_owned = env.storage().has(&StorageKey::GlyphOwner(hash.clone()));

    if is_owned {
        panic_with_error!(env, Error::NotEmpty);
    } else {
        // Save the glyph to storage {glyph hash: Glyph}
        env.storage().set(
            &StorageKey::Glyph(hash.clone()),
            &Glyph {
                width,
                length: b_palette.len() / 3, // because there are 3 values per color
                colors,
            },
        );

        let to = match to {
            None => minter.clone(),
            Some(address) => address,
        };

        // Save the glyph owner to storage {glyph hash: Address}
        env.storage()
            .set(&StorageKey::GlyphOwner(hash.clone()), &to);
    }

    let is_made = env.storage().has(&StorageKey::GlyphMinter(hash.clone()));

    if !is_made {
        // Save the glyph minter to storage {glyph hash: Address}
        env.storage()
            .set(&StorageKey::GlyphMinter(hash.clone()), &minter);
    }

    // Remove the colors from the owner
    colors_mint_or_burn(&env, &minter, &m_palette, false);

    hash
}

pub fn glyph_get(env: &Env, hash: BytesN<32>) -> Result<Glyph, Error> {
    env.storage()
        .get(&StorageKey::Glyph(hash.clone()))
        .ok_or(Error::NotFound)?
        .unwrap()
}

// TODO
// transfer glyph fn

pub fn glyph_scrape(
    env: &Env,
    owner: Address,
    to: Option<Address>,
    hash: BytesN<32>,
) {
    owner.require_auth();

    // TODO
    // Do we need to close any open sell offers (especially from the current owner)? `StorageKey::GlyphOffer`

    let glyph_owner: Address = env
        .storage()
        .get(&StorageKey::GlyphOwner(hash.clone()))
        .unwrap_or_else(|| panic_with_error!(env, Error::NotFound))
        .unwrap();

    if glyph_owner != owner {
        panic_with_error!(env, Error::NotAuthorized);
    }

    let glyph = glyph_get(&env, hash.clone()).unwrap();
    let mut m_palette: Vec<MinerColorAmount> = Vec::new(&env);

    for (miner_address, color_indexes) in glyph.colors.iter_unchecked() {
        for (color, indexes) in color_indexes.iter_unchecked() {
            m_palette.push_back(MinerColorAmount(miner_address.clone(), color, indexes.len()));
        }
    }

    let to = match to {
        None => owner.clone(),
        Some(address) => address,
    };

    // Add the colors to the owner
    colors_mint_or_burn(&env, &to, &m_palette, true);

    // Remove glyph
    env.storage().remove(&StorageKey::Glyph(hash.clone()));

    // Remove glyph owner
    env.storage().remove(&StorageKey::GlyphOwner(hash.clone()));
}

/*
NOTE: mint `colors` argument structure
[
    [
        miner_ABC,
        [
            [
                color_123,
                [
                    1, 2, 3, ...indexes
                ]
            ],
            [
                color_456,
                [
                    4, 5, 6, ...indexes
                ]
            ]
        ]
    ],
    [
        miner_DEF,
        [
            [
                color_123,
                [
                    7, 8, 9, ...indexes
                ]
            ],
            [
                color_456,
                [
                    10, 11, 12, ...indexes
                ]
            ]
        ]
    ]
]
*/