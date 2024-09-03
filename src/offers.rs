use soroban_fixed_point_math::FixedPoint;
use soroban_sdk::{
    token::{self, TokenClient},
    vec, Address, BytesN, Env, Vec,
};

use crate::{
    // events,
    glyphs::glyph_verify_ownership,
    storage::{
        instance::{read_miner_royalty_rate, read_minter_royalty_rate},
        persistent::{
            has_asset_offers_by_asset, read_asset_offers_by_asset, read_glyph_minter,
            read_glyph_or_error, read_glyph_owner, read_offers_by_glyph,
            remove_asset_offers_by_asset, remove_glyph_offer, write_asset_offers_by_asset,
            write_glyph_owner, write_offers_by_glyph,
        },
    },
    types::{Error, Offer, OfferCreate, StorageKey},
};

/* TODO
Document everything clearly
Break it up into individual functions to improve legibility
I'm not convinced it's terribly efficient or that we aren't over doing the match nesting hell
Support progressive offers? This would allow increasing the miner addresses per glyph
    Only really required when processing royalties.
    Also it only involves miner counts not unique colors so the cap is less concerning.
    How many miners are you likely to have really? 15 is the ceiling atm which is pretty high imo
Tweak MINTER_ROYALTY_RATE and MINER_ROYALTY_RATE values
Place caps on the number of GlyphOffer and AssetOffer Vec lengths
    how many sell offers can a Glyph owner open?
    how many identical glyph:asset:amount offers can be open?

!! Ensure we can't sell a partially minted glyph
    Or at the very least ensure we can't sell a glyph with a color length of zero

!! The way the indexer is written atm we should ensure no two offers are identical
*/

