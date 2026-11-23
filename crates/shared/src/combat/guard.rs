//! `Guard`: what spells lying on a fighter do to blows (magic plan PR 12),
//! per mille: Illusion makes it harder to hit (more Defence), Curse makes its
//! own blows weaker, Stoneskin makes blows on it weaker.

use crate::bonus::{Stat, Totals};

/// Per mille changes to a fighter's blows from what lies on it: 1000 is none.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Guard {
    /// Its Defence power (`Stat::Defence`).
    pub defence: u64,
    /// The harm its own blows do (`Stat::BlowHarm`).
    pub blows_done: u64,
    /// The harm blows do to it (`Stat::HarmTaken`).
    pub blows_taken: u64,
}

impl Guard {
    /// Nothing changed.
    pub const NONE: Self = Self {
        defence: 1000,
        blows_done: 1000,
        blows_taken: 1000,
    };

    /// From every bonus lying on a fighter now.
    pub fn of(totals: &Totals) -> Self {
        Self {
            defence: totals.per_mille(Stat::Defence),
            blows_done: totals.per_mille(Stat::BlowHarm),
            blows_taken: totals.per_mille(Stat::HarmTaken),
        }
    }

    /// `harm` (in thousandths) dealt by `self`'s blow to `defender`: less if
    /// it is cursed or `defender` has stoneskin.
    pub fn blow(self, defender: Self, harm: u32) -> u32 {
        let harm = u64::from(harm) * self.blows_done / 1000 * defender.blows_taken / 1000;
        harm.min(u64::from(u32::MAX)) as u32
    }
}

impl Default for Guard {
    fn default() -> Self {
        Self::NONE
    }
}
