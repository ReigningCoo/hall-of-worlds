//! `Temper`: how a race takes to those it doesn't know, before anything has
//! happened (from the creature research, `docs/context/research-creatures.md`).
//! Each race's is in `behaviours.ron`; `CreatureKind::temper` reads the game's own.

use serde::{Deserialize, Serialize};

use crate::creature::CreatureKind;

use super::Behaviours;

/// How a race takes to those it doesn't know, before anything has happened.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Temper {
    /// Comes over to say hello.
    Friendly,
    /// Leaves others alone. The research's "varies" (humans, giants) is this too,
    /// until creatures of one kind can differ.
    Neutral,
    /// Keeps away from anyone it doesn't know.
    Timid,
    /// Minds its own patch: guarding it comes later, and until then it is `Neutral`.
    Territorial,
    /// Dislikes everyone not its own kind.
    Hostile,
}

impl CreatureKind {
    /// How it takes to those it doesn't know: the game's own behaviour data's
    /// (a server's mods never change it, so client and server agree).
    pub fn temper(self) -> Temper {
        Behaviours::built_in_ref().of(self).temper
    }
}
