// use std::println;
// extern crate std;

use fixed_point_math::FixedPoint;
use soroban_sdk::{panic_with_error, token, Address, Env, Vec};

use crate::{
    contract::MAX_ENTRY_LIFETIME,
    glyphs::glyph_verify_ownership,
    types::{Error, Glyph, Offer, OfferType, StorageKey},
};

// TODO
// Document everything clearly
// Break it up into individual functions to improve legibility
// I'm not convinced it's terribly efficient or that we aren't over doing the match nesting hell
// Place caps on the number of GlyphOffer and AssetOffer Vec lengths
// Create fn for removing all a glyph owners open sell offers
// Ensure we can't offer to sell a glyph, scrape it, then accept a buy offer
// What happens if we try and submit a dupe offer? (for both selling a glyph and selling an asset)

const MINTER_ROYALTY_RATE: i128 = 3;
const MINER_ROYALTY_RATE: i128 = 2;

pub fn offer_post(
    env: &Env,
    seller: Address,
    sell: OfferType,
    buy: OfferType,
) -> Result<(), Error> {
    seller.require_auth();

    /*
    existing counter offer
    yes
        sell glyph
            match is glyph
                take glyph, give glyph
            match is asset
                take asset, give glyph
        sell asset
            give asset, take glyph
    no
        sell glyph
            set glyph offer
        sell asset
            take asset into custody
            set asset offer (save glyph hash and asset amount)
    */

    // TODO
    // permit progressive offer matching? Only really required when processing royalties. Also it only involves miners not unique colors so the cap is less concerning. How many miners are you likely to have really? 15 is the ceiling atm which is pretty high imo

    match offers_get(env, buy.clone(), sell.clone()) {
        Ok(existing_offer) => {
            match existing_offer {
                // Found someone buying your sale with a Glyph (meaning sell is either a Glyph or Asset)
                Offer::Glyph(_, _, existing_offer_owner, existing_offer_hash) => {
                    match sell {
                        // sell glyph now for glyph
                        OfferType::Glyph(offer_hash) => {
                            glyph_verify_ownership(env, seller.clone(), offer_hash.clone());

                            let glyph_owner_key = StorageKey::GlyphOwner(offer_hash.clone());
                            let existing_glyph_owner_key =
                                StorageKey::GlyphOwner(existing_offer_hash.clone());

                            // transfer ownership from seller to buyer
                            env.storage()
                                .persistent()
                                .set(&glyph_owner_key, &existing_offer_owner);

                            // transfer ownership from buyer to seller
                            env.storage()
                                .persistent()
                                .set(&existing_glyph_owner_key, &seller);

                            env.storage()
                                .persistent()
                                .bump(&glyph_owner_key, MAX_ENTRY_LIFETIME);
                            env.storage()
                                .persistent()
                                .bump(&existing_glyph_owner_key, MAX_ENTRY_LIFETIME);

                            // remove all glyph seller offers
                            env.storage()
                                .persistent()
                                .remove(&StorageKey::GlyphOffer(offer_hash));

                            // remove all glyph buyer offers
                            env.storage()
                                .persistent()
                                .remove(&StorageKey::GlyphOffer(existing_offer_hash));
                        }
                        // sell asset now for glyph
                        OfferType::Asset(offer_hash, amount) => {
                            // START royalties
                            // Might want to make a map of payees to reduce or eliminate piecemeal payments
                            let mut leftover_amount = amount;

                            // Get glyph
                            let glyph_key = StorageKey::Glyph(existing_offer_hash.clone());
                            let glyph_minter_key =
                                StorageKey::GlyphMinter(existing_offer_hash.clone());
                            let glyph = env
                                .storage()
                                .persistent()
                                .get::<StorageKey, Glyph>(&glyph_key)
                                .unwrap_or_else(|| panic_with_error!(env, Error::NotFound));
                            let glyph_minter = env
                                .storage()
                                .persistent()
                                .get::<StorageKey, Address>(&glyph_minter_key)
                                .unwrap_or_else(|| panic_with_error!(env, Error::NotFound));

                            env.storage()
                                .persistent()
                                .bump(&glyph_key, MAX_ENTRY_LIFETIME);
                            env.storage()
                                .persistent()
                                .bump(&glyph_minter_key, MAX_ENTRY_LIFETIME);

                            // TODO
                            // if glyph_minter is existing_offer_owner don't make this payment

                            // pay the glyph minter their cut
                            let minter_amount =
                                MINTER_ROYALTY_RATE.fixed_mul_ceil(amount, 100).unwrap();

                            let token = token::Client::new(env, &offer_hash);
                            token.transfer(&seller, &glyph_minter, &minter_amount);

                            leftover_amount -= minter_amount;

                            // Loop over miners
                            // TODO currently can support 17 miners
                            for (miner_address, colors_indexes) in glyph.colors.iter() {
                                let mut color_count: u32 = 0;

                                // Count colors per miner
                                for (_, indexes) in colors_indexes.iter() {
                                    color_count += indexes.len();
                                }

                                let miner_amount = MINER_ROYALTY_RATE
                                    .fixed_mul_ceil(amount, 100)
                                    .unwrap()
                                    .fixed_mul_ceil(
                                        i128::from(color_count),
                                        i128::from(glyph.length),
                                    )
                                    .unwrap();

                                // TODO
                                // if miner_address is existing_offer_owner don't make this payment

                                // Determine their percentage of whole
                                // Derive their share of the amount
                                // Make payment
                                token.transfer(&seller, &miner_address, &miner_amount);

                                leftover_amount -= miner_amount;
                            }

                            // xfer_from Asset from Glyph taker to Glyph giver
                            token.transfer(&seller, &existing_offer_owner, &leftover_amount);
                            // END royalties

                            // transfer ownership of Glyph from glyph giver to Glyph taker
                            let glyph_owner_key =
                                StorageKey::GlyphOwner(existing_offer_hash.clone());

                            env.storage().persistent().set(&glyph_owner_key, &seller);
                            env.storage()
                                .persistent()
                                .bump(&glyph_owner_key, MAX_ENTRY_LIFETIME);

                            // remove all other sell offers for this glyph
                            env.storage()
                                .persistent()
                                .remove(&StorageKey::GlyphOffer(existing_offer_hash));
                        }
                    }
                }
                // Found someone buying your sale with an Asset (meaning sell is a Glyph)
                Offer::Asset(mut offers, glyph_hash, asset_address, amount) => {
                    glyph_verify_ownership(env, seller.clone(), glyph_hash.clone());

                    // START royalties
                    // Might want to make a map of payees to reduce or eliminate piecemeal payments
                    let mut leftover_amount = amount;

                    // Get glyph
                    let glyph_key = StorageKey::Glyph(glyph_hash.clone());
                    let glyph = env
                        .storage()
                        .persistent()
                        .get::<StorageKey, Glyph>(&glyph_key)
                        .unwrap_or_else(|| panic_with_error!(env, Error::NotFound));
                    let glyph_minter_key = StorageKey::GlyphMinter(glyph_hash.clone());
                    let glyph_minter = env
                        .storage()
                        .persistent()
                        .get::<StorageKey, Address>(&glyph_minter_key)
                        .unwrap_or_else(|| panic_with_error!(env, Error::NotFound));

                    env.storage()
                        .persistent()
                        .bump(&glyph_key, MAX_ENTRY_LIFETIME);
                    env.storage()
                        .persistent()
                        .bump(&glyph_minter_key, MAX_ENTRY_LIFETIME);

                    // TODO
                    // if glyph_minter is existing_offer_owner don't make this payment

                    // pay the glyph minter their cut
                    let minter_amount = MINTER_ROYALTY_RATE.fixed_mul_ceil(amount, 100).unwrap();

                    let token = token::Client::new(env, &asset_address);
                    token.transfer(
                        &env.current_contract_address(),
                        &glyph_minter,
                        &minter_amount,
                    );

                    leftover_amount -= minter_amount;

                    // Loop over miners
                    // TODO currently can support 15 miners
                    for (miner_address, colors_indexes) in glyph.colors.iter() {
                        let mut color_count: u32 = 0;

                        // Count colors per miner
                        for (_, indexes) in colors_indexes.iter() {
                            color_count += indexes.len();
                        }

                        let miner_amount = MINER_ROYALTY_RATE
                            .fixed_mul_ceil(amount, 100)
                            .unwrap()
                            .fixed_mul_ceil(i128::from(color_count), i128::from(glyph.length))
                            .unwrap();

                        // TODO
                        // if miner_address is existing_offer_owner don't make this payment

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

                    // xfer_from Asset from Glyph taker to Glyph giver
                    token.transfer(&env.current_contract_address(), &seller, &leftover_amount);
                    // END royalties

                    // remove Asset counter offer
                    let asset_offer_key =
                        StorageKey::AssetOffer(glyph_hash.clone(), asset_address, amount);
                    let offer_owner = offers.pop_front().unwrap();

                    if offers.is_empty() {
                        env.storage().persistent().remove(&asset_offer_key);
                    } else {
                        env.storage().persistent().set(&asset_offer_key, &offers);

                        env.storage()
                            .persistent()
                            .bump(&asset_offer_key, MAX_ENTRY_LIFETIME);
                    }

                    // transfer ownership of Glyph from glyph giver to Glyph taker
                    let glyph_owner_key = StorageKey::GlyphOwner(glyph_hash.clone());

                    env.storage()
                        .persistent()
                        .set(&glyph_owner_key, &offer_owner);

                    env.storage()
                        .persistent()
                        .bump(&glyph_owner_key, MAX_ENTRY_LIFETIME);

                    // remove all other sell offers for this glyph
                    env.storage()
                        .persistent()
                        .remove(&StorageKey::GlyphOffer(glyph_hash));
                }
            }
        }
        Err(_) => {
            match sell {
                OfferType::Glyph(offer_hash) => {
                    glyph_verify_ownership(env, seller, offer_hash.clone());

                    // Selling a Glyph
                    let glyph_offer_key = StorageKey::GlyphOffer(offer_hash);
                    let mut offers = env
                        .storage()
                        .persistent()
                        .get::<StorageKey, Vec<OfferType>>(&glyph_offer_key)
                        .unwrap_or(Vec::new(env));

                    match offers.binary_search(&buy) {
                        Result::Err(i) => offers.insert(i, buy), // buy can be an Asset or a Glyph
                        _ => panic_with_error!(env, Error::NotEmpty), // dupe
                    }

                    env.storage().persistent().set(&glyph_offer_key, &offers);

                    env.storage()
                        .persistent()
                        .bump(&glyph_offer_key, MAX_ENTRY_LIFETIME);
                }
                OfferType::Asset(asset_hash, amount) => {
                    // Buying a Glyph
                    match buy {
                        OfferType::Glyph(glyph_hash) => {
                            let token = token::Client::new(env, &asset_hash);

                            token.transfer(&seller, &env.current_contract_address(), &amount);

                            let asset_offers_key = StorageKey::AssetOffer(
                                glyph_hash.clone(),
                                asset_hash.clone(),
                                amount,
                            );
                            let mut offers = env
                                .storage()
                                .persistent()
                                .get::<StorageKey, Vec<Address>>(&asset_offers_key)
                                .unwrap_or(Vec::new(env));

                            offers.push_back(seller);

                            env.storage().persistent().set(&asset_offers_key, &offers);

                            env.storage()
                                .persistent()
                                .bump(&asset_offers_key, MAX_ENTRY_LIFETIME);
                        }
                        _ => panic_with_error!(env, Error::NotPermitted), // You cannot sell an Asset for an Asset
                    }
                }
            }
        }
    }

    Ok(())
}

pub fn offer_delete(env: &Env, seller: Address, sell: OfferType, buy: OfferType) {
    seller.require_auth();

    match offers_get(env, sell, buy) {
        Ok(existing_offer) => {
            match existing_offer {
                // Selling a Glyph
                Offer::Glyph(offer_index, mut offers, offer_owner, offer_hash) => {
                    // You cannot delete an offer for a glyph you are not the owner of
                    if offer_owner != seller {
                        panic_with_error!(env, Error::NotAuthorized);
                    }

                    offers.remove(offer_index);

                    let glyph_offer_key = StorageKey::GlyphOffer(offer_hash);

                    env.storage().persistent().set(&glyph_offer_key, &offers);

                    env.storage()
                        .persistent()
                        .bump(&glyph_offer_key, MAX_ENTRY_LIFETIME);
                }
                // Selling an Asset
                Offer::Asset(mut offers, glyph_hash, asset_address, amount) => {
                    match offers.first_index_of(seller.clone()) {
                        Some(offer_index) => {
                            let token = token::Client::new(env, &asset_address);

                            token.transfer(&env.current_contract_address(), &seller, &amount);

                            offers.remove(offer_index);

                            let asset_offer_key =
                                StorageKey::AssetOffer(glyph_hash, asset_address, amount);

                            if offers.is_empty() {
                                env.storage().persistent().remove(&asset_offer_key);
                            } else {
                                env.storage().persistent().set(&asset_offer_key, &offers);

                                env.storage()
                                    .persistent()
                                    .bump(&asset_offer_key, MAX_ENTRY_LIFETIME);
                            }
                        }
                        None => panic_with_error!(env, Error::NotFound),
                    }
                }
            }
        }
        _ => panic_with_error!(env, Error::NotFound),
    }
}

pub fn offers_get(env: &Env, sell: OfferType, buy: OfferType) -> Result<Offer, Error> {
    match sell {
        OfferType::Glyph(offer_hash) => {
            // Selling a Glyph
            let glyph_offer_key = StorageKey::GlyphOffer(offer_hash.clone());
            let offers = env
                .storage()
                .persistent()
                .get::<StorageKey, Vec<OfferType>>(&glyph_offer_key)
                .ok_or(Error::NotFound)?;

            env.storage()
                .persistent()
                .bump(&glyph_offer_key, MAX_ENTRY_LIFETIME);

            match offers.binary_search(buy) {
                Ok(offer_index) => {
                    let glyph_owner_key = StorageKey::GlyphOwner(offer_hash.clone());
                    let offer_owner = env
                        .storage()
                        .persistent()
                        .get::<StorageKey, Address>(&glyph_owner_key)
                        .ok_or(Error::NotFound)?;

                    env.storage()
                        .persistent()
                        .bump(&glyph_owner_key, MAX_ENTRY_LIFETIME);

                    // We don't always use glyph_offers & offer_index but they're necessary to lookup here as it's how we look for a specific offer
                    Ok(Offer::Glyph(offer_index, offers, offer_owner, offer_hash))
                }
                _ => Err(Error::NotFound),
            }
        }
        OfferType::Asset(asset_hash, amount) => {
            // Selling an Asset
            match buy {
                OfferType::Glyph(glyph_hash) => {
                    let asset_offer_key =
                        StorageKey::AssetOffer(glyph_hash.clone(), asset_hash.clone(), amount);
                    let offers = env
                        .storage()
                        .persistent()
                        .get::<StorageKey, Vec<Address>>(&asset_offer_key)
                        .ok_or(Error::NotFound)?;

                    env.storage()
                        .persistent()
                        .bump(&asset_offer_key, MAX_ENTRY_LIFETIME);

                    Ok(Offer::Asset(offers, glyph_hash, asset_hash, amount))
                }
                _ => Err(Error::NotPermitted), // You cannot sell an Asset for an Asset
            }
        }
    }
}
