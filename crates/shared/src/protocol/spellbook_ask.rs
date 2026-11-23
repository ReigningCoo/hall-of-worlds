//! Changing the spellbook (`shared::spell::Spellbook`, magic plan PR 4).

use serde::{Deserialize, Serialize};

use crate::spell::Spell;

/// A client asking to change its spellbook, at once (it takes no time).
/// Answered with `Sheet` (the book is on it), whether or not it could be done;
/// refusals are only logged, as the client checks the same rules first.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpellbookAsk {
    /// Keep a design, checked against the caster's skills (`spell::check`),
    /// under the player's own name or the automatic one (`None`).
    Store { spell: Spell, name: Option<String> },
    /// Give a spell in the book the player's own name, or the automatic one.
    Rename { spell: Spell, name: Option<String> },
    /// Mark a spell a favourite, or not.
    Favourite { spell: Spell, favourite: bool },
    /// Take a spell out of the book.
    Remove { spell: Spell },
}
