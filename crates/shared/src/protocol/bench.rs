//! Crafting benches' messages (crafting benches plan PR 2): putting tools on
//! a bench and taking them off, what the server tells of each bench's tools,
//! and the developer tool that makes a bench.

use serde::{Deserialize, Serialize};

use crate::crafting_bench::BenchTools;
use crate::furniture::Piece;
use crate::id::Id;
use crate::item::{ItemKind, Quality};

/// About a crafting bench at `bench`, any cell of it on the player's level,
/// which must be within the player's reach (`ClientMessage::Bench`). Answered
/// with `Inventory`, and `BenchNews::Tools` for everyone who has the bench, or
/// `BenchNews::Refused`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BenchAsk {
    /// Put this carried item on it; a tool of the same kind there comes back
    /// to the player (a swap).
    Put { bench: (i64, i64), item: Id },
    /// Take this tool off it, into what is carried (or onto the ground if it
    /// won't fit).
    Take { bench: (i64, i64), item: Id },
    /// Developer tools: make a crafting bench (`DevBench`).
    Dev(DevBench),
}

/// About crafting benches (`ServerMessage::Bench`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BenchNews {
    /// Benches' tools on the player's level as they are now: a bench's once
    /// its cells have been sent on a visit, and again whenever they change.
    /// Each replaces what the client had for that bench.
    Tools(Vec<SeenBench>),
    /// A bench wouldn't take what was put on it (`BenchAsk::Put`). Never why,
    /// so trying things doesn't tell which kinds a craft takes (owner's).
    Refused,
}

/// A crafting bench's tools as they are now (`BenchNews::Tools`): the
/// bench's piece at `at` (`Piece::at`) on the player's level.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SeenBench {
    pub at: (i64, i64),
    pub tools: BenchTools,
}

/// Developer tools (Create Crafting Bench): a bench for the server to put on
/// the player's site level and keep, with these tools made new
/// (`BenchAsk::Dev`). `piece` must be a `CraftingBench` furnishing
/// could make, every cell on free floor (no wall, stairs or other furniture).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DevBench {
    pub piece: Piece,
    /// Each tool and its quality; a kind left out is none.
    pub tools: Vec<(ItemKind, Quality)>,
}
