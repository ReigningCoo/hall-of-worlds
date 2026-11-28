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
}
