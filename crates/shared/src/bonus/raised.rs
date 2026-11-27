//! `Raised`: what bonuses add to skills' values (`Stat::Skill`), for the rules
//! that read skills with their bonuses (`combat::Fighter`).

use std::collections::BTreeMap;

use super::MILLI;
use super::stat::Stat;
use super::totals::Totals;
use crate::skill::SkillId;

/// Each raised skill's added value, in its hundredths (`skill::VALUE_SCALE`),
/// negative when lowered. Empty for mobs.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Raised(BTreeMap<SkillId, i64>);

impl Raised {
    /// The skills these totals raise or lower (flat bonuses only).
    pub fn of(totals: &Totals) -> Self {
        Self(
            totals
                .skills()
                .map(|(id, flat)| (id, flat / MILLI))
                .collect(),
        )
    }

    /// A skill's value with what it is raised by, never below 0.
    pub fn value(&self, id: SkillId, value: u32) -> u32 {
        let added = self.0.get(&id).copied().unwrap_or(0);
        (i64::from(value) + added).clamp(0, i64::from(u32::MAX)) as u32
    }
}

impl Totals {
    /// Every skill with a flat bonus, and its sum in thousandths.
    pub fn skills(&self) -> impl Iterator<Item = (SkillId, i64)> + '_ {
        self.0.iter().filter_map(|(stat, sum)| match stat {
            Stat::Skill(id) => Some((*id, sum.flat)),
            _ => None,
        })
    }
}
