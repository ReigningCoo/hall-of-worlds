//! Missiles in flight (magic plan PR 6): for `ServerMessage::Missile`, told
//! to every player on the level they fly on, so each client can follow them.

use serde::{Deserialize, Serialize};

use crate::id::Id;
use crate::spell::Path;
use crate::spot::Spot;

/// One missile: whose spell launched it, and its number among that caster's.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MissileId {
    pub caster: Id,
    pub number: u32,
}

/// What became of a missile.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MissileNews {
    /// Launched just now along `path` (`Path::along_after` says where it is
    /// as time goes on). It lands at its end unless it meets someone first.
    Flying { id: MissileId, path: Path },
    /// It landed at `at`: on someone, or at the end of its way.
    Landed { id: MissileId, at: Spot },
}
