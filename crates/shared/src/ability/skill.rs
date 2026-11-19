//! Abilities that come from a skill: known by whoever knows the skill.

use super::Ability;
use crate::skill::{SkillId, Skills};

/// Alchemy's path in the skill tree: Press Oil, Press Juice and Alchemy come
/// from it.
pub const ALCHEMY: &str = "crafting/alchemy";

/// Skills whose abilities everyone knows for now, skill or not: Alchemy, until
/// it can be learned (TODO `todo-learn-alchemy`), and Fermenting, until
/// something teaches it.
const GIVEN_FOR_NOW: &[&str] = &[ALCHEMY, crate::ferment::FERMENTING];

impl Ability {
    /// The skill it comes from, by path, if any.
    pub const fn skill(self) -> Option<&'static str> {
        match self {
            Self::PressOil | Self::PressJuice | Self::Alchemy => Some(ALCHEMY),
            Self::Ferment => Some(crate::ferment::FERMENTING),
            _ => None,
        }
    }

    /// Whether these skills give it: its skill is known (or given to everyone
    /// for now).
    pub(super) fn known_from_skill(self, skills: &Skills) -> bool {
        self.skill().is_some_and(|path| {
            GIVEN_FOR_NOW.contains(&path) || SkillId::find(path).is_some_and(|id| skills.knows(id))
        })
    }
}
