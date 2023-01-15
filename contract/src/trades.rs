use soroban_auth::{Identifier, Signature};
use soroban_sdk::{panic_with_error, Address, BytesN, Env, Vec};

use crate::{
    types::{AssetType, StorageKey, Error, Trade, AssetAmount, TradeOwner, GlyphOwner, MaybeSignature, Side}, 
    token::Client as TokenClient
};

// NOPE:
    // Should we disable folks from offering to buy their own glyph?

pub fn trade(env: &Env, signature: MaybeSignature, buy: AssetType, sell: AssetType) -> Result<(), Error> {
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
            // Don't allow trades where no Glyph is involved
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

    // Look up counter trade by inverting the current trade args
    let existing_trade_owner = get_trade(
        env,
        sell_hash.clone(),
        buy_hash.clone(),
        amount.clone(),
        if side == Side::Buy {Side::Sell} else {Side::Buy}
    );

    match side {
        Side::Buy => { // Buying a glyph
            match existing_trade_owner {
                Ok(_) => {
                    // TODO: someone is selling this Glyph for this AssetAmount
                    panic!("we have a desirous seller!");
                },
                Err(_) => {
                    // NOPE:
                        // This logic only allows one trade offer per glyph per counter per amount
                            // Fine if the sell_hash is a Glyph (trade)
                            // Not fine if sell_hash is an Asset (multiple people should be able to offer Amount(10) Asset(hash) per Glyph(hash))

                    // I'm buying a Glyph for Amount of Asset 
                    // Is someone selling Glyph for Amount of Asset?

                    // I'm selling a Glyph for Amount of Asset
                    // Is someone buying Glyph for Amount of Asset?

                    // You don't care who the owner is you just want to know if anyone is a counter match

                    // I think we need to rely on Events to inform a sort of assurance when passing in a counter vs dynamically finding one?
                    // This would mean however you could have trades that _could_ match but we don't automatically do any dynamic matching

                    // Alternatively there can only be one trade per amount

                    match signature {
                        MaybeSignature::Signature(signature) => {
                            let contract_id = Identifier::Contract(env.get_current_contract());
                            let sig_id = signature.identifier(env);
                            let token_id: BytesN<32> = env.storage().get(StorageKey::InitToken).unwrap().unwrap();
                            let token = TokenClient::new(env, token_id);
                            let sender_nonce = token.nonce(&sig_id);

                            token.incr_allow(
                                &signature,
                                &sender_nonce, 
                                &contract_id,
                                &amount
                            );
                        
                            token.xfer_from(
                                &Signature::Invoker,
                                &0,
                                &sig_id,
                                &contract_id,
                                &amount
                            );

                            let trade = Trade{
                                buy: buy_hash,
                                sell: sell_hash,
                                amount
                            };

                            let has_trade = env
                                .storage()
                                .has(&trade);

                            if has_trade {
                                panic_with_error!(env, Error::NotEmpty)
                            } else {
                                env
                                    .storage()
                                    .set(&trade, env.invoker());
                            }
                        },
                        MaybeSignature::None => panic_with_error!(&env, Error::NotPermitted),
                    }
                },
            }
        },
        Side::Sell => { // Selling a glyph
            match existing_trade_owner {
                Ok(_) => {
                    // TODO: someone is buying this Glyph for this AssetAmount
                    panic!("we have a desirous buyer!");
                },
                Err(_) => {
                    let mut trades: Vec<AssetAmount> = env
                        .storage()
                        .get(&sell_hash)
                        .unwrap_or(Ok(Vec::new(env)))
                        .unwrap();

                    let trade = AssetAmount(buy_hash, amount);
                    
                    match trades.binary_search(&trade) {
                        Result::Err(i) => trades.insert(i, trade),
                        _ => panic_with_error!(&env, Error::NotEmpty), // dupe
                    }

                    env
                        .storage()
                        .set(&sell_hash, trades);
                }
            }
        },
        _ => panic_with_error!(env, Error::NotPermitted),
    }

    Ok(())
}

pub fn get_trade(env: &Env, buy_hash: BytesN<32>, sell_hash: BytesN<32>, amount: i128, side: Side) -> Result<TradeOwner, Error> {
    match side {
        Side::Sell => {
            let trades: Vec<AssetAmount> = env
                .storage()
                .get(&sell_hash)
                .ok_or(Error::NotFound)?
                .unwrap();

            match trades.binary_search(AssetAmount(buy_hash, amount)) {
                Err(_) => panic_with_error!(env, Error::NotFound),
                Ok(trade_index) => {
                    let glyph_owner = env
                        .storage()
                        .get(StorageKey::GlyphOwner(sell_hash.clone()))
                        .ok_or(Error::NotFound)?
                        .unwrap();

                    Ok(TradeOwner::GlyphOwner(GlyphOwner(
                        glyph_owner,
                        trades,
                        trade_index
                    )))
                },   
            }
        },
        Side::Buy => {
            let trade_owner = env
                .storage()
                .get(Trade{
                    buy: buy_hash,
                    sell: sell_hash,
                    amount
                })
                .ok_or(Error::NotFound)?
                .unwrap();

            Ok(TradeOwner::Address(trade_owner))
        },
        _ => panic_with_error!(env, Error::NotPermitted),
    }
}

pub fn rm_trade(env: &Env, buy_hash: BytesN<32>, sell_hash: BytesN<32>, amount: i128, side: Side) {
    let trade_owner = get_trade(env, buy_hash.clone(), sell_hash.clone(), amount.clone(), side.clone());

    match trade_owner.unwrap() {
        TradeOwner::Address(address) => { // Buying glyph
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
                .remove(Trade{
                    buy: buy_hash,
                    sell: sell_hash,
                    amount
                });
        },
        TradeOwner::GlyphOwner(GlyphOwner(address, mut trades, trade_index)) => { // Selling glyph
            if address != env.invoker() {
                panic_with_error!(env, Error::NotAuthorized);
            }

            trades.remove(trade_index);

            env
                .storage()
                .set(&sell_hash, trades);
        },
    }
}