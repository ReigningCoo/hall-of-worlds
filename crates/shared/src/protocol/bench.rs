//! Furniture with places' messages (crafting benches plan PR 2, generalised
//! by the tavern furniture plan PR 1): putting things on a crafting bench or
//! cask rack and taking them off, what the server tells of what lies on each,
//! and the developer tool that makes a bench.

use serde::{Deserialize, Serialize};

use crate::furniture::{Held, Piece};
use crate::id::Id;
use crate::item::{ItemKind, Quality};

/// About a piece with places (`furniture::Holder`: a crafting bench, a cask
/// rack) at `bench`, any cell of it on the player's level, which must be
/// within the player's reach (`ClientMessage::Bench`). Answered with
/// `Inventory`, and `BenchNews::Tools` for everyone who has the piece, or
/// `BenchNews::Refused`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BenchAsk {
    /// Put this carried item on it; a bench's tool of the same kind there
    /// comes back to the player (a swap).
    Put { bench: (i64, i64), item: Id },
    /// Take this tool (or cask) off it, into what is carried (or onto the ground if it
    /// won't fit).
    Take { bench: (i64, i64), item: Id },
    /// Developer tools: make a crafting bench (`DevBench`).
    Dev(DevBench),
}

/// About crafting benches (`ServerMessage::Bench`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BenchNews {
    /// What lies on pieces with places on the player's level as it is now: a
    /// piece's once its cells have been sent on a visit, and again whenever
    /// it changes. Each replaces what the client had for that piece.
    Tools(Vec<SeenBench>),
    /// A piece wouldn't take what was put on it (`BenchAsk::Put`). Never why,
    /// so trying things doesn't tell which kinds a craft takes (owner's).
    Refused,
}

/// What lies on a piece with places as it is now (`BenchNews::Tools`): the
/// piece at `at` (`Piece::at`) on the player's level.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SeenBench {
    pub at: (i64, i64),
    pub held: Held,
    /// Whether a hearth gives heat now (`BenchTools::is_lit` by the server's
    /// clock); `false` for other benches.
    pub lit: bool,
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
