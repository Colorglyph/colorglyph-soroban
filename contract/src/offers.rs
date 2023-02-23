use fixed_point_math::FixedPoint;
use soroban_auth::{Identifier, Signature};
use soroban_sdk::{panic_with_error, Address, Env, Vec};

use crate::{
    glyphs::get_glyph,
    token::Client as TokenClient,
    types::{
        AssetAmount, AssetOffer, AssetOfferArg, Error, GlyphOfferArg, MaybeSignature, Offer,
        OfferType, StorageKey,
    },
    utils::{get_token_bits, verify_glyph_ownership},
};

// TODO:
// ✅ Fine tooth comb everything
// Document everything clearly
// Break it up into individual functions to improve legibility
// I'm not convinced it's terribly efficient or that we aren't over doing the match nesting hell
// ✅ Ensure proper ownership of offer creation, removing and matching (almost positive this is dangerously missing atm)
// Place caps on the number of GlyphOffer and AssetOffer Vec lengths
// Create fn for removing all a glyph owners open sell offers

const MAKER_ROYALTY_RATE: i128 = 3;
const MINER_ROYALTY_RATE: i128 = 2;

pub fn offer(
    env: &Env,
    signature: &MaybeSignature,
    buy: &OfferType,
    sell: &OfferType,
) -> Result<(), Error> {
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

    match get_offer(env, sell, buy) {
        Ok(existing_offer) => {
            match existing_offer {
                // Found someone buying your sale with a Glyph (meaning sell is either a Glyph or Asset)
                Offer::Glyph(GlyphOfferArg(_, _, existing_offer_owner, existing_offer_hash)) => {
                    match sell {
                        // sell glyph now for glyph
                        OfferType::Glyph(offer_hash) => {
                            verify_glyph_ownership(env, offer_hash.clone());

                            // transfer ownership from seller to buyer
                            env.storage().set(
                                StorageKey::GlyphOwner(offer_hash.clone()),
                                existing_offer_owner,
                            );

                            // transfer ownership from buyer to seller
                            env.storage().set(
                                StorageKey::GlyphOwner(existing_offer_hash.clone()),
                                env.invoker(),
                            );

                            // remove all glyph seller offers
                            env.storage()
                                .remove(StorageKey::GlyphOffer(offer_hash.clone()));

                            // remove all glyph buyer offers
                            env.storage()
                                .remove(StorageKey::GlyphOffer(existing_offer_hash));
                        }
                        // sell asset now for glyph
                        OfferType::Asset(AssetAmount(offer_hash, amount)) => {
                            match signature {
                                MaybeSignature::Signature(signature) => {
                                    let (
                                        contract_identifier,
                                        signature_identifier,
                                        token,
                                        sender_nonce,
                                    ) = get_token_bits(env, &offer_hash, signature);

                                    // incr_allow Asset from Glyph taker to contract
                                    token.incr_allow(
                                        signature,
                                        &sender_nonce,
                                        &contract_identifier,
                                        &amount,
                                    );

                                    // START royalties
                                    // Might want to make a map of payees to reduce or eliminate piecemeal payments
                                    let mut leftover_amount = amount.clone();

                                    // Get glyph
                                    let glyph =
                                        get_glyph(env, existing_offer_hash.clone()).unwrap();
                                    let glyph_maker: Address = env
                                        .storage()
                                        .get(StorageKey::GlyphMaker(existing_offer_hash.clone()))
                                        .ok_or(Error::NotFound)?
                                        .unwrap();

                                    // pay the glyph maker their cut
                                    // TODO: if glyph_maker is existing_offer_owner don't make this payment
                                    let maker_amount =
                                        MAKER_ROYALTY_RATE.fixed_mul_ceil(*amount, 100).unwrap();

                                    token.xfer_from(
                                        &Signature::Invoker,
                                        &0,
                                        &signature_identifier,
                                        &Identifier::from(glyph_maker),
                                        &maker_amount,
                                    );

                                    leftover_amount -= maker_amount;

                                    // Loop over miners
                                    for (miner_address, colors_indexes) in
                                        glyph.colors.iter_unchecked()
                                    {
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

                                        // Determine their percentage of whole
                                        // Derive their share of the amount
                                        // Make payment?
                                        // TODO: if miner_address is existing_offer_owner don't make this payment
                                        token.xfer_from(
                                            &Signature::Invoker,
                                            &0,
                                            &signature_identifier,
                                            &Identifier::from(&miner_address),
                                            &miner_amount,
                                        );

                                        leftover_amount -= miner_amount;
                                    }

                                    // xfer_from Asset from Glyph taker to Glyph giver
                                    token.xfer_from(
                                        &Signature::Invoker,
                                        &0,
                                        &signature_identifier,
                                        &Identifier::from(existing_offer_owner),
                                        &leftover_amount,
                                    );
                                    // END royalties

                                    // transfer ownership of Glyph from glyph giver to Glyph taker
                                    env.storage().set(
                                        StorageKey::GlyphOwner(existing_offer_hash.clone()),
                                        env.invoker(),
                                    );

                                    // remove all other sell offers for this glyph
                                    env.storage()
                                        .remove(StorageKey::GlyphOffer(existing_offer_hash));
                                }
                                _ => panic_with_error!(env, Error::NotPermitted), // You cannot sell an Asset without a signature
                            }
                        }
                    }
                }
                // Found someone buying your sale with an Asset (meaning sell is a Glyph)
                Offer::Asset(AssetOfferArg(mut offers, offer)) => {
                    verify_glyph_ownership(env, offer.0.clone());

                    let token = TokenClient::new(env, &offer.1);

                    // START royalties
                    let existing_offer_hash = &offer.0;
                    let amount = &offer.2;

                    // Might want to make a map of payees to reduce or eliminate piecemeal payments
                    let mut leftover_amount = amount.clone();

                    // Get glyph
                    let glyph = get_glyph(env, existing_offer_hash.clone()).unwrap();
                    let glyph_maker: Address = env
                        .storage()
                        .get(StorageKey::GlyphMaker(existing_offer_hash.clone()))
                        .ok_or(Error::NotFound)?
                        .unwrap();

                    // pay the glyph maker their cut
                    // TODO: if glyph_maker is existing_offer_owner don't make this payment
                    let maker_amount = MAKER_ROYALTY_RATE.fixed_mul_ceil(*amount, 100).unwrap();

                    token.xfer(
                        &Signature::Invoker,
                        &0,
                        &Identifier::from(glyph_maker),
                        &maker_amount,
                    );

                    leftover_amount -= maker_amount;

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

                        // Determine their percentage of whole
                        // Derive their share of the amount
                        // Make payment?
                        // TODO: if miner_address is existing_offer_owner don't make this payment
                        token.xfer(
                            &Signature::Invoker,
                            &0,
                            &Identifier::from(&miner_address),
                            &miner_amount,
                        );

                        leftover_amount -= miner_amount;
                    }

                    // xfer_from Asset from Glyph taker to Glyph giver
                    token.xfer(
                        &Signature::Invoker,
                        &0,
                        &Identifier::from(env.invoker()),
                        &leftover_amount,
                    );
                    // END royalties

                    // remove Asset counter offer
                    let offer_owner = offers.pop_front().unwrap().unwrap();

                    if offers.is_empty() {
                        env.storage().remove(StorageKey::AssetOffer(offer.clone()));
                    } else {
                        env.storage()
                            .set(StorageKey::AssetOffer(offer.clone()), offers);
                    }

                    // transfer ownership of Glyph from glyph giver to Glyph taker
                    env.storage()
                        .set(StorageKey::GlyphOwner(offer.0.clone()), offer_owner);

                    // remove all other sell offers for this glyph
                    env.storage().remove(StorageKey::GlyphOffer(offer.0));
                }
            }
        }
        Err(_) => {
            match sell {
                OfferType::Glyph(offer_hash) => {
                    verify_glyph_ownership(env, offer_hash.clone());

                    // Selling a Glyph
                    let mut offers: Vec<OfferType> = env
                        .storage()
                        .get(StorageKey::GlyphOffer(offer_hash.clone()))
                        .unwrap_or(Ok(Vec::new(env)))
                        .unwrap();

                    match offers.binary_search(buy) {
                        Result::Err(i) => offers.insert(i, buy.clone()), // buy can be an Asset or a Glyph
                        _ => panic_with_error!(&env, Error::NotEmpty),   // dupe
                    }

                    env.storage()
                        .set(StorageKey::GlyphOffer(offer_hash.clone()), offers);
                }
                OfferType::Asset(AssetAmount(asset_hash, amount)) => {
                    // Buying a Glyph
                    match buy {
                        OfferType::Glyph(glyph_hash) => {
                            match signature {
                                MaybeSignature::Signature(signature) => {
                                    let (
                                        contract_identifier,
                                        signature_identifier,
                                        token,
                                        sender_nonce,
                                    ) = get_token_bits(env, asset_hash, signature);

                                    token.incr_allow(
                                        signature,
                                        &sender_nonce,
                                        &contract_identifier,
                                        &amount,
                                    );

                                    token.xfer_from(
                                        &Signature::Invoker,
                                        &0,
                                        &signature_identifier,
                                        &contract_identifier,
                                        &amount,
                                    );

                                    let offer =
                                        AssetOffer(glyph_hash.clone(), asset_hash.clone(), *amount);

                                    let mut offers: Vec<Address> = env
                                        .storage()
                                        .get(StorageKey::AssetOffer(offer.clone()))
                                        .unwrap_or(Ok(Vec::new(env)))
                                        .unwrap();

                                    offers.push_back(env.invoker());

                                    env.storage().set(StorageKey::AssetOffer(offer), offers);
                                }
                                _ => panic_with_error!(env, Error::NotPermitted), // You cannot sell an Asset without a signature
                            }
                        }
                        _ => panic_with_error!(env, Error::NotPermitted), // You cannot sell an Asset for an Asset
                    }
                }
            }
        }
    }

    Ok(())
}

