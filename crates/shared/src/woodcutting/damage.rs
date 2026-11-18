//! A tree's damage from felling cut short (moving or Esc): kept, so felling it
//! again goes on from there, until a day of world time after the last blow
//! (owner; whole until then, Claude's reading).

use serde::{Deserialize, Serialize};

use crate::herb::MINUTES_PER_DAY;

/// How long damage is kept after the last blow, in game minutes.
pub const DAMAGE_KEPT_MINUTES: u64 = MINUTES_PER_DAY;

/// Damage to one tree: how much of it is felled, per mille, and the game
/// minute of the last blow.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Damage {
    pub per_mille: u16,
    pub at: u64,
}

impl Damage {
    /// How much is left at game minute `now`: all of it within a day of the
    /// last blow, none after.
    pub fn left(self, now: u64) -> u16 {
        match now.saturating_sub(self.at) < DAMAGE_KEPT_MINUTES {
            true => self.per_mille,
            false => 0,
        }
    }

    /// The damage after felling for `ticks` at game minute `now`, of a tree
    /// that takes `whole` ticks to fell from nothing. 1000 is felled.
    pub fn chopped(self, now: u64, ticks: u64, whole: u64) -> Self {
        let done = (ticks * 1000).checked_div(whole).unwrap_or(1000);
        let per_mille = (u64::from(self.left(now)) + done).min(1000) as u16;
        Self { per_mille, at: now }
    }

    /// Whether the tree is down.
    pub fn is_felled(self) -> bool {
        self.per_mille >= 1000
    }

    /// The ticks still needed at game minute `now`, of `whole` from nothing.
    pub fn ticks_left(self, now: u64, whole: u64) -> u64 {
        let left = 1000 - u64::from(self.left(now).min(1000));
        (whole * left).div_ceil(1000)
    }
}
