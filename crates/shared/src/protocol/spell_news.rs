//! Spells happening on a level (magic plan PR 7): for `ServerMessage::Spell`,
//! told to every player on the level so each client can draw them. Missiles
//! in flight have their own news (`MissileNews`).

use serde::{Deserialize, Serialize};

use crate::spell::{Look, Shown};
use crate::spot::Spot;

/// What a spell just did, to draw.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpellNews {
    /// A where step (not a missile) happened, as it looked.
    Where(Shown),
    /// An effect step landed on whoever stands at these spots.
    Landed { on: Vec<Spot>, look: Look },
}
