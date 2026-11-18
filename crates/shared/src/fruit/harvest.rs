//! What picking a ripe fruit plant gives (owner, 2026-11-17): its row's
//! harvest tier untrained, with Orchards and Foraging adding as Herbs and
//! Foraging do for herbs. Big fruits come whole: an item is one whole fruit.

use super::FruitDef;
use crate::herb::{cube_root, skill_percent};
use crate::rng::Rng;

/// The skill of growing fruit: it adds to a harvest's least and most alike,
/// and picking fruit practises it.
pub const ORCHARDS: &str = "gathering/farming/orchards";

/// The least and most picking `def` gives someone with this much Foraging and
/// Orchards (`herb::skill_value`, hundredths of a percent): its harvest tier,
/// the cube root of Foraging's percent added to the most, and of Orchards' to
/// both (rounded down).
pub fn harvest_range(def: &FruitDef, foraging: u32, orchards: u32) -> (u32, u32) {
    let (least, most) = def.harvest.range();
    let foraging = cube_root(skill_percent(foraging));
    let orchards = cube_root(skill_percent(orchards));
    (least + orchards, most + foraging + orchards)
}

/// How many items of `def` picking a ripe plant gives someone with this much
/// Foraging and Orchards: always at least one.
pub fn harvest(def: &FruitDef, (foraging, orchards): (u32, u32), rng: &mut Rng) -> u32 {
    let (least, most) = harvest_range(def, foraging, orchards);
    rng.range_u64(u64::from(least), u64::from(most) + 1) as u32
}
