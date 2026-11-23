//! What is at a point of the interface, for abilities: a slot of the bar, an
//! ability on the Abilities page, a spell on the Spells page, elsewhere on
//! those pages, or elsewhere on the Character panel.

use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use shared::ability::{Ability, BarThing};
use shared::spell::SpellKey;

use super::bar::SlotNode;
use super::page::PageAbility;
use crate::common::{AbilitiesPage, SpellsPage};
use crate::dom::{AbilityCell, Finder};
use crate::widgets::UiId;

/// Where something can be picked up or dropped.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Spot {
    /// A slot of the action bar.
    Slot(u8),
    /// An ability on the Abilities page.
    Listed(Ability),
    /// A spell's picture on the Spells page.
    Spell(SpellKey),
    /// Anywhere else on the Abilities or Spells page.
    Page,
    /// Anywhere else on the Character panel (its other pages and tabs).
    Panel,
}

/// The Abilities page or the Spells page.
type EitherPage = Or<(With<AbilitiesPage>, With<SpellsPage>)>;

/// Finds the spot at a point: whatever is in front there (`dom`'s one
/// search), or holds it.
#[derive(SystemParam)]
pub(super) struct Spots<'w, 's> {
    finder: Finder<'w, 's>,
    parents: Query<'w, 's, &'static ChildOf>,
    slots: Query<'w, 's, &'static SlotNode>,
    listed: Query<'w, 's, &'static PageAbility>,
    pages: Query<'w, 's, (), EitherPage>,
    spell_pages: Query<'w, 's, (), With<SpellsPage>>,
    cells: Query<'w, 's, &'static AbilityCell>,
    ids: Query<'w, 's, &'static UiId>,
}

impl Spots<'_, '_> {
    /// The spot at `at` (in interface pixels), if there is one.
    pub(super) fn at(&self, at: Vec2) -> Option<Spot> {
        self.of(self.finder.target_at(at)?)
    }

    /// The spot this node, in front somewhere, is or is in: a spell's
    /// picture only on the Spells page (a spell in a slot is the slot's).
    pub(super) fn of(&self, front: Entity) -> Option<Spot> {
        let mut spell = None;
        std::iter::once(front)
            .chain(self.parents.iter_ancestors(front))
            .find_map(|entity| {
                if let Ok(AbilityCell {
                    thing: BarThing::Spell(key),
                    ..
                }) = self.cells.get(entity)
                {
                    spell = Some(*key);
                    None
                } else if let (Some(key), true) = (spell, self.spell_pages.contains(entity)) {
                    Some(Spot::Spell(key))
                } else if let Ok(slot) = self.slots.get(entity) {
                    Some(Spot::Slot(slot.0))
                } else if let Ok(listed) = self.listed.get(entity) {
                    Some(Spot::Listed(listed.0))
                } else if self.pages.contains(entity) {
                    Some(Spot::Page)
                } else if self
                    .ids
                    .get(entity)
                    .is_ok_and(|id| id.parts() == ("inventory", "panel"))
                {
                    // The Character panel (`inventory`'s).
                    Some(Spot::Panel)
                } else {
                    None
                }
            })
    }
}
