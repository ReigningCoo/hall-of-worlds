//! Running a process (Alchemy plan PR 3): which item each apparatus is, the
//! sub-skills learned on first carrying one, how long a run takes and the
//! practice it gives.

use super::{AlchemySkills, Apparatus};
use crate::energy::ACTION_TICKS_PER_SECOND;
use crate::item::{Inventory, ItemKind};
use crate::skill::{SkillId, Skills};

/// Practice to a step's sub-skill each run, whatever comes of it (flowing up
/// to Alchemy; first guess, as Grind's was).
pub const STEP_PRACTICE: u32 = 8;
/// Herbalism practice for each kind of herb handled (as Grind's was).
pub const HERB_PRACTICE: u32 = 2;

impl Apparatus {
    /// The item it is, carried to use it and to learn its sub-skill; none for
    /// those not built yet.
    pub fn item(self) -> Option<ItemKind> {
        match self {
            Self::Mortar => Some(ItemKind::MortarAndPestle),
            Self::Retort => Some(ItemKind::Retort),
            _ => None,
        }
    }

    /// Seconds a step on it takes untrained (first guesses: the Mortar as
    /// Grind, the Retort as Brew).
    fn first_seconds(self) -> u32 {
        match self {
            Self::Retort => 15,
            _ => 10,
        }
    }
}

/// Seconds one run takes: each step's own time less the ∛ of its sub-skill%
/// (0 to 4).
pub fn run_seconds(steps: &[Apparatus], skills: AlchemySkills) -> u32 {
    let step = |a: &Apparatus| a.first_seconds() - skills.sub_root(*a);
    steps.iter().map(step).sum::<u32>().max(1)
}

/// The same in action ticks.
pub fn run_ticks(steps: &[Apparatus], skills: AlchemySkills) -> u64 {
    u64::from(run_seconds(steps, skills)) * ACTION_TICKS_PER_SECOND as u64
}

/// Learns the sub-skill of every apparatus carried and not yet known, as
/// Collect Water is learned on a first empty bottle: whether any was new.
pub fn learn_sub_skills(carried: &Inventory, skills: &mut Skills) -> bool {
    let mut learned = false;
    for apparatus in Apparatus::ALL {
        let carried = apparatus
            .item()
            .is_some_and(|kind| carried.count_of(kind) > 0);
        let Some(id) = SkillId::find(apparatus.sub_skill()) else {
            continue;
        };
        if carried && !skills.knows(id) {
            skills.practise(id, 1);
            learned = true;
        }
    }
    learned
}
