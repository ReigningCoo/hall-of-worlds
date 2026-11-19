//! The skills a process leans on: Alchemy and Herbalism (the mortar's, as
//! brewing had them) and each apparatus's sub-skill.

use super::Apparatus;
use crate::brewing::BrewSkills;
use crate::herb::{cube_root, skill_percent, skill_value};
use crate::skill::Skills;

/// How good an alchemist is, each skill in hundredths of a percent as a check
/// counts it (`herb::skill_value`: the values along its path, at most 100%).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AlchemySkills {
    /// Alchemy and Herbalism, for the mortar's limits and the chance.
    pub brew: BrewSkills,
    /// Each apparatus's sub-skill, in `Apparatus::ALL`'s order.
    pub sub: [u32; Apparatus::ALL.len()],
}

impl AlchemySkills {
    /// A character's, from their skills.
    pub fn of(skills: &Skills) -> Self {
        Self {
            brew: BrewSkills::of(skills),
            sub: Apparatus::ALL.map(|apparatus| skill_value(skills, apparatus.sub_skill())),
        }
    }

    /// Alchemy in whole percent, 0 to 100.
    pub fn alchemy_percent(self) -> u32 {
        self.brew.alchemy_percent()
    }

    /// The whole cube root of this apparatus's sub-skill in percent, 0 to 4:
    /// what it adds to quality.
    pub fn sub_root(self, apparatus: Apparatus) -> u32 {
        cube_root(skill_percent(self.sub[apparatus.index()]))
    }
}
