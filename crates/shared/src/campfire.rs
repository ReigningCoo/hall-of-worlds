//! Campfires (cooking plan PR 7): a real fire anyone lights on a level with
//! Make Campfire, learned on first carrying kindling (no tinderbox). Each
//! piece of kindling burns 5 to 10 real minutes by its wood, it holds at most
//! `MAX_KINDLING` unburnt and is fed while it burns; it gives light while lit
//! and leaves Ash when it burns out. A `cooking::Heat::Fire` source later.
//! The server keeps fires and decides; the client draws them. See
//! `docs/context/shared/campfire.md`.

mod fire;
mod kindling;
mod place;

pub use fire::{Campfire, FeedRefused};
pub use kindling::kindling_to_burn;
pub use place::{campfire_faced, can_hold_fire, can_make_fire, place_faced};

use crate::energy::ACTION_TICKS_PER_SECOND;
use crate::material::Material;

/// The most kindling a fire holds unburnt, the one burning included (plan:
/// "about 5").
pub const MAX_KINDLING: usize = 5;

/// What one Ash item weighs: a handful from each piece of kindling burnt.
pub const ASH_GRAMS: u32 = 20;

/// Lighting a fire: seconds of striking and blowing (Claude's choice).
pub const LIGHT_SECONDS: u64 = 5;

/// Feeding a fire a piece of kindling: seconds (Claude's choice).
pub const FEED_SECONDS: u64 = 1;

/// How far a lit fire's light reaches, in cells (as a carried light's).
pub const LIGHT_CELLS: u32 = 6;

/// `LIGHT_SECONDS` in action ticks.
pub const fn light_ticks() -> u64 {
    LIGHT_SECONDS * ACTION_TICKS_PER_SECOND as u64
}

/// `FEED_SECONDS` in action ticks.
pub const fn feed_ticks() -> u64 {
    FEED_SECONDS * ACTION_TICKS_PER_SECOND as u64
}

/// How many real seconds one piece of `wood`'s kindling burns: 5 minutes for
/// the lightest woods (density 2, baobab) to 10 for the densest (11, mopane),
/// in proportion between; a material that isn't a tree's as the middle.
pub fn burn_seconds(wood: Material) -> u32 {
    const LIGHTEST: u32 = 2;
    const DENSEST: u32 = 11;
    let density = wood.wood().map_or(6, |w| w.density);
    let density = density.clamp(LIGHTEST, DENSEST);
    300 + (density - LIGHTEST) * 300 / (DENSEST - LIGHTEST)
}

#[cfg(test)]
mod tests;
