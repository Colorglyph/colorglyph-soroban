use soroban_auth::{Identifier, Signature};
use soroban_sdk::{panic_with_error, Address, BytesN, Env, Vec};

use crate::{types::{AssetType, DataKey, Error, Trade, AssetAmount, TradeOwner, GlyphOwner}, token::Client as TokenClient};

// NOPE:
    // Should we disable folks from offering to buy their own glyph?

pub fn trade(env: &Env, signature: Signature, buy: AssetType, sell: AssetType) {
    let buy_hash: BytesN<32>;
    let sell_hash: BytesN<32>;
    let mut amount = 0i128;
    let mut side = DataKey::None;

    // TODO:
        // Check for a buy/sell now opportunity
        // If actually performing a transfer deal with royalty payments
        // If transferring close all current glyph owner's open sell offers

    // Q:
        // Is someone selling this glyph?
            // Is someone selling this glyph for this counter asset?
                // If so take that trade

    // Glyph sells by owner (the thing we need to track to make it easy to clear)

    match buy {
        AssetType::Asset(hash_amount) => { // Selling a glyph
            buy_hash = hash_amount.0;
            amount = hash_amount.1;
        },
        AssetType::Glyph(hash) => {
            side = DataKey::SideBuy; // we're buying a glyph
            buy_hash = hash;
        }
    }

    match sell {
        AssetType::Asset(hash_amount) => {
            // Don't allow trades where no Glyph is involved
            if side == DataKey::None {
                panic_with_error!(env, Error::NotPermitted);
            }

            sell_hash = hash_amount.0;
            amount = hash_amount.1;
        },
        AssetType::Glyph(hash) => {
            let owner: Address = env
                .storage()
                .get(DataKey::GlyphOwner(hash.clone()))
                .unwrap_or_else(|| panic_with_error!(env, Error::NotFound))
                .unwrap();

            // If we're selling a glyph ensure we're the owner
            if owner != env.invoker() {
                panic_with_error!(env, Error::NotAuthorized);
            }

            side = DataKey::SideSell; // We're selling a glyph
            sell_hash = hash;
        },
    }

    match side {
        DataKey::SideBuy => {

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


            let contract_id = Identifier::Contract(env.get_current_contract());
            let sig_id = signature.identifier(env);
            let token_id: BytesN<32> = env.storage().get(DataKey::InitToken).unwrap().unwrap();
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
        DataKey::SideSell => {
            let mut trades: Vec<AssetAmount> = env
                .storage()
                .get(&sell_hash)
                .unwrap_or(Ok(Vec::new(env)))
                .unwrap();

            // NOPE: 
                // this should be inserted in sorted order via binary search
                // panic if not Err (item already exists)
                    // Do we care if the trade already exists aside from wasted gas for the user?

            trades.push_back(AssetAmount(buy_hash, amount));

            env
                .storage()
                .set(&sell_hash, trades);
        },
        _ => panic_with_error!(env, Error::NotPermitted),
    }
}

pub fn get_trade(env: &Env, buy_hash: BytesN<32>, sell_hash: BytesN<32>, amount: i128, side: DataKey) -> TradeOwner {
    match side {
        DataKey::SideSell => {
            let trades: Vec<AssetAmount> = env
                .storage()
                .get(&sell_hash)
                .unwrap_or_else(|| panic_with_error!(env, Error::NotFound))
                .unwrap();

            let trade_index = trades.first_index_of(AssetAmount(buy_hash, amount));

            if trade_index.is_none() { 
                panic_with_error!(env, Error::NotFound);
            } else {
                TradeOwner::GlyphOwner(GlyphOwner(
                    env
                        .storage()
                        .get(DataKey::GlyphOwner(sell_hash.clone()))
                        .unwrap_or_else(|| panic_with_error!(env, Error::NotFound))
                        .unwrap(),
                    trades,
                    trade_index.unwrap()
                ))
            }
        },
        DataKey::SideBuy => {
            TradeOwner::Address(
                env
                    .storage()
                    .get(Trade{
                        buy: buy_hash,
                        sell: sell_hash,
                        amount
                    })
                    .unwrap_or_else(|| panic_with_error!(env, Error::NotFound))
                    .unwrap()
            )
        },
        _ => panic_with_error!(env, Error::NotPermitted),
    }
}

pub fn rm_trade(env: &Env, buy_hash: BytesN<32>, sell_hash: BytesN<32>, amount: i128, side: DataKey) {
    let trade_owner = get_trade(env, buy_hash.clone(), sell_hash.clone(), amount.clone(), side.clone());

    match trade_owner {
        TradeOwner::Address(address) => { // Buying glyph
            if address != env.invoker() {
                panic_with_error!(env, Error::NotAuthorized);
            }

            let invoker_id = Identifier::from(address);
            let token_id: BytesN<32> = env.storage().get(DataKey::InitToken).unwrap().unwrap();
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