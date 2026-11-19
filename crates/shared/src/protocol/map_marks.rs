//! Personal map marks' messages (`shared::map_mark`).

use serde::{Deserialize, Serialize};

use crate::id::Id;
use crate::map_mark::{MapMark, MarkColour, MarkIcon};

/// A client asking about its character's marks on this planet.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum MarkAsk {
    /// Place a mark by hand at `at`, in wilderness cells (the server brings it
    /// round the planet and refuses one past the ice or over the limit):
    /// answered with `MarkNews::Added`, or nothing.
    Add {
        at: (i64, i64),
        icon: MarkIcon,
        colour: MarkColour,
    },
    /// Remove one of their marks: answered with `MarkNews::Removed`, or nothing.
    Remove(Id),
}

/// The server telling a client about its character's marks on this planet.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum MarkNews {
    /// Every mark, after `Welcome` (joining, or another planet).
    All(Vec<MapMark>),
    /// One more.
    Added(MapMark),
    /// One gone.
    Removed(Id),
}
