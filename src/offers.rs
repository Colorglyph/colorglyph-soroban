// use std::println;
// extern crate std;

use fixed_point_math::FixedPoint;
use soroban_sdk::{token, vec, Address, Env, Vec};

use crate::{
    contract::MAX_ENTRY_LIFETIME,
    glyphs::glyph_verify_ownership,
    types::{Error, Glyph, Offer, OfferCreate, StorageKey},
};

/* TODO
Document everything clearly
Break it up into individual functions to improve legibility
I'm not convinced it's terribly efficient or that we aren't over doing the match nesting hell
Support progressive offers? This would allow increasing the miner addresses per glyph
    Only really required when processing royalties.
    Also it only involves miners not unique colors so the cap is less concerning.
    How many miners are you likely to have really? 15 is the ceiling atm which is pretty high imo
Tweak MINTER_ROYALTY_RATE and MINER_ROYALTY_RATE values
Place caps on the number of GlyphOffer and AssetOffer Vec lengths
    how many sell offers can a Glyph owner open?
    how many identical glyph:asset:amount offers can be open?
*/

const MINTER_ROYALTY_RATE: i128 = 3; // 3%
const MINER_ROYALTY_RATE: i128 = 2; // 2%

pub fn offer_post(env: &Env, sell: Offer, buy: Offer) -> Result<(), Error> {
    // sell glyph
    // lookup if someone is selling what you're buying
    // sell asset
    // lookup if someone is selling what you're buying

    // Lookup if there are any open buy offers for what we're selling
    match &buy {
        // buying a glyph
        Offer::Glyph(buy_glyph_hash) => {
            let buy_glyph_offer_key = StorageKey::GlyphOffer(buy_glyph_hash.clone());
            let mut offers = env
                .storage()
                .persistent()
                .get::<StorageKey, Vec<Offer>>(&buy_glyph_offer_key)
                .unwrap_or(vec![&env]);
            env.storage()
                .persistent()
                .bump(&buy_glyph_offer_key, MAX_ENTRY_LIFETIME);

            match offers.binary_search(match &sell {
                Offer::Glyph(sell_glyph_hash) => Offer::Glyph(sell_glyph_hash.clone()),
                Offer::AssetSell(_, sell_asset_address, amount) => {
                    Offer::Asset(sell_asset_address.clone(), *amount)
                }
                _ => return Err(Error::NotPermitted),
            }) {
                Ok(offer_index) => {
                    let buy_glyph_owner_key = StorageKey::GlyphOwner(buy_glyph_hash.clone());
                    let buy_glyph_owner_address = env
                        .storage()
                        .persistent()
                        .get::<StorageKey, Address>(&buy_glyph_owner_key)
                        .ok_or(Error::NotFound)?;

                    env.storage()
                        .persistent()
                        .bump(&buy_glyph_owner_key, MAX_ENTRY_LIFETIME);

                    offers.remove(offer_index);

                    env.storage()
                        .persistent()
                        .set(&buy_glyph_offer_key, &offers);

                    match &sell {
                        Offer::Glyph(sell_glyph_hash) => {
                            let sell_glyph_offer_key =
                                StorageKey::GlyphOffer(sell_glyph_hash.clone());
                            let sell_glyph_owner_key =
                                StorageKey::GlyphOwner(sell_glyph_hash.clone());
                            let sell_glyph_owner_address =
                                glyph_verify_ownership(env, &sell_glyph_owner_key);

                            // transfer ownership from seller to buyer
                            env.storage()
                                .persistent()
                                .set(&sell_glyph_owner_key, &buy_glyph_owner_address);

                            // transfer ownership from buyer to seller
                            env.storage()
                                .persistent()
                                .set(&buy_glyph_owner_key, &sell_glyph_owner_address);

                            env.storage()
                                .persistent()
                                .bump(&sell_glyph_owner_key, MAX_ENTRY_LIFETIME);
                            env.storage()
                                .persistent()
                                .bump(&buy_glyph_owner_key, MAX_ENTRY_LIFETIME);

                            // remove all glyph seller offers
                            env.storage().persistent().remove(&sell_glyph_offer_key);

                            // remove all glyph buyer offers
                            env.storage().persistent().remove(&buy_glyph_offer_key);

                            Ok(())
                        }
                        Offer::AssetSell(sell_asset_owner_address, sell_asset_address, amount) => {
                            let buy_glyph_key = StorageKey::Glyph(buy_glyph_hash.clone());
                            let buy_glyph_minter_key =
                                StorageKey::GlyphMinter(buy_glyph_hash.clone());

                            // Might want to make a map of payees to reduce or eliminate piecemeal payments (e.g. over lap between minter and miner)
                            let mut leftover_amount = *amount;

                            // Get glyph
                            let buy_glyph = env
                                .storage()
                                .persistent()
                                .get::<StorageKey, Glyph>(&buy_glyph_key)
                                .ok_or(Error::NotFound)?;
                            let buy_glyph_minter_address = env
                                .storage()
                                .persistent()
                                .get::<StorageKey, Address>(&buy_glyph_minter_key)
                                .ok_or(Error::NotFound)?;

                            env.storage()
                                .persistent()
                                .bump(&buy_glyph_key, MAX_ENTRY_LIFETIME);
                            env.storage()
                                .persistent()
                                .bump(&buy_glyph_minter_key, MAX_ENTRY_LIFETIME);

                            // Pay the glyph minter their cut
                            let minter_amount =
                                MINTER_ROYALTY_RATE.fixed_mul_ceil(*amount, 100).unwrap();

                            let token = token::Client::new(env, &sell_asset_address);

                            if *sell_asset_owner_address != buy_glyph_minter_address {
                                token.transfer(
                                    &sell_asset_owner_address,
                                    &buy_glyph_minter_address,
                                    &minter_amount,
                                );

                                leftover_amount -= minter_amount;
                            }

                            // Loop over miners
                            // NOTE currently can support 17 miners
                            for (miner_address, colors_indexes) in buy_glyph.colors.iter() {
                                let mut color_count: u32 = 0;

                                // Count colors per miner
                                for (_, indexes) in colors_indexes.iter() {
                                    color_count += indexes.len();
                                }

                                let miner_amount = MINER_ROYALTY_RATE
                                    .fixed_mul_ceil(*amount, 100)
                                    .unwrap()
                                    .fixed_mul_ceil(color_count as i128, buy_glyph.length as i128)
                                    .unwrap();

                                // Determine their percentage of whole
                                // Derive their share of the amount
                                // Make payment
                                if *sell_asset_owner_address != miner_address {
                                    token.transfer(
                                        &sell_asset_owner_address,
                                        &miner_address,
                                        &miner_amount,
                                    );
                                    leftover_amount -= miner_amount;
                                }
                            }

                            // Transfer Asset from Glyph taker to Glyph giver
                            if *sell_asset_owner_address != buy_glyph_owner_address {
                                token.transfer(
                                    &sell_asset_owner_address,
                                    &buy_glyph_owner_address,
                                    &leftover_amount,
                                );
                            }

                            // Transfer ownership of Glyph from glyph giver to Glyph taker
                            env.storage()
                                .persistent()
                                .set(&buy_glyph_owner_key, &sell_asset_owner_address);

                            env.storage()
                                .persistent()
                                .bump(&buy_glyph_owner_key, MAX_ENTRY_LIFETIME);

                            // remove all other sell offers for this glyph
                            env.storage().persistent().remove(&buy_glyph_offer_key);

                            Ok(())
                        }
                        _ => Err(Error::NotPermitted),
                    }
                }
                _ => match &sell {
                    Offer::Glyph(sell_glyph_hash) => {
                        offer_post_create(env, OfferCreate::Glyph(sell_glyph_hash.clone(), buy))
                    }
                    Offer::AssetSell(sell_asset_owner_address, sell_asset_address, amount) => {
                        offer_post_create(
                            env,
                            OfferCreate::Asset(
                                buy_glyph_hash.clone(),
                                sell_asset_owner_address.clone(),
                                sell_asset_address.clone(),
                                *amount,
                            ),
                        )
                    }
                    _ => Err(Error::NotPermitted),
                },
            }
        }
        // buying an asset
        Offer::Asset(buy_asset_address, amount) => {
            match &sell {
                Offer::Glyph(sell_glyph_hash) => {
                    let buy_asset_offer_key = StorageKey::AssetOffer(
                        sell_glyph_hash.clone(),
                        buy_asset_address.clone(),
                        *amount,
                    );
                    let mut offers = env
                        .storage()
                        .persistent()
                        .get::<StorageKey, Vec<Address>>(&buy_asset_offer_key)
                        .unwrap_or(vec![&env]);

                    if offers.is_empty() {
                        return offer_post_create(
                            env,
                            OfferCreate::Glyph(sell_glyph_hash.clone(), buy),
                        );
                    }

                    env.storage()
                        .persistent()
                        .bump(&buy_asset_offer_key, MAX_ENTRY_LIFETIME);

                    let sell_glyph_owner_key = StorageKey::GlyphOwner(sell_glyph_hash.clone());
                    let sell_glyph_owner_address =
                        glyph_verify_ownership(env, &sell_glyph_owner_key);
                    let sell_glyph_minter_key = StorageKey::GlyphMinter(sell_glyph_hash.clone());
                    let sell_glyph_offer_key = StorageKey::GlyphOffer(sell_glyph_hash.clone());
                    let sell_glyph_key = StorageKey::Glyph(sell_glyph_hash.clone());

                    // TODO this next block is essentially a dupe from up above, this should be broken up into a separate function
                    // Might want to make a map of payees to reduce or eliminate piecemeal payments (e.g. over lap between minter and miner)
                    let mut leftover_amount = *amount;

                    // Get glyph
                    let sell_glyph = env
                        .storage()
                        .persistent()
                        .get::<StorageKey, Glyph>(&sell_glyph_key)
                        .ok_or(Error::NotFound)?;
                    let sell_glyph_minter = env
                        .storage()
                        .persistent()
                        .get::<StorageKey, Address>(&sell_glyph_minter_key)
                        .ok_or(Error::NotFound)?;

                    env.storage()
                        .persistent()
                        .bump(&sell_glyph_key, MAX_ENTRY_LIFETIME);
                    env.storage()
                        .persistent()
                        .bump(&sell_glyph_minter_key, MAX_ENTRY_LIFETIME);

                    // Pay the glyph minter their cut
                    let minter_amount = MINTER_ROYALTY_RATE.fixed_mul_ceil(*amount, 100).unwrap();
                    let token = token::Client::new(env, &buy_asset_address);

                    token.transfer(
                        &env.current_contract_address(),
                        &sell_glyph_minter,
                        &minter_amount,
                    );

                    leftover_amount -= minter_amount;

                    // Loop over miners
                    // NOTE currently can support 15 miners
                    for (miner_address, colors_indexes) in sell_glyph.colors.iter() {
                        let mut color_count: u32 = 0;

                        // Count colors per miner
                        for (_, indexes) in colors_indexes.iter() {
                            color_count += indexes.len();
                        }

                        let miner_amount = MINER_ROYALTY_RATE
                            .fixed_mul_ceil(*amount, 100)
                            .unwrap()
                            .fixed_mul_ceil(i128::from(color_count), i128::from(sell_glyph.length))
                            .unwrap();

                        // Determine their percentage of whole
                        // Derive their share of the amount
                        // Make payment
                        token.transfer(
                            &env.current_contract_address(),
                            &miner_address,
                            &miner_amount,
                        );

                        leftover_amount -= miner_amount;
                    }

                    // Transfer Asset from Glyph taker to Glyph giver
                    token.transfer(
                        &env.current_contract_address(),
                        &sell_glyph_owner_address,
                        &leftover_amount,
                    );

                    // Remove Asset counter offer
                    let buy_asset_owner = offers.pop_front().unwrap();

                    if offers.is_empty() {
                        env.storage().persistent().remove(&buy_asset_offer_key);
                    } else {
                        env.storage()
                            .persistent()
                            .set(&buy_asset_offer_key, &offers);

                        env.storage()
                            .persistent()
                            .bump(&buy_asset_offer_key, MAX_ENTRY_LIFETIME);
                    }

                    // Transfer ownership of Glyph from Glyph giver to Glyph taker
                    env.storage()
                        .persistent()
                        .set(&sell_glyph_owner_key, &buy_asset_owner);

                    env.storage()
                        .persistent()
                        .bump(&sell_glyph_owner_key, MAX_ENTRY_LIFETIME);

                    // Remove all other sell offers for this glyph
                    env.storage().persistent().remove(&sell_glyph_offer_key);

                    Ok(())
                }
                _ => Err(Error::NotPermitted),
            }
        }
        _ => Err(Error::NotPermitted),
    }
}

