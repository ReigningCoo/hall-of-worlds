//! Which spell the pointer is on: a spell in the book drawn as a picture (an
//! action bar slot, the Spells tab), or the spell design panel's preview.

use bevy::prelude::*;
use shared::ability::BarThing;
use shared::spell::{Spell, SpellKey, Spellbook};

use crate::dom::{AbilityCell, AbilitySlotShown, Hit};
use crate::spell_design::PreviewSpell;
use crate::widgets::UiId;

/// The preview's id (`spell_design`'s markup).
const PREVIEW: UiId = UiId::new("spell_design", "preview");

/// What a spell picture is of, telling one popup from another.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum Subject {
    /// A spell in the book, on the action bar with this key's short name (or
    /// none, elsewhere).
    Book(SpellKey, String),
    /// The design panel's preview.
    Preview,
}

/// What the pointer is on, if it is a spell's picture.
pub(super) fn pointed(
    hit: &Hit,
    cells: &Query<&AbilityCell>,
    slots: &Query<&AbilitySlotShown>,
    ids: &Query<&UiId>,
) -> Option<Subject> {
    hit.first(|entity| {
        if ids.get(entity).is_ok_and(|id| *id == PREVIEW) {
            return Some(Subject::Preview);
        }
        let cell = cells.get(entity).ok()?;
        let BarThing::Spell(key) = cell.thing else {
            return None;
        };
        let label = slots
            .get(cell.of)
            .map_or_else(|_| String::new(), |slot| slot.key.clone());
        Some(Subject::Book(key, label))
    })
}

/// The spell a subject is, with the player's own name for it if any; none
/// for one no longer in the book (its slot keeps its plain tip).
pub(super) fn spell_of(
    subject: &Subject,
    book: &Spellbook,
    preview: &PreviewSpell,
) -> Option<(Spell, Option<String>)> {
    match subject {
        Subject::Book(key, _) => {
            (book.by_key(*key)).map(|kept| (kept.spell.clone(), kept.name.clone()))
        }
        Subject::Preview => preview.0.clone(),
    }
}
