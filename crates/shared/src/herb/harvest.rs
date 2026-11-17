//! What harvesting a patch gives: how many (owner, 2026-11-17: 1 to 3, more
//! with Foraging and Herbs) and how good (always its plant's rarity).

use super::{HerbDef, STAGES, cube_root, skill_percent};
use crate::item::Quality;
use crate::rng::Rng;

/// How many a full-grown patch gives an untrained picker, inclusive.
pub const FULL_HARVEST: (u32, u32) = (1, 3);

/// What one harvest gives.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Harvest {
    pub count: u32,
    pub quality: Quality,
}

/// The least and most a full-grown patch gives someone with this much Foraging
/// and Herbs (`skill_value`, hundredths of a percent): `FULL_HARVEST`, the
/// cube root of Foraging's percent added to the most, and of Herbs' to both
/// (rounded down). So 100% Foraging gives 1 to 7, 100% Herbs 5 to 7, both 5 to
/// 11.
pub fn harvest_range(foraging: u32, herbs: u32) -> (u32, u32) {
    let foraging = cube_root(skill_percent(foraging));
    let herbs = cube_root(skill_percent(herbs));
    (FULL_HARVEST.0 + herbs, FULL_HARVEST.1 + foraging + herbs)
}

/// Harvesting a patch of `def` at `stage` (`EARLY_STAGE` or full grown) by
/// someone with this much Foraging and Herbs (`skill_value`): `None` when it
/// gives nothing. Early, it gives its `Early::taken` share of the number
/// rolled. Its quality is its herb's rarity (`Abundance::quality`).
pub fn harvest(
    def: &HerbDef,
    stage: u8,
    (foraging, herbs): (u32, u32),
    rng: &mut Rng,
) -> Option<Harvest> {
    let (least, most) = harvest_range(foraging, herbs);
    let full = rng.range_u64(u64::from(least), u64::from(most) + 1);
    let taken = match stage {
        STAGES => 100,
        _ => u64::from(def.early.taken),
    };
    let count = (full * taken + 50) / 100;
    if count == 0 {
        return None;
    }
    Some(Harvest {
        count: count as u32,
        quality: def.abundance.quality(),
    })
}

#[cfg(test)]
mod tests;
