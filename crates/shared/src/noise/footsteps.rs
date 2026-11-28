//! How loud someone's footsteps are: walking, more in armour, less sneaking
//! (`ability::Ability::Sneak`), and less again by the Sneaking skill
//! (`stealth/sneaking`).

use crate::bonus::armour_grams;
use crate::item::Item;
use crate::movement::{FULL, MOVEMENT_SKILLS};
use crate::skill::Skills;
use crate::spot::CELL_SPAN;

/// How far walking is heard in the open by normal hearing, in cells.
pub const WALK_CELLS: u32 = 5;

/// Grams of worn armour that add a cell to it: 5 kg (full plate, about 25 kg,
/// adds 5).
pub const ARMOUR_GRAMS_PER_CELL: u32 = 5_000;

/// What sneaking leaves of it at 0% Sneaking, per mille: half.
pub const SNEAK_WORST_PER_MILLE: u32 = 500;

/// The same at 100% Sneaking: an eighth.
pub const SNEAK_BEST_PER_MILLE: u32 = 125;

/// What footsteps are made of.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Footsteps {
    /// Sneak is on.
    pub sneaking: bool,
    /// The Sneaking skill's value, up to `movement::FULL`.
    pub skill: u32,
    /// The worn armour's own weight (`bonus::armour_grams`).
    pub armour_grams: u32,
}

impl Footsteps {
    /// Someone's: sneaking or not, with their skills and what they wear.
    pub fn of(sneaking: bool, skills: &Skills, worn: &[Item]) -> Self {
        Self {
            sneaking,
            skill: skills.value(MOVEMENT_SKILLS.sneaking),
            armour_grams: armour_grams(worn),
        }
    }

    /// How far they are heard in the open by normal hearing, in units.
    pub fn loudness(self) -> u32 {
        let span = CELL_SPAN as u64;
        let walking = u64::from(WALK_CELLS) * span;
        let armour = u64::from(self.armour_grams) * span / u64::from(ARMOUR_GRAMS_PER_CELL);
        let loud = walking + armour;
        let kept = match self.sneaking {
            false => 1000,
            true => {
                let skill = u64::from(self.skill.min(FULL));
                let gained = u64::from(SNEAK_WORST_PER_MILLE - SNEAK_BEST_PER_MILLE) * skill;
                u64::from(SNEAK_WORST_PER_MILLE) - gained / u64::from(FULL)
            }
        };
        (loud * kept / 1000) as u32
    }
}
