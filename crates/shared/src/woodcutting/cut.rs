//! Cut Kindling (owner): which log is split, and how long it takes. The first
//! log carried, else the first on the ground, which counts only while the
//! inventory is open with the ground's storage shown (the caller's to say).

use super::WOODCUTTING_SKILL;
use crate::energy::ACTION_TICKS_PER_SECOND;
use crate::herb::{cube_root, skill_percent, skill_value};
use crate::id::Id;
use crate::item::{Inventory, Item, ItemKind};
use crate::skill::Skills;

/// Where the log to split is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LogAt {
    /// Carried: in the character's own grid or a bag.
    Carried(Id),
    /// On the ground where they stand.
    Ground(Id),
}

/// How long a log takes untrained, in seconds (Claude's default).
const FIRST_SECONDS: u32 = 20;

/// The log Cut Kindling splits: the first carried (the own grid top to bottom,
/// left to right, then bags), else the first of `ground`, given only while the
/// inventory is open with the ground's storage shown.
pub fn log_to_cut(carried: &Inventory, ground: Option<&[Item]>) -> Option<LogAt> {
    let is_log = |item: &Item| item.kind == ItemKind::Log;
    let own = (carried.items.iter())
        .filter(|placed| is_log(&placed.item))
        .min_by_key(|placed| (placed.y, placed.x))
        .map(|placed| placed.item.id);
    let in_bags = || {
        (carried.stacks_where(is_log))
            .find(|(_, own)| !own)
            .map(|(log, _)| log.id)
    };
    if let Some(id) = own.or_else(in_bags) {
        return Some(LogAt::Carried(id));
    }
    let ground = ground?.iter().find(|item| is_log(item))?;
    Some(LogAt::Ground(ground.id))
}

/// How long splitting one log takes, in whole seconds: 20 less the cube root of
/// Woodcutting's percent, so 20 down to 16 (as Collect Water's bottles).
pub fn cut_seconds(skills: &Skills) -> u32 {
    FIRST_SECONDS - cube_root(skill_percent(skill_value(skills, WOODCUTTING_SKILL)))
}

/// The same in action ticks.
pub fn cut_ticks(skills: &Skills) -> u64 {
    u64::from(cut_seconds(skills)) * ACTION_TICKS_PER_SECOND as u64
}