fn offer_post_create(env: &Env, offer: OfferCreate) -> Result<(), Error> {
    match offer {
        OfferCreate::Glyph(sell_glyph_hash, buy) => {
            let sell_glyph_owner_key = StorageKey::GlyphOwner(sell_glyph_hash.clone());
            glyph_verify_ownership(env, &sell_glyph_owner_key);
            let sell_glyph_offer_key = StorageKey::GlyphOffer(sell_glyph_hash);

            // Selling a Glyph
            // let glyph_offer_key = StorageKey::GlyphOffer(offer_hash);
            let mut offers = env
                .storage()
                .persistent()
                .get::<StorageKey, Vec<Offer>>(&sell_glyph_offer_key)
                .unwrap_or(Vec::new(env));

            match offers.binary_search(&buy) {
                Err(offer_index) => offers.insert(offer_index, buy), // Buy can be an Asset or a Glyph
                _ => return Err(Error::NotEmpty),                    // Dupe
            }

            env.storage()
                .persistent()
                .set(&sell_glyph_offer_key, &offers);

            env.storage()
                .persistent()
                .bump(&sell_glyph_offer_key, MAX_ENTRY_LIFETIME);

            Ok(())
        }
        OfferCreate::Asset(
            buy_glyph_hash,
            sell_asset_owner_address,
            sell_asset_address,
            amount,
        ) => {
            let token = token::Client::new(env, &sell_asset_address);

            token.transfer(
                &sell_asset_owner_address,
                &env.current_contract_address(),
                &amount,
            );

            let sell_asset_offer_key =
                StorageKey::AssetOffer(buy_glyph_hash, sell_asset_address, amount);

            let mut offers = env
                .storage()
                .persistent()
                .get::<StorageKey, Vec<Address>>(&sell_asset_offer_key)
                .unwrap_or(Vec::new(env));

            if offers.contains(sell_asset_owner_address.clone()) {
                return Err(Error::NotEmpty);
            }

            offers.push_back(sell_asset_owner_address);

            env.storage()
                .persistent()
                .set(&sell_asset_offer_key, &offers);

            env.storage()
                .persistent()
                .bump(&sell_asset_offer_key, MAX_ENTRY_LIFETIME);

            Ok(())
        }
    }
}

