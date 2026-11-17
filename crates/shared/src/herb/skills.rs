//! The skills picking herbs leans on, and how good someone is at one.

use crate::skill::{SkillId, Skills, VALUE_SCALE};

/// The skill that finds more on a plant (`harvest`'s most), and grows with
/// picking.
pub const FORAGING: &str = "gathering/foraging";

/// The skill of growing herbs (owner, 2026-11-17): it adds to a harvest's
/// least and most alike. Nothing practises it yet; planting will.
pub const HERBS: &str = "gathering/farming/herbs";

/// A skill's value at 100%, in hundredths (`Skills::value`).
pub(super) const FULL: u32 = 100 * VALUE_SCALE;

/// How good someone is at the skill at `path`, as a check counts it (its value
/// and every value above it), at most 100%: hundredths of a percent.
pub fn skill_value(skills: &Skills, path: &str) -> u32 {
    SkillId::find(path).map_or(0, |id| skills.total(id).min(FULL))
}

/// A skill's value (`skill_value`) in whole percent, 0 to 100, rounded down.
pub fn skill_percent(value: u32) -> u32 {
    value.min(FULL) / VALUE_SCALE
}

/// The whole cube root of `n`, rounded down: 1 → 1, 8 → 2, 27 → 3, 100 → 4.
/// Whole numbers only, so client and server always agree.
pub fn cube_root(n: u32) -> u32 {
    let mut root = 0;
    while (root + 1) * (root + 1) * (root + 1) <= n {
        root += 1;
    }
    root
}
