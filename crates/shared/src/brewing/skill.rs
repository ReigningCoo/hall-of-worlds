//! The skills brewing leans on, and the limits they set (decisions 6, 7, 9,
//! 12 and 13).

use crate::herb::{HERBALISM, cube_root, skill_percent, skill_value};
use crate::skill::Skills;

/// The skill brewing is part of.
pub use crate::ability::ALCHEMY;

/// How good a brewer is: Alchemy and Herbalism, each in hundredths of a
/// percent as a check counts them (`herb::skill_value`), at most 100%.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BrewSkills {
    pub alchemy: u32,
    pub herbalism: u32,
}

impl BrewSkills {
    /// A character's, from their skills.
    pub fn of(skills: &Skills) -> Self {
        Self {
            alchemy: skill_value(skills, ALCHEMY),
            herbalism: skill_value(skills, HERBALISM),
        }
    }

    /// Alchemy in whole percent, 0 to 100.
    pub fn alchemy_percent(self) -> u32 {
        skill_percent(self.alchemy)
    }

    /// Herbalism in whole percent, 0 to 100.
    pub fn herbalism_percent(self) -> u32 {
        skill_percent(self.herbalism)
    }
}

/// The whole square root of `n`, rounded down: 1 → 1, 4 → 2, 99 → 9, 100 → 10.
/// Whole numbers only, so client and server always agree.
pub fn square_root(n: u32) -> u32 {
    let mut root = 0;
    while (root + 1) * (root + 1) <= n {
        root += 1;
    }
    root
}

/// The most steps one herb gives any effect (decision 9): √Alchemy% + 1, so 1
/// untrained, 11 at 100%.
pub fn per_herb_cap(skills: BrewSkills) -> i32 {
    square_root(skills.alchemy_percent()) as i32 + 1
}

/// The most steps any stat comes to (decision 12): 4 × (√Alchemy% + 1), so 4
/// untrained, 44 at 100%.
pub fn stat_cap(skills: BrewSkills) -> i32 {
    4 * per_herb_cap(skills)
}

/// The most kinds of herb in one mix (decision 6, first guess): 4 + ∛Alchemy%,
/// so 4 to 8.
pub fn most_kinds(skills: BrewSkills) -> usize {
    4 + cube_root(skills.alchemy_percent()) as usize
}

/// The most doses one grinding makes (decision 7): ∛Alchemy% + 1, so 1
/// untrained, 5 at 100%.
pub fn most_doses(skills: BrewSkills) -> u32 {
    cube_root(skills.alchemy_percent()) + 1
}

/// How many steps Herbalism takes off unwanted effects (decision 13):
/// ∛Herbalism%, so 0 to 4.
pub fn cut_steps(skills: BrewSkills) -> u32 {
    cube_root(skills.herbalism_percent())
}
