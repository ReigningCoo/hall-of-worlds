//! A process's chance of working, as Alchemy's: 100% + Cooking%, halved once
//! for each step beyond ⌈√Cooking%⌉, a join counting as a step and a half.
//! Only this session's own steps count: an intermediate's earlier steps were
//! rolled for when it was made.

use crate::alchemy::{CERTAIN, steps_allowed};

/// The chance, in hundredths of a percent (`alchemy::CERTAIN` is 100%), of a
/// process whose steps weigh `halves` halves of a step (`Step::halves`):
/// 100% + Cooking%, halved (rounded down) once per step, or part of one,
/// beyond `alchemy::steps_allowed`.
pub fn chance(cooking_percent: u32, halves: u32) -> u32 {
    let cooking = cooking_percent.min(100);
    let over = halves.saturating_sub(2 * steps_allowed(cooking));
    let halvings = over.div_ceil(2);
    (CERTAIN + cooking * 100).checked_shr(halvings).unwrap_or(0)
}
