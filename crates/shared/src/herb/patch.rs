//! A herb patch: one cell where a herb grows, and the stage it has grown to at
//! any time. Patches grow by the world's clock on their own: up through the
//! `STAGES` at their herb's speed, full grown for as long again, then back to
//! the first (it seeds and dies back). Each starts somewhere of its own in
//! that round (`phase`), and harvesting starts it again from the stage its
//! herb's `Regrow` says, so only the time it was last picked needs keeping.

use serde::{Deserialize, Serialize};

use super::{EARLY_STAGE, HerbDef, STAGES};

/// Game minutes in a game day.
pub const MINUTES_PER_DAY: u64 = 24 * 60;

/// One cell's herb.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HerbPatch {
    /// The herb's key (`HerbDef::key`).
    pub herb: String,
    /// Where it starts in its round, in game minutes (any number: taken
    /// round its round's length).
    pub phase: u32,
    /// The game minute it was last harvested, if ever.
    #[serde(default)]
    pub picked: Option<u64>,
}

/// Game minutes a herb takes to grow one stage.
pub fn stage_minutes(def: &HerbDef) -> u64 {
    u64::from(def.growth.days()) * MINUTES_PER_DAY / u64::from(STAGES)
}

impl HerbPatch {
    /// The stage it has grown to at game minute `now`: 1 to `STAGES`, or 0
    /// while it is gone (dug up).
    pub fn stage(&self, def: &HerbDef, now: u64) -> u8 {
        let step = stage_minutes(def);
        let growing = step * u64::from(STAGES);
        let round = 2 * growing;
        let at = match self.picked {
            None => now + u64::from(self.phase),
            Some(picked) => match def.regrow.back_to() {
                // Gone for as long as it takes to grow, then from the start.
                None if now < picked + growing => return 0,
                None => now - picked - growing,
                Some(back_to) => u64::from(back_to - 1) * step + now.saturating_sub(picked),
            },
        };
        let at = at % round;
        if at >= growing {
            STAGES
        } else {
            1 + (at / step) as u8
        }
    }

    /// Whether it can be harvested at game minute `now`: grown to
    /// `EARLY_STAGE` or more, and grown a stage since it was last harvested
    /// (picked fruit comes back at `EARLY_STAGE`, and isn't there to pick
    /// again at once).
    pub fn can_pick(&self, def: &HerbDef, now: u64) -> bool {
        let regrown = self
            .picked
            .is_none_or(|picked| now >= picked + stage_minutes(def));
        regrown && self.stage(def, now) >= EARLY_STAGE
    }
}

#[cfg(test)]
mod tests;
