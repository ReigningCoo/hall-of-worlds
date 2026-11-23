//! What the removing effects take away (magic plan PR 12): Poison reversed
//! cures poison (eaten, a fungus's begun, a spell's), Curse reversed lifts a
//! Curse (and, on the server, curses on what is worn), Illusion reversed
//! ends Illusions, and Warding reversed dispels every spell's effect, good
//! and bad.

use serde::{Deserialize, Serialize};

use crate::bonus::Source;

use super::lasting::{CURSED, ILLUSION, POISONED};

/// Which effects a removing outcome ends.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Ends {
    /// Poison reversed.
    Poison,
    /// Curse reversed.
    Curse,
    /// Illusion reversed.
    Illusion,
    /// Warding reversed: every spell's.
    Spells,
}

impl Ends {
    /// Whether it ends an effect from `source`.
    pub fn ends(self, source: Source) -> bool {
        match self {
            Self::Poison => matches!(
                source,
                Source::Poison | Source::FungusPoison(_) | Source::Spell(POISONED)
            ),
            Self::Curse => source == Source::Spell(CURSED),
            Self::Illusion => source == Source::Spell(ILLUSION),
            Self::Spells => matches!(source, Source::Spell(_)),
        }
    }
}

impl crate::sheet::Sheet {
    /// Ends every effect `ends` ends, keeping Health's and Mana's share.
    /// Whether any were.
    pub fn end_effects(
        &mut self,
        ends: Ends,
        situation: crate::sheet::Situation,
        now: u64,
    ) -> bool {
        if !self.effects.0.iter().any(|effect| ends.ends(effect.source)) {
            return false;
        }
        self.change_effects(situation, now, |effects| {
            effects.0.retain(|effect| !ends.ends(effect.source));
        });
        true
    }
}
