use soroban_auth::{Identifier, Signature};
use soroban_sdk::{panic_with_error, Address, BytesN, Env, Vec};

use crate::{
    types::{AssetType, StorageKey, Error, Offer, AssetAmount, OfferOwner, MaybeSignature, Side, GlyphOffer}, 
    token::Client as TokenClient
};

// NOPE:
    // Should we disable folks from offering to buy their own glyph?

pub fn offer(env: &Env, signature: MaybeSignature, buy: AssetType, sell: AssetType) -> Result<(), Error> {
    let buy_hash: BytesN<32>;
    let sell_hash: BytesN<32>;
    let mut amount = 0i128;
    let mut side = Side::default();

    // TODO:
        // If transferring close all current glyph owner's open sell offers
        // If actually performing a transfer deal with royalty payments

    match buy {
        AssetType::Asset(hash_amount) => { // Selling a glyph
            buy_hash = hash_amount.0;
            amount = hash_amount.1;
        },
        AssetType::Glyph(hash) => {
            side = Side::Buy; // we're buying a glyph
            buy_hash = hash;
        }
    }

    match sell {
        AssetType::Asset(hash_amount) => {
            // Don't allow offers where no Glyph is involved
            if side == Side::None {
                panic_with_error!(env, Error::NotPermitted);
            }

            sell_hash = hash_amount.0;
            amount = hash_amount.1;
        },
        AssetType::Glyph(hash) => {
            let owner: Address = env
                .storage()
                .get(StorageKey::GlyphOwner(hash.clone()))
                .ok_or(Error::NotFound)?
                .unwrap();

            // If we're selling a glyph ensure we're the owner
            if owner != env.invoker() {
                panic_with_error!(env, Error::NotAuthorized);
            }

            side = Side::Sell; // We're selling a glyph
            sell_hash = hash;
        },
    }

    // Look up counter offer by inverting the current offer args
    let existing_offer_owner = get_offer(
        env,
        sell_hash.clone(),
        buy_hash.clone(),
        amount.clone(),
        if side == Side::Buy {Side::Sell} else {Side::Buy}
    );

    // TODO 
        // I think there's some simplification in the logic if there's an existing offer to not need to pick a side as the side will be determined if the existing offer is a Glyph or an Asset
        // Can we derive side based on if the invoker is the glyph owner? (you can't sell something you don't have and you can't buy something you already own)

    match side {
        Side::Buy => { // Buying a glyph
            match existing_offer_owner {
                Ok(existing_offer_owner) => {
                    // TODO: someone is selling this Glyph for this AssetAmount
                    // panic!("we have a desirous seller!");

                    match signature {
                        MaybeSignature::Signature(signature) => {
                            let (
                                contract_identifier, 
                                signature_identifier, 
                                token, 
                                sender_nonce
                            ) = get_token_bits(env, &signature);
                            
                            // incr_allow Asset from Glyph taker to contract
                            token.incr_allow(
                                &signature,
                                &sender_nonce,
                                &contract_identifier,
                                &amount
                            );

                            match existing_offer_owner {
                                OfferOwner::Address(existing_offer_owner_address) => {
                                    // xfer_from Asset from Glyph taker to Glyph giver
                                    token.xfer_from(
                                        &Signature::Invoker,
                                        &0,
                                        &signature_identifier,
                                        &Identifier::from(existing_offer_owner_address),
                                        &amount
                                    );

                                    // transfer ownership of Glyph from glyph giver to Glyph taker
                                    // TODO: <--

                                    // rm existing_offer
                                    // TODO: 
                                        // pretty repetitive args from getting the existing from above. Is there a faster way to do this?
                                        // We also need to clear any and all open glyph sell offers for the current owner
                                        
                                    rm_offer(
                                        env,
                                        sell_hash.clone(),
                                        buy_hash.clone(),
                                        amount.clone(),
                                        if side == Side::Buy {Side::Sell} else {Side::Buy}
                                    );
                                },
                                _ => panic_with_error!(&env, Error::NotPermitted),
                            }
                        },
                        MaybeSignature::None => panic_with_error!(&env, Error::NotPermitted),
                    }
                },
                Err(_) => {
                    // NOPE:
                        // This logic only allows one offer offer per glyph per counter per amount
                            // Fine if the sell_hash is a Glyph (offer)
                            // Not fine if sell_hash is an Asset (multiple people should be able to offer Amount(10) Asset(hash) per Glyph(hash))

                    // I'm buying a Glyph for Amount of Asset 
                    // Is someone selling Glyph for Amount of Asset?

                    // I'm selling a Glyph for Amount of Asset
                    // Is someone buying Glyph for Amount of Asset?

                    // You don't care who the owner is you just want to know if anyone is a counter match

                    // I think we need to rely on Events to inform a sort of assurance when passing in a counter vs dynamically finding one?
                    // This would mean however you could have offers that _could_ match but we don't automatically do any dynamic matching

                    // Alternatively there can only be one offer per amount

                    match signature {
                        MaybeSignature::Signature(signature) => {
                            let (
                                contract_identifier, 
                                signature_identifier, 
                                token, 
                                sender_nonce
                            ) = get_token_bits(env, &signature);

                            token.incr_allow(
                                &signature,
                                &sender_nonce, 
                                &contract_identifier,
                                &amount
                            );
                        
                            token.xfer_from(
                                &Signature::Invoker,
                                &0,
                                &signature_identifier,
                                &contract_identifier,
                                &amount
                            );

                            let offer = Offer{
                                buy: buy_hash,
                                sell: sell_hash,
                                amount
                            };

                            let has_offer = env
                                .storage()
                                .has(&offer);

                            if has_offer {
                                panic_with_error!(env, Error::NotEmpty)
                            } else {
                                env
                                    .storage()
                                    .set(&offer, env.invoker());
                            }
                        },
                        MaybeSignature::None => panic_with_error!(&env, Error::NotPermitted),
                    }
                },
            }
        },
        Side::Sell => { // Selling a glyph
            match existing_offer_owner {
                Ok(_) => {
                    // TODO: someone is buying this Glyph for this AssetAmount
                    panic!("we have a desirous buyer!");
                },
                Err(_) => {
                    let mut offers: Vec<AssetAmount> = env
                        .storage()
                        .get(&sell_hash)
                        .unwrap_or(Ok(Vec::new(env)))
                        .unwrap();

                    let offer = AssetAmount(buy_hash, amount);
                    
                    match offers.binary_search(&offer) {
                        Result::Err(i) => offers.insert(i, offer),
                        _ => panic_with_error!(&env, Error::NotEmpty), // dupe
                    }

                    env
                        .storage()
                        .set(&sell_hash, offers);
                }
            }
        },
        _ => panic_with_error!(env, Error::NotPermitted),
    }

    Ok(())
}