pub fn offer_delete(env: &Env, sell: Offer, buy: Option<Offer>) -> Result<(), Error> {
    match sell {
        Offer::Glyph(glyph_hash) => {
            // Selling a Glyph (delete Glyph or Asset buy offer)
            let glyph_owner_key = StorageKey::GlyphOwner(glyph_hash.clone());
            glyph_verify_ownership(env, &glyph_owner_key);

            let glyph_hash_key = StorageKey::GlyphOffer(glyph_hash.clone());
            let mut offers = env
                .storage()
                .persistent()
                .get::<StorageKey, Vec<Offer>>(&glyph_hash_key)
                .ok_or(Error::NotFound)?;

            env.storage()
                .persistent()
                .bump(&glyph_hash_key, MAX_ENTRY_LIFETIME);

            match buy {
                Some(buy) => match offers.binary_search(buy) {
                    Ok(offer_index) => {
                        offers.remove(offer_index);

                        env.storage().persistent().set(&glyph_hash_key, &offers);

                        env.storage()
                            .persistent()
                            .bump(&glyph_hash_key, MAX_ENTRY_LIFETIME);

                        Ok(())
                    }
                    _ => Err(Error::NotFound),
                },
                None => {
                    env.storage().persistent().remove(&glyph_hash_key);
                    Ok(())
                }
            }
        }
        Offer::AssetSell(asset_owner_address, asset_address, amount) => {
            // Selling an Asset (delete Glyph buy offer)
            asset_owner_address.require_auth();

            match buy {
                Some(buy) => {
                    match buy {
                        Offer::Glyph(glyph_hash) => {
                            let asset_offer_key = StorageKey::AssetOffer(
                                glyph_hash.clone(),
                                asset_address.clone(),
                                amount,
                            );
                            let mut offers = env
                                .storage()
                                .persistent()
                                .get::<StorageKey, Vec<Address>>(&asset_offer_key)
                                .ok_or(Error::NotFound)?;

                            env.storage()
                                .persistent()
                                .bump(&asset_offer_key, MAX_ENTRY_LIFETIME);

                            match offers.binary_search(asset_owner_address.clone()) {
                                Ok(offer_index) => {
                                    let token = token::Client::new(env, &asset_address);

                                    token.transfer(
                                        &env.current_contract_address(),
                                        &asset_owner_address,
                                        &amount,
                                    );

                                    offers.remove(offer_index);

                                    if offers.is_empty() {
                                        env.storage().persistent().remove(&asset_offer_key);
                                    } else {
                                        env.storage().persistent().set(&asset_offer_key, &offers);

                                        env.storage()
                                            .persistent()
                                            .bump(&asset_offer_key, MAX_ENTRY_LIFETIME);
                                    }

                                    Ok(())
                                }
                                _ => Err(Error::NotFound),
                            }
                        }
                        _ => Err(Error::NotPermitted), // You cannot sell an Asset for an Asset
                    }
                }
                None => Err(Error::MissingBuy), // When deleting a Glyph offer for an Asset you must specify the buy offer (and it must be for a Glyph)
            }
        }
        _ => Err(Error::NotPermitted),
    }
}

