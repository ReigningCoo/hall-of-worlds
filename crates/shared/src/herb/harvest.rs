//! What harvesting a patch gives: how many, and how good. First guesses.

use super::{HerbDef, STAGES, pick_quality};
use crate::item::Quality;
use crate::rng::Rng;

/// How many a full-grown patch gives, inclusive.
pub const FULL_HARVEST: (u64, u64) = (2, 4);

/// What one harvest gives.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Harvest {
    pub count: u32,
    pub quality: Quality,
}

/// Harvesting a patch of `def` at `stage` (`EARLY_STAGE` or full grown) by
/// someone with this much Foraging (`skill_value`): `None` when it gives
/// nothing. Early, it gives its `Early::taken` share of a full harvest, and is
/// as good as Foraging times its `Early::strength` would make it.
pub fn harvest(def: &HerbDef, stage: u8, foraging: u32, rng: &mut Rng) -> Option<Harvest> {
    let full = rng.range_u64(FULL_HARVEST.0, FULL_HARVEST.1 + 1);
    let (taken, strength) = match stage {
        STAGES => (100, 100),
        _ => (u64::from(def.early.taken), u64::from(def.early.strength)),
    };
    let count = (full * taken + 50) / 100;
    if count == 0 {
        return None;
    }
    let foraging = (u64::from(foraging) * strength.min(100) / 100) as u32;
    Some(Harvest {
        count: count as u32,
        quality: pick_quality(rng, foraging),
    })
}

#[cfg(test)]
mod tests;
