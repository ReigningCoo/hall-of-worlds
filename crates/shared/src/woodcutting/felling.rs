//! How long felling a tree takes (owner): 60 seconds untrained with a basic
//! axe, down to 30 at best, by the cube roots of the Woodcutting skill and the
//! axe's quality; any axe but a hatchet half as long again.

use super::{HeldAxe, WOODCUTTING_SKILL};
use crate::energy::ACTION_TICKS_PER_SECOND;
use crate::herb::{cube_root, skill_value};
use crate::item::Quality;
use crate::skill::Skills;

/// Untrained, with a basic (Common) hatchet, in hundredths of a second.
const FIRST_CENTIS: u64 = 6000;

/// Taken off for 100% of the skill, and again for the best axe: 15 seconds
/// each, so 30 seconds at best.
const MOST_OFF_CENTIS: u64 = 1500;

/// A share from 0 to 100: the cube root of `n` out of a million.
fn share(n: u32) -> u64 {
    u64::from(cube_root(n.min(1_000_000)))
}

/// The skill's share of the time taken off: ∛ of its value, 0 to 100.
fn skill_share(skills: &Skills) -> u64 {
    share(skill_value(skills, WOODCUTTING_SKILL) * 100)
}

/// The axe's share: ∛ of its steps above Common (Poor counting as Common) out
/// of Artifact's five, 0 to 100.
fn quality_share(quality: Quality) -> u64 {
    let rank = Quality::ALL.iter().position(|&q| q == quality).unwrap_or(1) as u32;
    share(rank.saturating_sub(1) * 200_000)
}

/// How long felling a whole tree takes, in action ticks.
pub fn fell_ticks(skills: &Skills, axe: HeldAxe) -> u64 {
    let off = MOST_OFF_CENTIS * (skill_share(skills) + quality_share(axe.quality)) / 100;
    let mut centis = FIRST_CENTIS - off;
    if !axe.hatchet {
        centis = centis * 3 / 2;
    }
    centis * ACTION_TICKS_PER_SECOND as u64 / 100
}

/// The same in whole seconds, rounded, for showing.
pub fn fell_seconds(skills: &Skills, axe: HeldAxe) -> u64 {
    (fell_ticks(skills, axe) + ACTION_TICKS_PER_SECOND as u64 / 2) / ACTION_TICKS_PER_SECOND as u64
}
