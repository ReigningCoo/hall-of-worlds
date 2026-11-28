//! Critical hits (mob behaviour plan PR 7): for now only sneak attacks, a blow
//! landing on a mob caught off guard (`alertness::Wariness::off_guard`:
//! Unaware, or Alert but not yet turned to what it heard) is always one. Kept as its own rule so later critical hits (chances by skill,
//! weak spots) build on it.

use crate::vitals::Points;

/// How many times its harm a critical hit does.
pub const CRITICAL_TIMES: u32 = 2;

/// Why a blow is a critical hit, if it is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Critical {
    /// It landed on someone caught off guard.
    Unaware,
}

impl Critical {
    /// A blow on someone `unaware` (caught off guard): always a critical.
    pub const fn of_blow(unaware: bool) -> Option<Self> {
        match unaware {
            true => Some(Self::Unaware),
            false => None,
        }
    }

    /// The harm a critical does instead of `harm`.
    pub const fn harm(self, harm: Points) -> Points {
        Points(harm.0.saturating_mul(CRITICAL_TIMES))
    }
}

/// `harm` as it lands, critical or not.
pub const fn critical_harm(harm: Points, critical: Option<Critical>) -> Points {
    match critical {
        Some(critical) => critical.harm(harm),
        None => harm,
    }
}
