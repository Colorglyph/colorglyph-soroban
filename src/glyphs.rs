// use std::println;
// extern crate std;

use soroban_sdk::{panic_with_error, Address, Bytes, BytesN, Env, Vec};

use crate::{
    colors::colors_mint_or_burn,
    types::{Error, Glyph, MinerColorAmount, StorageKey},
};

// TODO
// Limit number of unique miner addresses in a mint `colors` Vec

pub fn glyph_mint(
    env: &Env,
    minter: Address,
    to: Option<Address>,
    mut colors: Vec<(Address, Vec<(u32, Vec<u32>)>)>,
    width: u32,
    hash: Option<BytesN<32>>,
    mint: bool,
) -> BytesN<32> {
    minter.require_auth();

    let mut b_palette = Bytes::new(&env);
    let mut m_palette: Vec<MinerColorAmount> = Vec::new(&env);

    let glyph_hash: BytesN<32>;

    // TODO
    // better error for not enough colors
    // should we error if there's a dupe index? (will result in burned colors)
    // Need to ensure hash gen is consistent when duping indexes or mixing in white/missing pixels
    // Should we enable some concept of ranging between 2 indexs vs listing out all the indexes? 0..=5 vs 0,1,2,3,4,5
    // Support progressive minting

    match hash {
        Some(hash) => {
            // TODO this is potentiall a big read. Might be better to split the `colors` from the rest of the Glyph data until the final `minted: true`
            match env
                .storage()
                .get::<StorageKey, Glyph>(&StorageKey::Glyph(hash.clone()))
            {
                // progressive mint
                Some(glyph_error) => {
                    match glyph_error {
                        Ok(glyph) => {
                            let is_minted =
                                env.storage().has(&StorageKey::GlyphMinter(hash.clone()));

                            // glyph is minted
                            if is_minted {
                                panic_with_error!(env, Error::NotEmpty);
                            }

                            // add onto exisiting unminted glyph
                            glyph_verify_ownership(env, minter.clone(), hash.clone());

                            for color in glyph.colors.iter_unchecked() {
                                colors.push_back(color);
                            }

                            // delete previous unminted glyph
                            env.storage().remove(&StorageKey::Glyph(hash.clone()));

                            // genereate glyph hash
                            glyph_hash = generate_glyph_hash(
                                env,
                                &width,
                                &mut b_palette,
                                &mut m_palette,
                                &colors,
                            );
                        }
                        _ => panic!(),
                    }
                }
                None => {
                    panic_with_error!(env, Error::NotFound);
                }
            }
        }
        // new mint
        None => {
            // genereate glyph hash
            glyph_hash = generate_glyph_hash(env, &width, &mut b_palette, &mut m_palette, &colors);
        }
    }

    // Save the glyph to storage {glyph hash: Glyph}
    env.storage().set(
        &StorageKey::Glyph(glyph_hash.clone()),
        &Glyph {
            width,
            length: (b_palette.len() - 4) / 3, // -4 because we're appending width, /3 because there are 3 u8 values per u32 color
            colors,
        },
    );

    // Save the glyph owner to storage {glyph hash: Address}
    env.storage().set(
        &StorageKey::GlyphOwner(glyph_hash.clone()),
        &match to {
            None => minter.clone(),
            Some(address) => address,
        },
    );

    if mint {
        // Remove the colors from the owner
        // NOTE this will allow folks to build unminted glyphs utilizing the same colors
        // which is fine as long as we don't allow sales or scrapes that mint colors of unminted glyphs
        colors_mint_or_burn(&env, &minter, &m_palette, false);

        // Save the glyph minter to storage {glyph hash: Address}
        env.storage()
            .set(&StorageKey::GlyphMinter(glyph_hash.clone()), &minter);
    }

    glyph_hash
}

// TODO
// transfer glyph fn

pub fn glyph_scrape(env: &Env, owner: Address, to: Option<Address>, hash: BytesN<32>) {
    owner.require_auth();

    // NOTE
    // Support progressive scraping

    glyph_verify_ownership(env, owner.clone(), hash.clone());

    let mut glyph = env
        .storage()
        .get::<StorageKey, Glyph>(&StorageKey::Glyph(hash.clone()))
        .unwrap_or_else(|| panic_with_error!(env, Error::NotFound))
        .unwrap();
    let mut m_palette: Vec<MinerColorAmount> = Vec::new(&env);

    // this 16 number should likely be variable as it's related to the progressive scraping
    while glyph.colors.len() > 0 && m_palette.len() < 16 {
        match glyph.colors.pop_back() {
            Some(result) => match result {
                Ok((miner_address, mut color_indexes)) => {
                    match color_indexes.pop_back() {
                        Some(result) => match result {
                            Ok((color, indexes)) => {
                                m_palette.push_back(MinerColorAmount(
                                    miner_address.clone(),
                                    color,
                                    indexes.len(),
                                ));
                            }
                            _ => panic!(),
                        },
                        None => {}
                    }

                    // If we've got leftover colors push them back into the Glyph
                    if color_indexes.len() > 0 {
                        glyph
                            .colors
                            .push_back((miner_address.clone(), color_indexes));
                    }
                }
                _ => panic!(),
            },
            None => {}
        }
    }

    let to = match to {
        None => owner.clone(),
        Some(address) => address,
    };

    // Add the colors to the owner
    colors_mint_or_burn(&env, &to, &m_palette, true);

    if glyph.colors.len() == 0 {
        // Remove glyph
        env.storage().remove(&StorageKey::Glyph(hash.clone()));

        // Remove glyph owner
        env.storage().remove(&StorageKey::GlyphOwner(hash.clone()));

        // remove all glyph sell offers
        env.storage().remove(&StorageKey::GlyphOffer(hash.clone()));
    } else {
        // Save the glyph to storage
        env.storage().set(&StorageKey::Glyph(hash.clone()), &glyph);
    }
}

pub fn glyph_verify_ownership(env: &Env, from: Address, glyph_hash: BytesN<32>) {
    let glyph_owner = env
        .storage()
        .get::<StorageKey, Address>(&StorageKey::GlyphOwner(glyph_hash))
        .unwrap_or_else(|| panic_with_error!(env, Error::NotFound))
        .unwrap();

    if glyph_owner != from {
        panic_with_error!(env, Error::NotAuthorized);
    }
}

pub fn generate_glyph_hash(
    env: &Env,
    width: &u32,
    b_palette: &mut Bytes,
    m_palette: &mut Vec<MinerColorAmount>,
    colors: &Vec<(Address, Vec<(u32, Vec<u32>)>)>,
) -> BytesN<32> {
    for (miner_address, color_indexes) in colors.iter_unchecked() {
        for (color, indexes) in color_indexes.iter_unchecked() {
            m_palette.push_back(MinerColorAmount(
                miner_address.clone(),
                color,
                indexes.len(),
            ));

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
                            let slice: [u8; 3] = color.to_le_bytes()[0..3].try_into().unwrap();
                            b_palette.insert_from_slice(index * 3, &slice);
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
                    let slice: [u8; 3] = color.to_le_bytes()[0..3].try_into().unwrap();
                    b_palette.copy_from_slice(index, &slice);
                }
            }
        }
    }

    // NOTE
    // should the hash also include something with the width? Otherwise two identical palettes with different widths would clash

    b_palette.extend_from_slice(&width.to_le_bytes());
    env.crypto().sha256(&b_palette)
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
