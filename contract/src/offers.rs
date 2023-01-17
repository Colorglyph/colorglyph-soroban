use soroban_auth::{Identifier, Signature};
use soroban_sdk::{panic_with_error, Env, Vec};

use crate::{
    token::Client as TokenClient,
    types::{
        AssetAmount, AssetOffer, AssetSellOffer, Error, GlyphSellOffer, MaybeSignature, Offer,
        OfferType, StorageKey,
    },
    utils::get_token_bits,
};

// QA:
// Should we disable folks from offering to buy their own glyph?

// NOTES:
// This logic only allows one offer per glyph per counter per amount
// Fine if the sell_hash is a Glyph (offer)
// Not fine if sell_hash is an Asset (multiple people should be able to offer Amount(10) Asset(hash) per Glyph(hash))
// You don't care who the owner is you just want to know if anyone is a counter match
// I think we need to rely on Events to inform a sort of assurance when passing in a counter vs dynamically finding one?
// This would mean however you could have offers that _could_ match but we don't automatically do any dynamic matching
// Alternatively there can only be one offer per amount (what we're doing currently)

pub fn offer(
    env: &Env,
    signature: &MaybeSignature,
    buy: &OfferType,
    sell: &OfferType,
) -> Result<(), Error> {
    // TODO:
    // If actually performing a transfer deal with royalty payments

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
                Offer::Glyph(GlyphSellOffer(
                    buy_offer_owner,
                    buy_glyph_hash,
                    _, // buy_glyph_offers,
                    _, // buy_offer_index,
                )) => {
                    match sell {
                        // sell glyph now for glyph
                        OfferType::Glyph(sell_glyph_hash) => {
                            // transfer ownership from seller to buyer
                            env.storage().set(
                                StorageKey::GlyphOwner(sell_glyph_hash.clone()),
                                buy_offer_owner,
                            );

                            // remove all glyph seller offers
                            env.storage()
                                .remove(StorageKey::GlyphOffer(sell_glyph_hash.clone()));

                            // transfer ownership from buyer to seller
                            env.storage().set(
                                StorageKey::GlyphOwner(buy_glyph_hash.clone()),
                                env.invoker(),
                            );

                            // remove all glyph buyer offers
                            env.storage().remove(StorageKey::GlyphOffer(buy_glyph_hash));
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
                                        &Identifier::from(buy_offer_owner),
                                        &amount,
                                    );

                                    // transfer ownership of Glyph from glyph giver to Glyph taker
                                    env.storage().set(
                                        StorageKey::GlyphOwner(buy_glyph_hash.clone()),
                                        env.invoker(),
                                    );

                                    // remove Asset counter offer
                                    env.storage().remove(StorageKey::AssetOffer(AssetOffer(
                                        buy_glyph_hash.clone(),
                                        sell_asset_hash.clone(),
                                        *amount,
                                    )));

                                    // remove all other sell offers for this glyph
                                    env.storage().remove(StorageKey::GlyphOffer(buy_glyph_hash));
                                }
                                _ => panic_with_error!(env, Error::NotPermitted), // You cannot sell an Asset without a signature
                            }
                        }
                    }
                }

                // Found someone buying your sale with an Asset (meaning sell is a Glyph)
                Offer::Asset(AssetSellOffer(offer_owner, glyph_hash, asset_hash, amount)) => {
                    let token = TokenClient::new(env, &asset_hash);

                    // xfer Asset from Glyph taker to Glyph giver
                    token.xfer(
                        &Signature::Invoker,
                        &0,
                        &Identifier::from(env.invoker()),
                        &amount,
                    );

                    // transfer ownership of Glyph from glyph giver to Glyph taker
                    env.storage()
                        .set(StorageKey::GlyphOwner(glyph_hash.clone()), offer_owner);

                    // remove Asset counter offer
                    env.storage().remove(StorageKey::AssetOffer(AssetOffer(
                        glyph_hash.clone(),
                        asset_hash,
                        amount,
                    )));

                    // remove all other sell offers for this glyph
                    env.storage().remove(StorageKey::GlyphOffer(glyph_hash));
                }
            }
        }
        Err(_) => {
            match sell {
                OfferType::Glyph(glyph_hash) => {
                    // Selling a Glyph
                    let mut offers: Vec<OfferType> = env
                        .storage()
                        .get(StorageKey::GlyphOffer(glyph_hash.clone()))
                        .unwrap_or(Ok(Vec::new(env)))
                        .unwrap();

                    match offers.binary_search(buy) {
                        Result::Err(i) => offers.insert(i, buy.clone()),
                        _ => panic_with_error!(&env, Error::NotEmpty), // dupe
                    }

                    env.storage()
                        .set(StorageKey::GlyphOffer(glyph_hash.clone()), offers);
                }
                OfferType::Asset(AssetAmount(asset_hash, amount)) => {
                    // Selling an Asset
                    match buy {
                        OfferType::Glyph(glyph_hash) => {
                            // Buying a Glyph
                            let offer = AssetOffer(glyph_hash.clone(), asset_hash.clone(), *amount);

                            // TODO: Add support for storing a Vec of AssetOffer owners vs just one Address

                            if env.storage().has(StorageKey::AssetOffer(offer.clone())) {
                                panic_with_error!(env, Error::NotEmpty)
                            }

                            match signature {
                                MaybeSignature::Signature(signature) => {
                                    // Selling Asset (Buying Glyph)
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

                                    env.storage()
                                        .set(StorageKey::AssetOffer(offer), env.invoker());
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

// TODO: this might should be a private function

pub fn get_offer(env: &Env, buy: &OfferType, sell: &OfferType) -> Result<Offer, Error> {
    match sell {
        OfferType::Glyph(glyph_hash) => {
            // Selling a Glyph
            let glyph_offers: Vec<OfferType> = env
                .storage()
                .get(StorageKey::GlyphOffer(glyph_hash.clone()))
                .ok_or(Error::NotFound)?
                .unwrap();

            match glyph_offers.binary_search(buy) {
                Ok(offer_index) => {
                    let glyph_owner = env
                        .storage()
                        .get(StorageKey::GlyphOwner(glyph_hash.clone()))
                        .ok_or(Error::NotFound)?
                        .unwrap();

                    Ok(Offer::Glyph(GlyphSellOffer(
                        glyph_owner,
                        glyph_hash.clone(),
                        glyph_offers,
                        offer_index,
                    )))
                }
                _ => panic_with_error!(env, Error::NotFound),
            }
        }
        OfferType::Asset(AssetAmount(asset_hash, amount)) => {
            // Selling an Asset
            match buy {
                OfferType::Glyph(glyph_hash) => {
                    let asset_sell_offer_owner = env
                        .storage()
                        .get(StorageKey::AssetOffer(AssetOffer(
                            glyph_hash.clone(),
                            asset_hash.clone(),
                            *amount,
                        )))
                        .ok_or(Error::NotFound)?
                        .unwrap();

                    Ok(Offer::Asset(AssetSellOffer(
                        asset_sell_offer_owner,
                        glyph_hash.clone(),
                        asset_hash.clone(),
                        *amount,
                    )))
                }
                _ => panic_with_error!(env, Error::NotPermitted), // You cannot sell an Asset for an Asset
            }
        }
    }
}

pub fn rm_offer(env: &Env, buy: &OfferType, sell: &OfferType) {
    match get_offer(env, buy, sell) {
        Ok(offer) => {
            match offer {
                Offer::Glyph(GlyphSellOffer(
                    // Selling a Glyph
                    offer_owner,
                    glyph_hash,
                    mut glyph_offers,
                    offer_index,
                )) => {
                    // You cannot delete an offer for a glyph you are not the owner of
                    if offer_owner != env.invoker() {
                        panic_with_error!(env, Error::NotAuthorized);
                    }

                    glyph_offers.remove(offer_index);

                    env.storage()
                        .set(StorageKey::GlyphOffer(glyph_hash.clone()), glyph_offers);
                }
                Offer::Asset(AssetSellOffer(
                    // Selling an Asset
                    offer_owner,
                    glyph_hash,
                    asset_hash,
                    amount,
                )) => {
                    if offer_owner != env.invoker() {
                        panic_with_error!(env, Error::NotAuthorized);
                    }

                    let token = TokenClient::new(env, &asset_hash);

                    token.xfer(
                        &Signature::Invoker,
                        &0,
                        &Identifier::from(offer_owner),
                        &amount,
                    );

                    env.storage().remove(StorageKey::AssetOffer(AssetOffer(
                        glyph_hash, asset_hash, amount,
                    )));
                }
            }
        }
        _ => panic_with_error!(env, Error::NotFound),
    }
}
