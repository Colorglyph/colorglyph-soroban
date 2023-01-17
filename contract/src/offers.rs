use soroban_auth::{Identifier, Signature};
use soroban_sdk::{panic_with_error, Address, Env, Vec};

use crate::{
    token::Client as TokenClient,
    types::{
        AssetAmount, AssetOfferArg, AssetSell, Error, GlyphOfferArg, MaybeSignature, Offer,
        OfferType, StorageKey,
    },
    utils::get_token_bits,
};

// TODO:
// Fine tooth comb everything
// Document everything clearly
// Break it up into individual functions to improve legibility
// I'm not convinced it's terribly efficient or that we aren't over doing the types and match nesting hell
// Ensure proper ownership of offer creation, removing and matching (almost positive this is dangerously missing atm)
// Place caps on the number of GlyphSell and AssetSell Vec lengths

pub fn offer(
    env: &Env,
    signature: &MaybeSignature,
    buy: &OfferType,
    sell: &OfferType,
) -> Result<(), Error> {
    // TODO:
    // If actually performing a transfer, deal with royalty payments

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
                Offer::Glyph(GlyphOfferArg(_, _, buy_glyph_owner, buy_glyph_hash)) => {
                    match sell {
                        // sell glyph now for glyph
                        OfferType::Glyph(sell_glyph_hash) => {
                            // transfer ownership from seller to buyer
                            env.storage().set(
                                StorageKey::GlyphOwner(sell_glyph_hash.clone()),
                                buy_glyph_owner,
                            );

                            // remove all glyph seller offers
                            env.storage()
                                .remove(StorageKey::GlyphSell(sell_glyph_hash.clone()));

                            // transfer ownership from buyer to seller
                            env.storage().set(
                                StorageKey::GlyphOwner(buy_glyph_hash.clone()),
                                env.invoker(),
                            );

                            // remove all glyph buyer offers
                            env.storage().remove(StorageKey::GlyphSell(buy_glyph_hash));
                        }
                        // sell asset now for glyph
                        OfferType::Asset(AssetAmount(sell_asset_hash, amount)) => {
                            match signature {
                                MaybeSignature::Signature(signature) => {
                                    let (
                                        contract_identifier,
                                        signature_identifier,
                                        token,
                                        sender_nonce,
                                    ) = get_token_bits(env, &sell_asset_hash, signature);

                                    // incr_allow Asset from Glyph taker to contract
                                    token.incr_allow(
                                        signature,
                                        &sender_nonce,
                                        &contract_identifier,
                                        &amount,
                                    );

                                    // xfer_from Asset from Glyph taker to Glyph giver
                                    token.xfer_from(
                                        &Signature::Invoker,
                                        &0,
                                        &signature_identifier,
                                        &Identifier::from(buy_glyph_owner),
                                        &amount,
                                    );

                                    // transfer ownership of Glyph from glyph giver to Glyph taker
                                    env.storage().set(
                                        StorageKey::GlyphOwner(buy_glyph_hash.clone()),
                                        env.invoker(),
                                    );

                                    // remove Asset counter offer
                                    env.storage().remove(StorageKey::AssetSell(AssetSell(
                                        buy_glyph_hash.clone(),
                                        sell_asset_hash.clone(),
                                        *amount,
                                    )));

                                    // remove all other sell offers for this glyph
                                    env.storage().remove(StorageKey::GlyphSell(buy_glyph_hash));
                                }
                                _ => panic_with_error!(env, Error::NotPermitted), // You cannot sell an Asset without a signature
                            }
                        }
                    }
                }

                // Found someone buying your sale with an Asset (meaning sell is a Glyph)
                Offer::Asset(AssetOfferArg(mut asset_offers, asset_sell)) => {
                    let token = TokenClient::new(env, &asset_sell.1);

                    // xfer Asset from Glyph taker to Glyph giver
                    token.xfer(
                        &Signature::Invoker,
                        &0,
                        &Identifier::from(env.invoker()),
                        &asset_sell.2,
                    );

                    // remove Asset counter offer
                    let offer_owner = asset_offers.pop_front().unwrap().unwrap();

                    if asset_offers.is_empty() {
                        env.storage()
                            .remove(StorageKey::AssetSell(asset_sell.clone()));
                    } else {
                        env.storage()
                            .set(StorageKey::AssetSell(asset_sell.clone()), asset_offers);
                    }

                    // transfer ownership of Glyph from glyph giver to Glyph taker
                    env.storage()
                        .set(StorageKey::GlyphOwner(asset_sell.0.clone()), offer_owner);

                    // remove all other sell offers for this glyph
                    env.storage().remove(StorageKey::GlyphSell(asset_sell.0));
                }
            }
        }
        Err(_) => {
            match sell {
                OfferType::Glyph(glyph_hash) => {
                    // Selling a Glyph
                    let mut glyph_offers: Vec<OfferType> = env
                        .storage()
                        .get(StorageKey::GlyphSell(glyph_hash.clone()))
                        .unwrap_or(Ok(Vec::new(env)))
                        .unwrap();

                    match glyph_offers.binary_search(buy) {
                        Result::Err(i) => glyph_offers.insert(i, buy.clone()), // buy can be an Asset or a Glyph
                        _ => panic_with_error!(&env, Error::NotEmpty),         // dupe
                    }

                    env.storage()
                        .set(StorageKey::GlyphSell(glyph_hash.clone()), glyph_offers);
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

                                    let asset_sell =
                                        AssetSell(glyph_hash.clone(), asset_hash.clone(), *amount);

                                    let mut asset_offers: Vec<Address> = env
                                        .storage()
                                        .get(StorageKey::AssetSell(asset_sell.clone()))
                                        .unwrap_or(Ok(Vec::new(env)))
                                        .unwrap();

                                    asset_offers.push_back(env.invoker());

                                    env.storage()
                                        .set(StorageKey::AssetSell(asset_sell), asset_offers);
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
        OfferType::Glyph(glyph_hash) => {
            // Selling a Glyph
            let glyph_offers: Vec<OfferType> = env
                .storage()
                .get(StorageKey::GlyphSell(glyph_hash.clone()))
                .ok_or(Error::NotFound)?
                .unwrap();

            match glyph_offers.binary_search(buy) {
                Ok(offer_index) => {
                    let offer_owner = env
                        .storage()
                        .get(StorageKey::GlyphOwner(glyph_hash.clone()))
                        .ok_or(Error::NotFound)?
                        .unwrap();

                    // We don't always use glyph_offers & offer_index but they're necessary to lookup here as it's how we look for a specific
                    Ok(Offer::Glyph(GlyphOfferArg(
                        offer_index,
                        glyph_offers,
                        offer_owner,
                        glyph_hash.clone(),
                    )))
                }
                _ => panic_with_error!(env, Error::NotFound),
            }
        }
        OfferType::Asset(AssetAmount(asset_hash, amount)) => {
            // Selling an Asset
            match buy {
                OfferType::Glyph(glyph_hash) => {
                    let asset_sell = AssetSell(glyph_hash.clone(), asset_hash.clone(), *amount);

                    let asset_offers: Vec<Address> = env
                        .storage()
                        .get(StorageKey::AssetSell(asset_sell.clone()))
                        .ok_or(Error::NotFound)?
                        .unwrap();

                    // let offer_owner = asset_offers.first().ok_or(Error::NotFound)?.unwrap();

                    // We don't always use glyph_offers & offer_index but they're necessary to lookup here as it's how we look for a specific
                    Ok(Offer::Asset(AssetOfferArg(asset_offers, asset_sell)))
                }
                _ => panic_with_error!(env, Error::NotPermitted), // You cannot sell an Asset for an Asset
            }
        }
    }
}

// TODO: fn for removing all a glyph owners open sell offers

pub fn rm_offer(env: &Env, buy: &OfferType, sell: &OfferType) {
    match get_offer(env, buy, sell) {
        Ok(offer) => {
            match offer {
                // Selling a Glyph
                Offer::Glyph(GlyphOfferArg(
                    offer_index,
                    mut glyph_offers,
                    offer_owner,
                    glyph_hash,
                )) => {
                    // You cannot delete an offer for a glyph you are not the owner of
                    if offer_owner != env.invoker() {
                        panic_with_error!(env, Error::NotAuthorized);
                    }

                    glyph_offers.remove(offer_index);

                    env.storage()
                        .set(StorageKey::GlyphSell(glyph_hash.clone()), glyph_offers);
                }
                // Selling an Asset
                Offer::Asset(AssetOfferArg(mut asset_offers, asset_sell)) => {
                    let offer_owner = env.invoker();

                    match asset_offers.first_index_of(&offer_owner) {
                        Some(offer_index) => {
                            let token = TokenClient::new(env, &asset_sell.1);

                            token.xfer(
                                &Signature::Invoker,
                                &0,
                                &Identifier::from(offer_owner),
                                &asset_sell.2,
                            );

                            asset_offers.remove(offer_index);

                            if asset_offers.is_empty() {
                                env.storage().remove(StorageKey::AssetSell(asset_sell));
                            } else {
                                env.storage()
                                    .set(StorageKey::AssetSell(asset_sell), asset_offers);
                            }
                        }
                        None => panic_with_error!(env, Error::NotAuthorized),
                    }
                }
            }
        }
        _ => panic_with_error!(env, Error::NotFound),
    }
}
