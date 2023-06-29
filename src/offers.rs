use fixed_point_math::FixedPoint;
use soroban_sdk::{panic_with_error, token, Address, Env, Vec};

use crate::{
    glyphs::{glyph_get},
    types::{
        AssetAmount, AssetOffer, AssetOfferArg, Error, GlyphOfferArg, Offer, OfferType, StorageKey,
    }, utils::glyph_verify_ownership
};

// TODO
// Document everything clearly
// Break it up into individual functions to improve legibility
// I'm not convinced it's terribly efficient or that we aren't over doing the match nesting hell
// Place caps on the number of GlyphOffer and AssetOffer Vec lengths
// Create fn for removing all a glyph owners open sell offers
// Ensure we can't offer to sell a glyph, scrape it, then accept a buy offer

const MINTER_ROYALTY_RATE: i128 = 3;
const MINER_ROYALTY_RATE: i128 = 2;

pub fn offer_post(
    env: &Env,
    seller: Address,
    sell: &OfferType,
    buy: &OfferType,
) -> Result<(), Error> {
    seller.require_auth();

    // existing counter offer
    // yes
    // sell glyph
    // match is glyph
    // take glyph, give glyph
    // match is asset
    // take asset, give glyph
    // sell asset
    // give asset, take glyph
    // no
    // sell glyph
    // set glyph offer
    // sell asset
    // take asset into custody
    // set asset offer (save glyph hash and asset amount)

    match offer_get(env, buy, sell) {
        Ok(existing_offer) => {
            match existing_offer {
                // Found someone buying your sale with a Glyph (meaning sell is either a Glyph or Asset)
                Offer::Glyph(GlyphOfferArg(_, _, existing_offer_owner, existing_offer_hash)) => {
                    match sell {
                        // sell glyph now for glyph
                        OfferType::Glyph(offer_hash) => {
                            glyph_verify_ownership(env, seller.clone(), offer_hash.clone());

                            // transfer ownership from seller to buyer
                            env.storage().set(
                                &StorageKey::GlyphOwner(offer_hash.clone()),
                                &existing_offer_owner,
                            );

                            // transfer ownership from buyer to seller
                            env.storage().set(
                                &StorageKey::GlyphOwner(existing_offer_hash.clone()),
                                &seller,
                            );

                            // remove all glyph seller offers
                            env.storage()
                                .remove(&StorageKey::GlyphOffer(offer_hash.clone()));

                            // remove all glyph buyer offers
                            env.storage()
                                .remove(&StorageKey::GlyphOffer(existing_offer_hash));
                        }
                        // sell asset now for glyph
                        OfferType::Asset(AssetAmount(offer_hash, amount)) => {
                            let token = token::Client::new(env, offer_hash);

                            // START royalties
                            // Might want to make a map of payees to reduce or eliminate piecemeal payments
                            let mut leftover_amount = amount.clone();

                            // Get glyph
                            let glyph = glyph_get(env, existing_offer_hash.clone()).unwrap();
                            let glyph_minter: Address = env
                                .storage()
                                .get(&StorageKey::GlyphMinter(existing_offer_hash.clone()))
                                .ok_or(Error::NotFound)?
                                .unwrap();

                            // TODO
                            // if glyph_minter is existing_offer_owner don't make this payment

                            // pay the glyph minter their cut
                            let minter_amount =
                                MINTER_ROYALTY_RATE.fixed_mul_ceil(*amount, 100).unwrap();

                            token.transfer(&seller, &glyph_minter, &minter_amount);

                            leftover_amount -= minter_amount;

                            // Loop over miners
                            for (miner_address, colors_indexes) in glyph.colors.iter_unchecked() {
                                let mut color_count: u32 = 0;

                                // Count colors per miner
                                for (_, indexes) in colors_indexes.iter_unchecked() {
                                    color_count += indexes.len();
                                }

                                let miner_amount = MINER_ROYALTY_RATE
                                    .fixed_mul_ceil(*amount, 100)
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
                            env.storage().set(
                                &StorageKey::GlyphOwner(existing_offer_hash.clone()),
                                &seller,
                            );

                            // remove all other sell offers for this glyph
                            env.storage()
                                .remove(&StorageKey::GlyphOffer(existing_offer_hash));
                        }
                    }
                }
                // Found someone buying your sale with an Asset (meaning sell is a Glyph)
                Offer::Asset(AssetOfferArg(mut offers, offer)) => {
                    glyph_verify_ownership(env, seller.clone(), offer.0.clone());

                    let token = token::Client::new(env, &offer.1);

                    // START royalties
                    let existing_offer_hash = &offer.0;
                    let amount = &offer.2;

                    // Might want to make a map of payees to reduce or eliminate piecemeal payments
                    let mut leftover_amount = amount.clone();

                    // Get glyph
                    let glyph = glyph_get(env, existing_offer_hash.clone()).unwrap();
                    let glyph_minter: Address = env
                        .storage()
                        .get(&StorageKey::GlyphMinter(existing_offer_hash.clone()))
                        .ok_or(Error::NotFound)?
                        .unwrap();

                    // TODO 
                    // if glyph_minter is existing_offer_owner don't make this payment

                    // pay the glyph minter their cut
                    let minter_amount = MINTER_ROYALTY_RATE.fixed_mul_ceil(*amount, 100).unwrap();

                    token.transfer(
                        &env.current_contract_address(),
                        &glyph_minter,
                        &minter_amount,
                    );

                    leftover_amount -= minter_amount;

                    // Loop over miners
                    for (miner_address, colors_indexes) in glyph.colors.iter_unchecked() {
                        let mut color_count: u32 = 0;

                        // Count colors per miner
                        for (_, indexes) in colors_indexes.iter_unchecked() {
                            color_count += indexes.len();
                        }

                        let miner_amount = MINER_ROYALTY_RATE
                            .fixed_mul_ceil(*amount, 100)
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
                    let offer_owner = offers.pop_front().unwrap().unwrap();

                    if offers.is_empty() {
                        env.storage().remove(&StorageKey::AssetOffer(offer.clone()));
                    } else {
                        env.storage()
                            .set(&StorageKey::AssetOffer(offer.clone()), &offers);
                    }

                    // transfer ownership of Glyph from glyph giver to Glyph taker
                    env.storage()
                        .set(&StorageKey::GlyphOwner(offer.0.clone()), &offer_owner);

                    // remove all other sell offers for this glyph
                    env.storage().remove(&StorageKey::GlyphOffer(offer.0));
                }
            }
        }
        Err(_) => {
            match sell {
                OfferType::Glyph(offer_hash) => {
                    glyph_verify_ownership(env, seller.clone(), offer_hash.clone());

                    // Selling a Glyph
                    let mut offers: Vec<OfferType> = env
                        .storage()
                        .get(&StorageKey::GlyphOffer(offer_hash.clone()))
                        .unwrap_or(Ok(Vec::new(env)))
                        .unwrap();

                    match offers.binary_search(buy) {
                        Result::Err(i) => offers.insert(i, buy.clone()), // buy can be an Asset or a Glyph
                        _ => panic_with_error!(&env, Error::NotEmpty),   // dupe
                    }

                    env.storage()
                        .set(&StorageKey::GlyphOffer(offer_hash.clone()), &offers);
                }
                OfferType::Asset(AssetAmount(asset_hash, amount)) => {
                    // Buying a Glyph
                    match buy {
                        OfferType::Glyph(glyph_hash) => {
                            let token_id = env
                                .storage()
                                .get::<StorageKey, Address>(&StorageKey::InitToken)
                                .unwrap()
                                .unwrap();
                            let token = token::Client::new(env, &token_id);

                            token.transfer(&seller, &env.current_contract_address(), &amount);

                            let offer = AssetOffer(glyph_hash.clone(), asset_hash.clone(), *amount);

                            let mut offers: Vec<Address> = env
                                .storage()
                                .get(&StorageKey::AssetOffer(offer.clone()))
                                .unwrap_or(Ok(Vec::new(env)))
                                .unwrap();

                            offers.push_back(seller);

                            env.storage().set(&StorageKey::AssetOffer(offer), &offers);
                        }
                        _ => panic_with_error!(env, Error::NotPermitted), // You cannot sell an Asset for an Asset
                    }
                }
            }
        }
    }

    Ok(())
}

pub fn offer_get(env: &Env, sell: &OfferType, buy: &OfferType) -> Result<Offer, Error> {
    match sell {
        OfferType::Glyph(offer_hash) => {
            // Selling a Glyph
            let offers: Vec<OfferType> = env
                .storage()
                .get(&StorageKey::GlyphOffer(offer_hash.clone()))
                .ok_or(Error::NotFound)?
                .unwrap();

            match offers.binary_search(buy) {
                Ok(offer_index) => {
                    let offer_owner = env
                        .storage()
                        .get(&StorageKey::GlyphOwner(offer_hash.clone()))
                        .ok_or(Error::NotFound)?
                        .unwrap();

                    // We don't always use glyph_offers & offer_index but they're necessary to lookup here as it's how we look for a specific
                    Ok(Offer::Glyph(GlyphOfferArg(
                        offer_index,
                        offers,
                        offer_owner,
                        offer_hash.clone(),
                    )))
                }
                _ => panic_with_error!(env, Error::NotFound),
            }
        }
        OfferType::Asset(AssetAmount(asset_hash, amount)) => {
            // Selling an Asset
            match buy {
                OfferType::Glyph(glyph_hash) => {
                    let offer = AssetOffer(glyph_hash.clone(), asset_hash.clone(), *amount);
                    let offers: Vec<Address> = env
                        .storage()
                        .get(&StorageKey::AssetOffer(offer.clone()))
                        .ok_or(Error::NotFound)?
                        .unwrap();

                    Ok(Offer::Asset(AssetOfferArg(offers, offer)))
                }
                _ => panic_with_error!(env, Error::NotPermitted), // You cannot sell an Asset for an Asset
            }
        }
    }
}

pub fn offer_delete(env: &Env, seller: Address, sell: &OfferType, buy: &OfferType) {
    seller.require_auth();

    match offer_get(env, sell, buy) {
        Ok(existing_offer) => {
            match existing_offer {
                // Selling a Glyph
                Offer::Glyph(GlyphOfferArg(offer_index, mut offers, offer_owner, offer_hash)) => {
                    // You cannot delete an offer for a glyph you are not the owner of
                    if offer_owner != seller {
                        panic_with_error!(env, Error::NotAuthorized);
                    }

                    offers.remove(offer_index);

                    env.storage()
                        .set(&StorageKey::GlyphOffer(offer_hash.clone()), &offers);
                }
                // Selling an Asset
                Offer::Asset(AssetOfferArg(mut offers, offer)) => {
                    match offers.first_index_of(seller.clone()) {
                        Some(offer_index) => {
                            let token = token::Client::new(env, &offer.1);

                            token.transfer(&env.current_contract_address(), &seller, &offer.2);

                            offers.remove(offer_index);

                            if offers.is_empty() {
                                env.storage().remove(&StorageKey::AssetOffer(offer));
                            } else {
                                env.storage().set(&StorageKey::AssetOffer(offer), &offers);
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
