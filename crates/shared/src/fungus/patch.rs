//! A fungus patch: one cell where a fungus grows, growing as a herb patch does
//! (`herb::grown_stage`): up through the `herb::STAGES` at its `growth`, full
//! grown as long again, then back to the first; picking starts it again where
//! its `regrow` says (dug up, it is gone for a whole growth). Only when it was
//! last picked needs keeping. Picked full grown only, as fungi have no early
//! harvest; how many by Foraging, as herbs' (`harvest`).

use serde::{Deserialize, Serialize};

use super::{FungusDef, Unit};
use crate::herb::{STAGES, grown_stage, growth_stage_minutes, harvest_range};
use crate::rng::Rng;

/// One cell's fungus.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FungusPatch {
    /// The fungus's key (`FungusDef::key`).
    pub fungus: String,
    /// Where it starts in its round, in game minutes (any number: taken round
    /// its round's length).
    pub phase: u32,
    /// The game minute it was last picked, if ever.
    #[serde(default)]
    pub picked: Option<u64>,
}

impl FungusPatch {
    /// The stage it has grown to at game minute `now`: 1 to `herb::STAGES`, or
    /// 0 while it is gone (dug up).
    pub fn stage(&self, def: &FungusDef, now: u64) -> u8 {
        grown_stage(def.growth, def.regrow, self.phase, self.picked, now)
    }

    /// Whether it can be picked at game minute `now`: full grown, and grown a
    /// stage since it was last picked.
    pub fn can_pick(&self, def: &FungusDef, now: u64) -> bool {
        let regrown =
            (self.picked).is_none_or(|picked| now >= picked + growth_stage_minutes(def.growth));
        regrown && self.stage(def, now) == STAGES
    }
}

/// How many items picking a full-grown patch of `def` gives someone with this
/// much Foraging (`herb::skill_value`): as many as a herb patch without Herbs
/// (`herb::harvest_range`, 1 to 3 untrained), but a single large piece for
/// `Unit::Piece` (a giant puffball is one).
pub fn harvest(def: &FungusDef, foraging: u32, rng: &mut Rng) -> u32 {
    if def.unit == Unit::Piece {
        return 1;
    }
    let (least, most) = harvest_range(foraging, 0);
    rng.range_u64(u64::from(least), u64::from(most) + 1) as u32
}
