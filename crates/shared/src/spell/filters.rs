//! `Filters`: Self, Friend and Enemy, limiting what a spell's first step can
//! hit. Free, and only in the first step (`check`).

use serde::{Deserialize, Serialize};

/// Which of the three filters a where step has. None set hits everything.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Filters {
    /// Self: the caster.
    pub caster: bool,
    /// Friends: other players until parties exist.
    pub friend: bool,
    /// Anything hostile to the caster.
    pub enemy: bool,
}

impl Filters {
    /// No filter: everything can be hit.
    pub const NONE: Self = Self {
        caster: false,
        friend: false,
        enemy: false,
    };

    /// Whether any filter is set.
    pub const fn any(self) -> bool {
        self.caster || self.friend || self.enemy
    }
}