pub fn get_offer(env: &Env, buy: &OfferType, sell: &OfferType) -> Result<Offer, Error> {
    match sell {
        OfferType::Glyph(offer_hash) => {
            // Selling a Glyph
            let offers: Vec<OfferType> = env
                .storage()
                .get(StorageKey::GlyphOffer(offer_hash.clone()))
                .ok_or(Error::NotFound)?
                .unwrap();

            match offers.binary_search(buy) {
                Ok(offer_index) => {
                    let offer_owner = env
                        .storage()
                        .get(StorageKey::GlyphOwner(offer_hash.clone()))
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
                        .get(StorageKey::AssetOffer(offer.clone()))
                        .ok_or(Error::NotFound)?
                        .unwrap();

                    Ok(Offer::Asset(AssetOfferArg(offers, offer)))
                }
                _ => panic_with_error!(env, Error::NotPermitted), // You cannot sell an Asset for an Asset
            }
        }
    }
}

pub fn rm_offer(env: &Env, buy: &OfferType, sell: &OfferType) {
    match get_offer(env, buy, sell) {
        Ok(existing_offer) => {
            match existing_offer {
                // Selling a Glyph
                Offer::Glyph(GlyphOfferArg(offer_index, mut offers, offer_owner, offer_hash)) => {
                    // You cannot delete an offer for a glyph you are not the owner of
                    if offer_owner != env.invoker() {
                        panic_with_error!(env, Error::NotAuthorized);
                    }

                    offers.remove(offer_index);

                    env.storage()
                        .set(StorageKey::GlyphOffer(offer_hash.clone()), offers);
                }
                // Selling an Asset
                Offer::Asset(AssetOfferArg(mut offers, offer)) => {
                    let offer_owner = env.invoker();

                    match offers.first_index_of(&offer_owner) {
                        Some(offer_index) => {
                            let token = TokenClient::new(env, &offer.1);

                            token.xfer(
                                &Signature::Invoker,
                                &0,
                                &Identifier::from(offer_owner),
                                &offer.2,
                            );

                            offers.remove(offer_index);

                            if offers.is_empty() {
                                env.storage().remove(StorageKey::AssetOffer(offer));
                            } else {
                                env.storage().set(StorageKey::AssetOffer(offer), offers);
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
