//! What picking a full grown vegetable plant gives (owner, 2026-11-17): its
//! harvest tier untrained (1 to 6 items), with Crops and Foraging adding as
//! Orchards and Foraging do for fruit.

use super::VegetableDef;
use crate::herb::{cube_root, skill_percent};
use crate::rng::Rng;

/// The skill of growing crops: it adds to a harvest's least and most alike,
/// and picking vegetables practises it.
pub const CROPS: &str = "gathering/farming/crops";

/// The least and most picking `def` gives someone with this much Foraging and
/// Crops (`herb::skill_value`, hundredths of a percent): its harvest tier, the
/// cube root of Foraging's percent added to the most, and of Crops' to both
/// (rounded down).
pub fn harvest_range(def: &VegetableDef, foraging: u32, crops: u32) -> (u32, u32) {
    let (least, most) = def.harvest.range();
    let foraging = cube_root(skill_percent(foraging));
    let crops = cube_root(skill_percent(crops));
    (least + crops, most + foraging + crops)
}

/// How many items of `def` picking a full grown plant gives someone with this
/// much Foraging and Crops: always at least one.
pub fn harvest(def: &VegetableDef, (foraging, crops): (u32, u32), rng: &mut Rng) -> u32 {
    let (least, most) = harvest_range(def, foraging, crops);
    rng.range_u64(u64::from(least), u64::from(most) + 1) as u32
}
