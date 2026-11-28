//! `Ways`: how a mob's race gets about (`shared::behaviour`), kept on the mob
//! for its steps: fences, fleeing straight or zigzag, fear of open fire.

use shared::behaviour::{BehaviourDef, Fences, Flee};

/// How a mob gets about, from its race's behaviour. Not saved: set again from
/// the server's behaviours every tick (`run_mobs`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct Ways {
    pub(crate) fences: Fences,
    pub(crate) flee: Flee,
    pub(crate) fears_fire: bool,
    /// Which side a zigzag goes next.
    pub(crate) zig: bool,
}

impl Ways {
    pub(crate) fn of(def: &BehaviourDef) -> Self {
        Self {
            fences: def.fences,
            flee: def.flee,
            fears_fire: def.fears_fire,
            zig: false,
        }
    }

    /// Takes `def`'s, keeping which side a zigzag goes next.
    pub(crate) fn set(&mut self, def: &BehaviourDef) {
        *self = Self {
            zig: self.zig,
            ..Self::of(def)
        };
    }

    /// Whether fences and gates stop it now: always for `Never`, and for
    /// `Seldom` unless it is running for its life.
    pub(crate) fn stopped_by_fences(&self, fleeing: bool) -> bool {
        match self.fences {
            Fences::Through => false,
            Fences::Seldom => !fleeing,
            Fences::Never => true,
        }
    }
}
