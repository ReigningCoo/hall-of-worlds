//! The Character panel's Spells tab (magic plan PR 8): the spellbook, each
//! spell a row with its picture, name and Mana, favourites at the top. A
//! spell's picture is dragged onto the action bar like an ability on the
//! Abilities page (`drag.rs`), or clicked or tapped to cast it as its slot
//! would; Rename asks for a name (left empty, the automatic one), the star
//! marks a favourite, and Remove asks first. Every change is asked of the
//! server (`SpellbookAsk`), which answers with the sheet.

mod answer;
mod markup;
mod names;
mod show;

use bevy::prelude::*;
use shared::spell::BookSpell;
use shared::spell::Spell;

pub(super) use answer::answer_spellbook;
pub(super) use names::{name_spells, spell_id};
pub(super) use show::{show_spellbook, spawn_spellbook};

/// Where the page's ids are.
const FORM: &str = "spellbook";

/// The Spells tab: the page's document, the spells listed (a row's index is
/// its place here) with the Mana shown for each, and the spell being renamed
/// or whose removal is being asked about.
#[derive(Resource, Default)]
pub(super) struct SpellbookTab {
    /// The document the list was drawn into: a new one is drawn afresh.
    pub(super) document: Option<Entity>,
    pub(super) drawn_in: Option<Entity>,
    pub(super) listed: Vec<(BookSpell, u32)>,
    pub(super) renaming: Option<Spell>,
    /// What the name box says while renaming.
    pub(super) draft: String,
    pub(super) asking: Option<Spell>,
    /// The requesters as last shown, so a box is filled only on opening.
    pub(super) shown: (Option<Spell>, Option<Spell>),
}

#[cfg(test)]
mod tests;
