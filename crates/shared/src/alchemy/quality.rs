//! A product's quality (Alchemy plan): the purity of its doses as brewing had
//! it, plus whole steps from the sub-skills used when the chance is certain,
//! a step less for an old source, and the server's luck on success.

use super::{AlchemySkills, Apparatus, CERTAIN};
use crate::item::Quality;

/// Rolls (of `CERTAIN`) under this give two steps more: 0.1%.
pub const LUCKY_TWICE: u32 = 10;
/// Rolls under this (and not under `LUCKY_TWICE`) give a step more: 1%.
pub const LUCKY_ONCE: u32 = LUCKY_TWICE + 100;

/// Whole quality steps the sub-skills add: steps × the mean ∛ of the
/// sub-skills used (each apparatus once, `AlchemySkills::sub_root`) ÷ 10,
/// rounded down once.
pub fn skill_steps(steps: u32, used: &[Apparatus], skills: AlchemySkills) -> u32 {
    let mut distinct: Vec<Apparatus> = used.to_vec();
    distinct.sort_unstable();
    distinct.dedup();
    if distinct.is_empty() {
        return 0;
    }
    let roots: u32 = distinct.iter().map(|&a| skills.sub_root(a)).sum();
    steps * roots / (distinct.len() as u32 * 10)
}

/// `quality` moved `by` steps, kept from Poor to Legendary (alchemy never
/// makes an Artifact).
pub fn moved(quality: Quality, by: i32) -> Quality {
    let at = Quality::ALL.iter().position(|&q| q == quality).unwrap_or(0) as i32;
    let legendary = Quality::ALL.len() as i32 - 2;
    Quality::ALL[(at + by).clamp(0, legendary) as usize]
}

/// The least quality a process makes, what the preview shows: `purity`
/// (`brewing::quality` of the product), plus `skill_steps` only when `chance`
/// is at least `CERTAIN`, less a step for an old source.
pub fn least_quality(purity: Quality, chance: u32, skill_steps: u32, old: bool) -> Quality {
    let bonus = if chance >= CERTAIN {
        skill_steps as i32
    } else {
        0
    };
    moved(purity, bonus - i32::from(old))
}

/// Steps luck adds on success, for the server's `roll` from 0 to `CERTAIN` −
/// 1: 2 at 0.1%, 1 at 1%, otherwise none.
pub fn luck_steps(roll: u32) -> i32 {
    match roll {
        r if r < LUCKY_TWICE => 2,
        r if r < LUCKY_ONCE => 1,
        _ => 0,
    }
}
