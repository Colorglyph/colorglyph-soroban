#![cfg(test)]

use std::println;
extern crate std;

use crate::types::{
    AssetAmount, AssetOffer, AssetOfferArg, Error, GlyphOfferArg, MinerOwnerColor, Offer,
    OfferType, StorageKey,
};
use fixed_point_math::FixedPoint;
use soroban_sdk::{testutils::Address as _, Address, Bytes, Env, Vec};

#[test]
fn test_bytes_stuff() {
    let env = Env::default();

    let mut b_palette = Bytes::new(&env);

    for i in 0..10 {
        b_palette.push(i);
    }

    b_palette.copy_from_slice(0, &[255; 3]);

    println!("{:?}", b_palette);
}

#[test]
fn test_mootz_math() {
    const MINTER_ROYALTY_RATE: i128 = 3;
    const MINER_ROYALTY_RATE: i128 = 2;

    let amount = 16i128;
    let total_pixels = 10u32;
    let miner_pixels = 10u32;

    let res1 = MINTER_ROYALTY_RATE.fixed_mul_ceil(amount, 100).unwrap();

    println!("{}", res1);

    let res2 = MINER_ROYALTY_RATE
        .fixed_mul_ceil(amount, 100)
        .unwrap()
        .fixed_mul_ceil(i128::from(miner_pixels), i128::from(total_pixels))
        .unwrap();

    println!("{}", res2);
}

#[test]
fn test_vec_pop() {
    let env = Env::default();

    let mut items_front: Vec<Address> = Vec::new(&env);
    let mut items_back: Vec<Address> = Vec::new(&env);

    env.budget().reset_default();

    for _ in 0..10 {
        items_front.push_front(Address::random(&env));
    }

    println!("{:?}", items_front.len());

    let test = items_front.pop_front();

    println!("{:?}", test);
    println!("{:?}", items_front.len());
    // - CPU Instructions: 258920
    // - Memory Bytes: 11437
    println!("items front {:?}", env.budget().print());

    env.budget().reset_default();

    for _ in 0..10 {
        items_back.push_back(Address::random(&env));
    }

    println!("{:?}", items_back.len());

    let test = items_back.pop_back();

    println!("{:?}", test);
    println!("{:?}", items_back.len());
    // - CPU Instructions: 258920
    // - Memory Bytes: 11437
    println!("items back {:?}", env.budget().print());
}

#[test]
fn test_binary_vs_index() {
    let env = Env::default();

    let item = AssetAmount(Address::random(&env), 10i128);
    let mut unsorted: Vec<AssetAmount> = Vec::new(&env);
    let mut binary_sorted: Vec<AssetAmount> = Vec::new(&env);
    let mut index_sorted: Vec<AssetAmount> = Vec::new(&env);

    for i in 0..10 {
        unsorted.push_back(AssetAmount(Address::random(&env), i));
    }

    unsorted.push_back(item.clone());

    env.budget().reset_default();

    for v in unsorted.clone().into_iter_unchecked() {
        match binary_sorted.binary_search(&v) {
            Result::Err(i) => binary_sorted.insert(i, v),
            _ => (),
        }
    }

    // - CPU Instructions: 563584
    // - Memory Bytes: 44879
    println!("binary build {:?}", env.budget().print());

    env.budget().reset_default();

    for v in unsorted.clone().into_iter_unchecked() {
        index_sorted.push_back(v);
    }

    // - CPU Instructions: 413640
    // - Memory Bytes: 25818
    println!("index build {:?}", env.budget().print());

    env.budget().reset_default();

    let index = binary_sorted.binary_search(&item).unwrap();
    binary_sorted.get(index).unwrap().unwrap();

    // - CPU Instructions: 17458
    // - Memory Bytes: 3551
    println!("binary get {:?}", env.budget().print());

    env.budget().reset_default();

    let index = index_sorted.first_index_of(&item).unwrap();
    index_sorted.get(index).unwrap().unwrap();

    // - CPU Instructions: 24582
    // - Memory Bytes: 6351
    println!("index get {:?}", env.budget().print());
}

pub fn color_balance(
    env: &Env,
    id: &Address,
    owner: Address,
    miner: Option<Address>,
    color: u32,
) -> u32 {
    let miner = match miner {
        None => owner.clone(),
        Some(address) => address,
    };

    env.as_contract(&id, || {
        env.storage()
            .get::<MinerOwnerColor, u32>(&MinerOwnerColor(miner, owner, color))
            .unwrap_or(Ok(0))
            .unwrap()
    })
}

pub fn offers_get(
    env: &Env,
    id: &Address,
    sell: &OfferType,
    buy: &OfferType,
) -> Result<Offer, Error> {
    env.as_contract(id, || {
        match sell {
            OfferType::Glyph(offer_hash) => {
                // Selling a Glyph
                let offers = env
                    .storage()
                    .get::<StorageKey, Vec<OfferType>>(&StorageKey::GlyphOffer(offer_hash.clone()))
                    .ok_or(Error::NotFound)?
                    .unwrap();

                match offers.binary_search(buy) {
                    Ok(offer_index) => {
                        let offer_owner = env
                            .storage()
                            .get::<StorageKey, Address>(&StorageKey::GlyphOwner(offer_hash.clone()))
                            .ok_or(Error::NotFound)?
                            .unwrap();

                        // We don't always use glyph_offers & offer_index but they're necessary to lookup here as it's how we look for a specific offer
                        Ok(Offer::Glyph(GlyphOfferArg(
                            offer_index,
                            offers,
                            offer_owner,
                            offer_hash.clone(),
                        )))
                    }
                    _ => Err(Error::NotFound),
                }
            }
            OfferType::Asset(AssetAmount(asset_hash, amount)) => {
                // Selling an Asset
                match buy {
                    OfferType::Glyph(glyph_hash) => {
                        let offer = AssetOffer(glyph_hash.clone(), asset_hash.clone(), *amount);
                        let offers = env
                            .storage()
                            .get::<StorageKey, Vec<Address>>(&StorageKey::AssetOffer(offer.clone()))
                            .ok_or(Error::NotFound)?
                            .unwrap();

                        Ok(Offer::Asset(AssetOfferArg(offers, offer)))
                    }
                    _ => Err(Error::NotPermitted), // You cannot sell an Asset for an Asset
                }
            }
        }
    })
}
