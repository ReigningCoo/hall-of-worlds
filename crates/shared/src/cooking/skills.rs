//! The skills a cooking process leans on: Cooking, and each tool's sub-skill.

use super::{COOKING, Tool};
use crate::herb::{cube_root, skill_percent, skill_value};
use crate::skill::Skills;

/// How good a cook is, each skill in hundredths of a percent as a check
/// counts it (`herb::skill_value`: the values along its path, at most 100%).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CookingSkills {
    /// Cooking, for the chance and the step limit.
    pub cooking: u32,
    /// Each tool's sub-skill, in `Tool::ALL`'s order.
    pub sub: [u32; Tool::ALL.len()],
}

impl CookingSkills {
    /// A character's, from their skills.
    pub fn of(skills: &Skills) -> Self {
        Self {
            cooking: skill_value(skills, COOKING),
            sub: Tool::ALL.map(|tool| skill_value(skills, tool.sub_skill())),
        }
    }

    /// Cooking in whole percent, 0 to 100.
    pub fn cooking_percent(self) -> u32 {
        skill_percent(self.cooking)
    }

    /// The whole cube root of this tool's sub-skill in percent, 0 to 4: what
    /// it adds to quality.
    pub fn sub_root(self, tool: Tool) -> u32 {
        cube_root(skill_percent(self.sub[tool.index()]))
    }
}
