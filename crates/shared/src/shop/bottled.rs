//! What a shop's bottles hold: Herbalists and Potion Shops sell waters (never
//! Salt Water) and oils (brewing plan, owner, 2026-11-17).

use crate::item::ItemKind;
use crate::item_form::RollRequest;
use crate::potion::{LiquidKind, Oil, Water};
use crate::rng::Seed;
use crate::site::BuildingKind;

/// The liquids in the bottles a kind of building sells, each as likely.
const BREWERS: &[LiquidKind] = &[
    LiquidKind::Water(Water::Plain),
    LiquidKind::Water(Water::Cave),
    LiquidKind::Water(Water::Moon),
    LiquidKind::Oil(Oil::Vegetable),
    LiquidKind::Oil(Oil::Nut),
    LiquidKind::Oil(Oil::Seed),
];

/// The liquids a kind of building's bottles hold: none if it sells no bottles.
pub fn bottled(kind: BuildingKind) -> &'static [LiquidKind] {
    match kind {
        BuildingKind::Herbalist | BuildingKind::PotionShop => BREWERS,
        _ => &[],
    }
}

/// What a new item of `item`'s kind in this shop's stock is asked to be, from
/// its own seed: a bottle holds one of `bottled`'s liquids; anything else is
/// rolled as usual.
pub fn stock_request(shop: BuildingKind, item: ItemKind, seed: Seed) -> RollRequest {
    let liquids = bottled(shop);
    if item != ItemKind::Bottle || liquids.is_empty() {
        return RollRequest::NONE;
    }
    let mut rng = seed.derive("shop liquid").rng();
    let at = rng.range_u64(0, liquids.len() as u64) as usize;
    RollRequest {
        liquid: Some(liquids[at]),
        ..RollRequest::NONE
    }
}
