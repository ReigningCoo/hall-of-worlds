//! What items are worth (trading plan PR 1, owner's rule): (materials +
//! labour) × quality, in whole numbers both sides work out the same. Values
//! are kept in fine units, a thousandth of a copper, and only rounded up to
//! copper when shown or paid (`copper_shown`).
//!
//! - Materials: the item's own weight × a price a kilogram for what it is made
//!   of (`per_kg`), the finer (its tier) the dearer.
//! - Labour: the time to make it at Common with no bonuses (an hour × its
//!   kind's `make_percent`), paid at `LABOUR_PER_MINUTE`.
//! - Raw goods (herbs, fruit, vegetables, fungi, eggs, flour, wood, ash) by
//!   weight and abundance, with no labour (`raw.rs`).
//! - Liquids by the litre (`liquid.rs`); containers add what is in them.
//! - Quality doubles it: Poor 1×, Common 2× … Legendary 32×, Artifact 64×;
//!   anything without a quality counts as Common.
//! - Coins are worth what they say; quest items nothing.
//!
//! All figures are first guesses, anchored so a Common apple is 1 copper.

mod coins;
mod item;
mod liquid;
mod price;
mod raw;

pub use coins::Coins;
pub use item::{each, total};
pub use liquid::per_litre;
pub use price::per_kg;

use crate::item::Quality;

/// Fine units in a copper coin.
pub const FINE_PER_COPPER: u64 = 1000;

/// What a minute of a crafter's time adds, in fine units: half a copper.
pub const LABOUR_PER_MINUTE: u64 = 500;

/// A Common item's time to make, in game minutes, before its kind's share.
pub const BASE_MINUTES: u64 = 60;

/// What quality multiplies a value by.
pub const fn quality_times(quality: Quality) -> u64 {
    match quality {
        Quality::Poor => 1,
        Quality::Common => 2,
        Quality::Uncommon => 4,
        Quality::Rare => 8,
        Quality::Epic => 16,
        Quality::Legendary => 32,
        Quality::Artifact => 64,
    }
}

/// `fine` as whole coppers, rounded up: what is shown or paid.
pub const fn copper_shown(fine: u64) -> u64 {
    fine.div_ceil(FINE_PER_COPPER)
}

#[cfg(test)]
mod tests;
