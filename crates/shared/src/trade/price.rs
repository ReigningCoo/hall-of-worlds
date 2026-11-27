//! What a shop pays for one thing and asks for one, in fine units (a
//! thousandth of a copper, `shared::value`), from its value and the rates.

use super::Rates;
use crate::item::ItemKind;
use crate::shop::sells;
use crate::site::BuildingKind;

/// What a shop pays for something it doesn't normally sell, as a share of
/// what it would pay otherwise: after everything else (owner).
pub const NOT_SOLD_PERCENT: u64 = 50;

/// What a shop pays for something worth `value` (fine units).
pub fn shop_pays(value: u64, rates: Rates, normally_sold: bool) -> u64 {
    let pays = value * u64::from(rates.buys) / 100;
    match normally_sold {
        true => pays,
        false => pays * NOT_SOLD_PERCENT / 100,
    }
}

/// What a shop asks for something worth `value` (fine units): the same
/// whether or not it was bought in, as it resells at its normal price.
pub fn shop_asks(value: u64, rates: Rates) -> u64 {
    value * u64::from(rates.sells) / 100
}

/// Whether a shop of this kind normally sells things of this kind.
pub fn normally_sold(shop: BuildingKind, kind: ItemKind) -> bool {
    sells(shop).iter().any(|&(sold, _)| sold == kind)
}
