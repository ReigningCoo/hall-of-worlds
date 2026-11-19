//! What quality the yeast shops sell is: nearly always Common (owner,
//! 2026-11-18), whatever the shop.

use crate::item::Quality;
use crate::rng::Rng;

/// Out of 100, how often shop yeast keeps the quality its shop rolled instead
/// of being Common. A first guess.
const NOT_COMMON_PERCENT: u64 = 5;

/// The quality of yeast a shop made, given the quality it rolled.
pub fn shop_quality(rolled: Quality, rng: &mut Rng) -> Quality {
    match rng.range_u64(0, 100) < NOT_COMMON_PERCENT {
        true => rolled,
        false => Quality::Common,
    }
}
