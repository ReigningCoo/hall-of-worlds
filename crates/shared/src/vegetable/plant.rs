//! A vegetable plant: one cell where a vegetable, pulse or grain grows (vegetable
//! plan PR 3), in a farm's field or wild. It grows as a herb patch does
//! (`herb::grown_stage`): up through the `herb::STAGES` at its `growth`, full
//! grown as long again, then back to the first; picking it starts it again
//! where its `regrow` says (dug up, it is gone for a whole growth). Only when
//! it was last picked needs keeping.

use serde::{Deserialize, Serialize};

use super::VegetableDef;
use crate::herb::{STAGES, grown_stage, growth_stage_minutes};

/// One cell's vegetable plant.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VegetablePlant {
    /// The vegetable's key (`VegetableDef::key`).
    pub vegetable: String,
    /// Where it starts in its round, in game minutes (any number: taken round
    /// its round's length). A field's plants share one, as they are sown
    /// together.
    pub phase: u32,
    /// The game minute it was last picked, if ever.
    #[serde(default)]
    pub picked: Option<u64>,
}

impl VegetablePlant {
    /// The stage it has grown to at game minute `now`: 1 to `herb::STAGES`, or
    /// 0 while it is gone (dug up).
    pub fn stage(&self, def: &VegetableDef, now: u64) -> u8 {
        grown_stage(def.growth, def.regrow, self.phase, self.picked, now)
    }

    /// Whether it can be picked at game minute `now`: full grown, and grown a
    /// stage since it was last picked (one picked comes back no further than
    /// full grown less a stage, so it isn't there to pick again at once).
    pub fn can_pick(&self, def: &VegetableDef, now: u64) -> bool {
        let regrown =
            (self.picked).is_none_or(|picked| now >= picked + growth_stage_minutes(def.growth));
        regrown && self.stage(def, now) == STAGES
    }
}
