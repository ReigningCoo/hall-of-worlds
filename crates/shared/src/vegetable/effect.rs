//! A vegetable's effect: a herb's kind of effect (`herb::Effect`) with when it
//! applies, raw, cooked or both (owner, 2026-11-17). Only raw and both apply
//! until cooking comes.

use serde::{Deserialize, Serialize};

use crate::herb::{self, Stat};

/// When an effect applies.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum When {
    Raw,
    Cooked,
    Both,
}

impl When {
    /// Whether it applies eaten raw.
    pub const fn raw(self) -> bool {
        matches!(self, Self::Raw | Self::Both)
    }

    /// Whether it applies cooked (later).
    pub const fn cooked(self) -> bool {
        matches!(self, Self::Cooked | Self::Both)
    }
}

/// One effect of eating a vegetable.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Effect {
    pub stat: Stat,
    /// -3 to 3, never 0.
    pub change: i8,
    pub when: When,
}

impl Effect {
    /// As a herb's effect, without when.
    pub const fn effect(self) -> herb::Effect {
        herb::Effect {
            stat: self.stat,
            change: self.change,
        }
    }
}
