//! `Doing`: what a mob is doing, kept with it and saved.

use serde::{Deserialize, Serialize};
use shared::id::Id;

/// What a mob is doing. Anything but `Idle` keeps it, and its level, going.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Doing {
    /// At rest, and needing no time until something wakes it.
    Idle,
    /// Going after someone, stopping this near.
    Going {
        after: Id,
        last_seen: (i64, i64),
        gap: i64,
    },
    /// Going after someone it saw take these stairs.
    Following {
        after: Id,
        stairs: (i64, i64),
        to_level: u32,
        arrives: (i64, i64),
        gap: i64,
    },
    /// Keeping away from someone it can see (`timid`, or fleeing by its
    /// morale: `choice`). `struck`: it has just struck at them, so steps away
    /// before it may again.
    Fleeing {
        from: Id,
        #[serde(default)]
        struck: bool,
    },
    /// Wandering from where it lost someone, so as not to wait there.
    Settling {
        steps_left: u8,
    },
    GoingHome {
        since: u64,
    },
    /// Standing between someone and its home, at its territory's edge.
    Guarding {
        against: Id,
        last_seen: (i64, i64),
    },
    /// Following someone's scent, stopping this near once it sees them, by marks
    /// left at or after `since`.
    Tracking {
        after: Id,
        gap: i64,
        since: u64,
    },
    /// Eating prey it has killed (`hunt.rs`), until this tick; then home.
    Eating {
        until: u64,
    },
    /// Going to look at where it heard a noise it can't see (`curious.rs`):
    /// walking from `since` until it can see the cell `at`, then looking until
    /// `looking` (`Some`), then home. Seeing anyone hands it to its choices.
    Investigating {
        at: (i64, i64),
        since: u64,
        #[serde(default)]
        looking: Option<u64>,
    },
    /// Going for a bag on the ground at `at` holding something it wants
    /// (`desire.rs`), from `since`; in reach, it takes from it and goes home.
    Taking {
        at: shared::spot::Spot,
        since: u64,
    },
    /// A timid one stepping away from a noise it heard but can't see
    /// (`curious.rs`), this many steps more, then home.
    Shying {
        from: (i64, i64),
        steps_left: u8,
    },
}
