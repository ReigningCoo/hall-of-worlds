//! A felled tree growing back: its stages, what walking through each costs, and
//! when each comes, on world time (game minutes, as plants' growth).

use serde::{Deserialize, Serialize};

use crate::herb::MINUTES_PER_DAY;

/// How far a felled tree has grown back, from its stump to full grown.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum TreeStage {
    /// What is left the first day after felling.
    Stump,
    Sapling,
    Young,
    HalfGrown,
    /// As it was before felling: the seed's tree again.
    Grown,
}

/// The stump is gone a day after felling (owner).
pub const STUMP_MINUTES: u64 = MINUTES_PER_DAY;

/// The tree grows again from 5 days after felling (owner).
pub const REGROW_MINUTES: u64 = 5 * MINUTES_PER_DAY;

/// About a week a stage (owner).
pub const STAGE_MINUTES: u64 = 7 * MINUTES_PER_DAY;

/// A move over a stump or a young tree costs this much more (owner): 1.5×.
pub const STUMP_COST_PER_MILLE: u32 = 1500;

/// A move through a half-grown tree: 3× (owner).
pub const HALF_GROWN_COST_PER_MILLE: u32 = 3000;

impl TreeStage {
    /// What is there `minutes` after felling: a stump the first day, nothing
    /// until day 5, then a sapling, a young tree and a half-grown one a week
    /// each, then the full-grown tree (whose saved entry can go).
    pub fn after_felling(minutes: u64) -> Option<Self> {
        if minutes < STUMP_MINUTES {
            return Some(Self::Stump);
        }
        let growing = minutes.checked_sub(REGROW_MINUTES)?;
        Some(match growing / STAGE_MINUTES {
            0 => Self::Sapling,
            1 => Self::Young,
            2 => Self::HalfGrown,
            _ => Self::Grown,
        })
    }

    /// How much a move over it costs against open ground, per mille (as
    /// furniture's, `dungeon::Level::furniture_cost_per_mille`): `None` where
    /// it blocks, a grown tree.
    pub const fn cost_per_mille(self) -> Option<u32> {
        match self {
            Self::Sapling => Some(1000),
            Self::Stump | Self::Young => Some(STUMP_COST_PER_MILLE),
            Self::HalfGrown => Some(HALF_GROWN_COST_PER_MILLE),
            Self::Grown => None,
        }
    }

    /// Whether it can be felled: a young tree or bigger.
    pub const fn can_fell(self) -> bool {
        matches!(self, Self::Young | Self::HalfGrown | Self::Grown)
    }

    /// How many logs felling it gives, from a grown tree's `grown` logs: a
    /// quarter of them young, half half-grown (at least 1 each), none from a
    /// stump or a sapling.
    pub const fn logs(self, grown: u8) -> u8 {
        let logs = match self {
            Self::Stump | Self::Sapling => return 0,
            Self::Young => (grown + 2) / 4,
            Self::HalfGrown => grown.div_ceil(2),
            Self::Grown => grown,
        };
        if logs == 0 && grown > 0 { 1 } else { logs }
    }
}