pub fn offer_post(env: &Env, sell: Offer, buy: Offer) -> Result<(), Error> {
    // sell glyph
    // lookup if someone is selling what you're buying
    // sell asset
    // lookup if someone is selling what you're buying

    // Lookup if there are any open buy offers for what we're selling
    match &buy {
        // buying a glyph
        Offer::Glyph(buy_glyph_hash) => {
            let mut offers = read_offers_by_glyph(env, buy_glyph_hash);

            match offers.binary_search(match &sell {
                Offer::Glyph(sell_glyph_hash) => Offer::Glyph(sell_glyph_hash.clone()),
                Offer::AssetSell(_, sell_asset_address, amount) => {
                    Offer::Asset(sell_asset_address.clone(), *amount)
                }
                _ => return Err(Error::NotPermitted),
            }) {
                Ok(offer_index) => {
                    let buy_glyph_owner_address =
                        read_glyph_owner(env, buy_glyph_hash).ok_or(Error::NotFound)?;

                    offers.remove(offer_index);

                    write_offers_by_glyph(&env, buy_glyph_hash, offers);

                    match &sell {
                        Offer::Glyph(sell_glyph_hash) => {
                            let sell_glyph_owner_key =
                                StorageKey::GlyphOwner(sell_glyph_hash.clone());
                            let sell_glyph_owner_address =
                                glyph_verify_ownership(env, &sell_glyph_owner_key);

                            transfer_ownership(env, sell_glyph_hash, &buy_glyph_owner_address);

                            transfer_ownership(env, buy_glyph_hash, &sell_glyph_owner_address);

                            // events::offer_match(
                            //     env,
                            //     sell_glyph_hash,
                            //     &sell_glyph_owner_address,
                            //     buy_glyph_hash,
                            //     &buy_glyph_owner_address,
                            // );

                            Ok(())
                        }
                        Offer::AssetSell(sell_asset_owner_address, sell_asset_address, amount) => {
                            sell_asset_owner_address.require_auth();

                            reward_minter_and_miners(
                                env,
                                buy_glyph_owner_address,
                                buy_glyph_hash,
                                amount,
                                sell_asset_address,
                                Some(sell_asset_owner_address.clone()),
                            )?;

                            // Transfer ownership of Glyph from glyph giver to Glyph taker
                            write_glyph_owner(env, buy_glyph_hash, sell_asset_owner_address);

                            // remove all other sell offers for this glyph
                            remove_glyph_offer(env, buy_glyph_hash);

                            // events::offer_match_sell_asset(
                            //     env,
                            //     sell_asset_address,
                            //     sell_asset_owner_address,
                            //     buy_glyph_hash,
                            //     offer_index,
                            // );

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
                    let mut offers = read_asset_offers_by_asset(
                        env,
                        sell_glyph_hash,
                        buy_asset_address,
                        *amount,
                    )
                    .unwrap_or(vec![&env]);

                    if offers.is_empty() {
                        return offer_post_create(
                            env,
                            OfferCreate::Glyph(sell_glyph_hash.clone(), buy),
                        );
                    }

                    let sell_glyph_owner_key = StorageKey::GlyphOwner(sell_glyph_hash.clone());
                    let sell_glyph_owner_address =
                        glyph_verify_ownership(env, &sell_glyph_owner_key);

                    reward_minter_and_miners(
                        env,
                        sell_glyph_owner_address,
                        sell_glyph_hash,
                        amount,
                        buy_asset_address,
                        None,
                    )?;

                    // Remove Asset counter offer
                    let buy_asset_owner = offers.pop_front().unwrap();

                    if offers.is_empty() {
                        remove_asset_offers_by_asset(
                            env,
                            sell_glyph_hash,
                            buy_asset_address,
                            *amount,
                        );
                    } else {
                        write_asset_offers_by_asset(
                            env,
                            sell_glyph_hash,
                            buy_asset_address,
                            *amount,
                            &offers,
                        );
                    }

                    // Transfer ownership of Glyph from Glyph giver to Glyph taker
                    write_glyph_owner(env, sell_glyph_hash, &buy_asset_owner);

                    // Remove all other sell offers for this glyph
                    remove_glyph_offer(env, sell_glyph_hash);

                    // events::asset_offer_post(
                    //     env,
                    //     &buy_asset_address,
                    //     &buy_asset_owner,
                    //     sell_glyph_hash,
                    //     *amount,
                    // );

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
            let sell_glyph_owner_address = glyph_verify_ownership(env, &sell_glyph_owner_key);

            // Selling a Glyph
            let mut offers = read_offers_by_glyph(env, &sell_glyph_hash);

            match offers.binary_search(&buy) {
                Err(offer_index) => offers.insert(offer_index, buy.clone()), // Buy can be an Asset or a Glyph
                _ => return Err(Error::NotEmpty),                            // Error on dupe offer
            }

            write_offers_by_glyph(env, &sell_glyph_hash, offers);

            // events::glyph_offer_post(env, &sell_glyph_hash, &sell_glyph_owner_address, buy);
            
            Ok(())
        }
        OfferCreate::Asset(
            buy_glyph_hash,
            sell_asset_owner_address,
            sell_asset_address,
            amount,
        ) => {
            sell_asset_owner_address.require_auth();

            let token = token::Client::new(env, &sell_asset_address);
            token.transfer(
                &sell_asset_owner_address,
                &env.current_contract_address(),
                &amount,
            );

            let mut offers =
                read_asset_offers_by_asset(env, &buy_glyph_hash, &sell_asset_address, amount)
                    .unwrap_or(Vec::new(env));
            if offers.contains(sell_asset_owner_address.clone()) {
                return Err(Error::NotEmpty); // Error on dupe offer
            }
            offers.push_back(sell_asset_owner_address.clone());

            write_asset_offers_by_asset(env, &buy_glyph_hash, &sell_asset_address, amount, &offers);

            // events::asset_offer_post(
            //     env,
            //     &sell_asset_address,
            //     &sell_asset_owner_address,
            //     &buy_glyph_hash,
            //     amount,
            // );

            Ok(())
        }
    }
}

pub fn offer_delete(env: &Env, sell: Offer, buy: Option<Offer>) -> Result<(), Error> {
    match sell {
        Offer::Glyph(glyph_hash) => {
            // Selling a Glyph (delete Glyph or Asset buy offer)
            let glyph_owner_key = StorageKey::GlyphOwner(glyph_hash.clone());
            let glyph_owner = glyph_verify_ownership(env, &glyph_owner_key);

            let mut offers = read_offers_by_glyph(env, &glyph_hash);

            match &buy {
                Some(buy) => match offers.binary_search(buy) {
                    Ok(offer_index) => {
                        offers.remove(offer_index);
                        write_offers_by_glyph(env, &glyph_hash, offers);

                        // events::glyph_offer_delete(
                        //     env,
                        //     &glyph_hash,
                        //     &glyph_owner,
                        //     buy.clone(),
                        //     offer_index,
                        // );

                        Ok(())
                    }
                    _ => Err(Error::NotFound),
                },
                None => {
                    remove_glyph_offer(env, &glyph_hash);

                    // events::glyph_offer_delete_all(env, &glyph_hash, &glyph_owner);

                    Ok(())
                }
            }
        }
        Offer::AssetSell(asset_owner_address, asset_address, amount) => {
            // Selling an Asset (delete Glyph buy offer)
            asset_owner_address.require_auth();

            match buy {
                Some(Offer::Glyph(glyph_hash)) => {
                    let mut offers =
                        read_asset_offers_by_asset(env, &glyph_hash, &asset_address, amount)
                            .ok_or(Error::NotFound)?;

                    match offers.binary_search(asset_owner_address.clone()) {
                        Ok(offer_index) => {
                            let token = token::Client::new(env, &asset_address);

                            token.transfer(
                                &env.current_contract_address(),
                                &asset_owner_address,
                                &amount,
                            );

                            offers.remove(offer_index);

                            // events::asset_offer_delete(
                            //     env,
                            //     &asset_address,
                            //     &asset_owner_address,
                            //     &glyph_hash,
                            //     amount,
                            //     offer_index,
                            // );

                            if offers.is_empty() {
                                remove_asset_offers_by_asset(
                                    env,
                                    &glyph_hash,
                                    &asset_address,
                                    amount,
                                );
                            } else {
                                write_asset_offers_by_asset(
                                    env,
                                    &glyph_hash,
                                    &asset_address,
                                    amount,
                                    &offers,
                                );
                            }

                            Ok(())
                        }
                        _ => Err(Error::NotFound),
                    }
                }
                Some(_) => Err(Error::NotPermitted), // You cannot sell an Asset for an Asset
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
            let offers = read_offers_by_glyph(env, &glyph_hash);
            let not_exists = buy
                .clone()
                .map_or(true, |buy| offers.binary_search(buy).is_err());
            if (!offers.is_empty() && buy.is_none()) || !not_exists {
                Ok(())
            } else {
                Err(Error::NotFound)
            }
        }
        Offer::Asset(asset_hash, amount) => {
            // Selling an Asset
            match buy {
                Some(Offer::Glyph(glyph_hash)) => {
                    if has_asset_offers_by_asset(env, &glyph_hash, &asset_hash, amount) {
                        Ok(())
                    } else {
                        Err(Error::NotFound)
                    }
                }
                Some(_) => Err(Error::NotPermitted), // cannot sell asset for asset
                None => Err(Error::MissingBuy),
            }
        }
        Offer::AssetSell(seller_address, asset_hash, amount) => {
            // Selling an Asset but check for a specific seller address
            match buy {
                Some(Offer::Glyph(glyph_hash)) => {
                    let offers = read_asset_offers_by_asset(env, &glyph_hash, &asset_hash, amount)
                        .ok_or(Error::NotFound)?;
                    if offers.contains(seller_address) {
                        return Ok(());
                    }

                    Err(Error::NotFound)
                }
                Some(_) => Err(Error::NotPermitted), // You cannot sell an Asset for an Asset
                None => Err(Error::MissingBuy), // When looking up a Glyph offer for an Asset you must specify the buy offer (and it must be for a Glyph
            }
        }
    }
}

fn transfer_ownership(env: &Env, hash: &BytesN<32>, new_owner: &Address) {
    let glyph_owner_key = StorageKey::GlyphOwner(hash.clone());
    let glyph_offer_key = StorageKey::GlyphOffer(hash.clone());

    env.storage().persistent().set(&glyph_owner_key, &new_owner);

    if env.storage().persistent().has(&glyph_offer_key) {
        // TODO emit offer remove event?
        env.storage().persistent().remove(&glyph_offer_key);
    }
}

fn reward_minter_and_miners(
    env: &Env,
    glyph_owner: Address,
    hash: &BytesN<32>,
    amount: &i128,
    asset: &Address,
    asset_owner: Option<Address>,
) -> Result<(), Error> {
    let mut leftover_amount = *amount;

    // Get glyph
    let glyph = read_glyph_or_error(env, hash);
    let glyph_minter_address = read_glyph_minter(env, hash).ok_or(Error::NotFound)?;

    // Pay the glyph minter their cut
    let minter_royalty_rate = read_minter_royalty_rate(env);
    let minter_amount = minter_royalty_rate.fixed_mul_ceil(*amount, 100).unwrap();

    let token = token::Client::new(env, asset);
    make_transfer(
        env,
        Some(&mut leftover_amount),
        &token,
        &asset_owner,
        &glyph_minter_address,
        &minter_amount,
    );

    // Loop over miners
    // NOTE currently can support 17 miners
    for (miner_address, colors_indexes) in glyph.colors.iter() {
        let mut color_count: u32 = 0;

        // Count colors per miner
        for (_, indexes) in colors_indexes.iter() {
            color_count += indexes.len();
        }

        let miner_royalty_rate = read_miner_royalty_rate(env);
        let miner_amount = miner_royalty_rate
            .fixed_mul_ceil(*amount, 100)
            .unwrap()
            .fixed_mul_ceil(color_count as i128, glyph.length as i128)
            .unwrap();

        // Determine their percentage of whole
        // Derive their share of the amount
        // Make payment
        make_transfer(
            env,
            Some(&mut leftover_amount),
            &token,
            &asset_owner,
            &miner_address,
            &miner_amount,
        );
    }

    make_transfer(
        env,
        None,
        &token,
        &asset_owner,
        &glyph_owner,
        &leftover_amount,
    );
    Ok(())
}

fn make_transfer(
    env: &Env,
    leftover: Option<&mut i128>,
    token: &TokenClient,
    asset_owner: &Option<Address>,
    comp: &Address,
    amount: &i128,
) {
    let (do_transfer, from_self) = asset_owner
        .clone()
        .map_or((true, true), |owner| (&owner != comp, false));
    if do_transfer {
        // eliminate self payments
        let curr_addr = env.current_contract_address();
        token.transfer(
            if from_self {
                &curr_addr
            } else {
                &asset_owner.as_ref().unwrap()
            },
            &comp,
            amount,
        );

        if let Some(leftover) = leftover {
            *leftover -= amount;
        }
    }
}