pub fn get_offer(
    env: &Env, 
    buy_hash: BytesN<32>, 
    sell_hash: BytesN<32>, 
    amount: i128, 
    side: Side
) -> Result<OfferOwner, Error> {
    match side {
        Side::Sell => {
            let offers: Vec<AssetAmount> = env
                .storage()
                .get(&sell_hash)
                .ok_or(Error::NotFound)?
                .unwrap();

            match offers.binary_search(AssetAmount(buy_hash, amount)) {
                Err(_) => panic_with_error!(env, Error::NotFound),
                Ok(offer_index) => {
                    let glyph_owner = env
                        .storage()
                        .get(StorageKey::GlyphOwner(sell_hash.clone()))
                        .ok_or(Error::NotFound)?
                        .unwrap();

                    Ok(OfferOwner::Glyph(GlyphOffer(
                        glyph_owner,
                        offer_index,
                        offers,
                    )))
                },   
            }
        },
        Side::Buy => {
            let offer_owner = env
                .storage()
                .get(Offer{
                    buy: buy_hash,
                    sell: sell_hash,
                    amount
                })
                .ok_or(Error::NotFound)?
                .unwrap();

            Ok(OfferOwner::Address(offer_owner))
        },
        _ => panic_with_error!(env, Error::NotPermitted),
    }
}

pub fn rm_offer(env: &Env, buy_hash: BytesN<32>, sell_hash: BytesN<32>, amount: i128, side: Side) {
    let offer_owner = get_offer(env, buy_hash.clone(), sell_hash.clone(), amount.clone(), side.clone());

    match offer_owner.unwrap() {
        OfferOwner::Address(address) => { // Buying glyph
            if address != env.invoker() {
                panic_with_error!(env, Error::NotAuthorized);
            }

            let invoker_id = Identifier::from(address);
            let token_id: BytesN<32> = env.storage().get(StorageKey::InitToken).unwrap().unwrap();
            let token = TokenClient::new(env, token_id);

            token.xfer(
                &Signature::Invoker,
                &0,
                &invoker_id,
                &amount
            );

            env
                .storage()
                .remove(Offer{
                    buy: buy_hash,
                    sell: sell_hash,
                    amount
                });
        },
        OfferOwner::Glyph(GlyphOffer(address, offer_index, mut offers)) => { // Selling glyph
            if address != env.invoker() {
                panic_with_error!(env, Error::NotAuthorized);
            }

            offers.remove(offer_index);

            env
                .storage()
                .set(&sell_hash, offers);
        },
    }
}

fn get_token_bits(env: &Env, signature: &Signature) -> (
    Identifier, 
    Identifier, 
    TokenClient, 
    i128
) {
    let contract_identifier = Identifier::Contract(env.get_current_contract());
    let signature_identifier = signature.identifier(env);
    let token_id: BytesN<32> = env.storage().get(StorageKey::InitToken).unwrap().unwrap();
    let token = TokenClient::new(env, token_id);
    let sender_nonce = token.nonce(&signature_identifier);

    (
        contract_identifier, 
        signature_identifier, 
        token, 
        sender_nonce
    )
}