pub fn offers_get(env: &Env, sell: Offer, buy: Option<Offer>) -> Result<(), Error> {
    match sell {
        Offer::Glyph(glyph_hash) => {
            // Selling a Glyph
            let glyph_hash_key = StorageKey::GlyphOffer(glyph_hash.clone());
            let offers = env
                .storage()
                .persistent()
                .get::<StorageKey, Vec<Offer>>(&glyph_hash_key)
                .ok_or(Error::NotFound)?;

            env.storage()
                .persistent()
                .bump(&glyph_hash_key, MAX_ENTRY_LIFETIME);

            match buy {
                Some(buy) => match offers.binary_search(buy) {
                    Ok(_) => Ok(()), // Found the buy offer
                    _ => Err(Error::NotFound),
                },
                _ => Ok(()), // There are buy offers for this Glyph
            }
        }
        Offer::Asset(asset_hash, amount) => {
            // Selling an Asset
            match buy {
                Some(buy) => {
                    match buy {
                        Offer::Glyph(glyph_hash) => {
                            let asset_offer_key = StorageKey::AssetOffer(
                                glyph_hash.clone(),
                                asset_hash.clone(),
                                amount,
                            );
                            env.storage()
                                .persistent()
                                .get::<StorageKey, Vec<Address>>(&asset_offer_key)
                                .ok_or(Error::NotFound)?;

                            env.storage()
                                .persistent()
                                .bump(&asset_offer_key, MAX_ENTRY_LIFETIME);

                            Ok(())
                        }
                        _ => Err(Error::NotPermitted), // You cannot sell an Asset for an Asset
                    }
                }
                _ => Err(Error::MissingBuy), // When looking up a Glyph offer for an Asset you must specify the buy offer (and it must be for a Glyph
            }
        }
        Offer::AssetSell(seller_address, asset_hash, amount) => {
            // Selling an Asset but check for a specific seller address
            match buy {
                Some(buy) => {
                    match buy {
                        Offer::Glyph(glyph_hash) => {
                            let asset_offer_key = StorageKey::AssetOffer(
                                glyph_hash.clone(),
                                asset_hash.clone(),
                                amount,
                            );
                            let offers = env
                                .storage()
                                .persistent()
                                .get::<StorageKey, Vec<Address>>(&asset_offer_key)
                                .ok_or(Error::NotFound)?;

                            if offers.contains(seller_address) {
                                env.storage()
                                    .persistent()
                                    .bump(&asset_offer_key, MAX_ENTRY_LIFETIME);

                                return Ok(());
                            }

                            Err(Error::NotFound)
                        }
                        _ => Err(Error::NotPermitted), // You cannot sell an Asset for an Asset
                    }
                }
                _ => Err(Error::MissingBuy), // When looking up a Glyph offer for an Asset you must specify the buy offer (and it must be for a Glyph
            }
        }
    }
}
