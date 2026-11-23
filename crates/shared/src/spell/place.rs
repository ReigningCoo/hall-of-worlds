//! `Where`: how a step of a spell reaches what it acts on. Five, unlocked in
//! this order (`unlocked_wheres`), each its own skill under Form.

use serde::{Deserialize, Serialize};

use crate::skill::SkillId;

/// How a where step reaches what it hits (the magic plan's table of wheres).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Where {
    /// Touching distance in the direction faced.
    Touch,
    /// Flies to a fixed spot, hitting the first creature in its way.
    Missile,
    /// A ring round its centre, not the centre itself.
    Surround,
    /// A line to the target and everything along it.
    Beam,
    /// Exactly the target, instantly.
    Point,
}

impl Where {
    /// Every where, in the order they unlock.
    pub const ALL: [Self; 5] = [
        Self::Touch,
        Self::Missile,
        Self::Surround,
        Self::Beam,
        Self::Point,
    ];

    /// Its tier, 1 to 5: its place in the unlock order, and its base cost.
    pub const fn tier(self) -> u32 {
        self as u32 + 1
    }

    /// Its key, as in skill paths and language files.
    pub const fn key(self) -> &'static str {
        match self {
            Self::Touch => "touch",
            Self::Missile => "missile",
            Self::Surround => "surround",
            Self::Beam => "beam",
            Self::Point => "point",
        }
    }

    /// The Form skill it practises and is cheaper by.
    pub fn skill(self) -> SkillId {
        SkillId::find(&format!("magic/form/{}", self.key())).expect("Form has every where")
    }
}
