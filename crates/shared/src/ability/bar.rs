//! The action bar: ten slots, each holding an active ability, a kind of item
//! used from there (food), or nothing, kept per character. Keys 1 to 0 use them,
//! and a slot with an ability can be set to use it by itself (`auto`). The
//! client changes it with `BarChange`s, which the server checks with the same
//! `ActionBar::apply`.

use serde::{Deserialize, Serialize};

use super::Ability;
use super::bar_slot::{BarSlot, BarThing, usable_from_bar};
use crate::item::ItemKind;

/// How many slots the bar has.
pub const BAR_SLOTS: usize = 10;

/// A character's action bar. Automatic slots take turns from the first to the
/// last: the first whose ability can be done is (owner, 2026-11-07), so their
/// order is the player's to choose.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionBar {
    pub slots: [Option<BarSlot>; BAR_SLOTS],
    /// How many of `GIVEN` it has been given: a bar saved before one was added
    /// gets it once (`catch_up`), and keeps it off if the player takes it off.
    #[serde(default)]
    pub given: u8,
}

/// Abilities new characters' bars start with, beyond Attack, in the order they
/// were added: older bars are given each once.
const GIVEN: [Ability; 1] = [Ability::Heal];

/// A change the player asks for.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BarChange {
    /// Put an ability in a slot, from the Abilities page, replacing what was there.
    /// If it was already in another slot, it moves, keeping `auto`.
    Place { slot: u8, ability: Ability },
    /// Take the ability out of a slot (dropped on the Abilities page).
    Clear { slot: u8 },
    /// Swap two slots, each ability keeping its `auto`.
    Swap { a: u8, b: u8 },
    /// Turn a slot's `auto` on or off.
    ToggleAuto { slot: u8 },
    /// Put a kind of item in a slot, from a grid of items, replacing what was
    /// there. If it was already in another slot, it moves.
    PlaceItem { slot: u8, item: ItemKind },
}

/// Why a change was refused.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BarRefusal {
    /// There is no such slot.
    NoSlot,
    /// Passive abilities can't go on the bar.
    Passive,
    /// The creature doesn't know it.
    Unknown,
    /// There is nothing in the slot to make automatic.
    Empty,
    /// It can't be made automatic (`BarThing::can_be_automatic`).
    NotAutomatic,
    /// That kind of item can't be used from the bar (`usable_from_bar`).
    NotUsable,
}

/// A new character's: Heal in the first slot and Attack in the second, so
/// with both automatic, healing comes first (Claude's suggestion, 2026-11-07).
impl Default for ActionBar {
    fn default() -> Self {
        let mut slots = [None; BAR_SLOTS];
        for (slot, ability) in [Ability::Heal, Ability::Attack].into_iter().enumerate() {
            slots[slot] = Some(BarSlot::new(ability));
        }
        Self {
            slots,
            given: GIVEN.len() as u8,
        }
    }
}

impl ActionBar {
    /// Gives a bar saved before some of `GIVEN` were added each of those, in
    /// the first empty slot (none if it is full, or already there).
    pub fn catch_up(&mut self) {
        for &ability in GIVEN.iter().skip(usize::from(self.given)) {
            let empty = self.slots.iter().position(Option::is_none);
            if let (None, Some(slot)) = (self.slot_of(ability), empty) {
                self.slots[slot] = Some(BarSlot::new(ability));
            }
        }
        self.given = GIVEN.len() as u8;
    }

    /// Makes the change for a creature that knows `known` (`ability::known`), or
    /// says why not (and changes nothing). An ability it no longer knows (after
    /// shapeshifting) stays where it is, only it can't be used.
    pub fn apply(&mut self, change: BarChange, known: &[Ability]) -> Result<(), BarRefusal> {
        match change {
            BarChange::Place { slot, ability } => {
                let slot = index(slot)?;
                if !ability.is_active() {
                    return Err(BarRefusal::Passive);
                }
                if !known.contains(&ability) {
                    return Err(BarRefusal::Unknown);
                }
                self.place(slot, BarThing::Ability(ability));
            }
            BarChange::PlaceItem { slot, item } => {
                let slot = index(slot)?;
                if !usable_from_bar(item) {
                    return Err(BarRefusal::NotUsable);
                }
                self.place(slot, BarThing::Item(item));
            }
            BarChange::Clear { slot } => self.slots[index(slot)?] = None,
            BarChange::Swap { a, b } => {
                let (a, b) = (index(a)?, index(b)?);
                self.slots.swap(a, b);
            }
            BarChange::ToggleAuto { slot } => {
                let held = self.slots[index(slot)?].as_mut().ok_or(BarRefusal::Empty)?;
                if !held.auto && !held.thing.can_be_automatic() {
                    return Err(BarRefusal::NotAutomatic);
                }
                held.auto = !held.auto;
            }
        }
        Ok(())
    }

    /// Puts this in a slot, moving it with its `auto` if it is already on the bar.
    fn place(&mut self, slot: usize, thing: BarThing) {
        let auto = match self.slot_of(thing) {
            Some(old) => self.slots[old].take().is_some_and(|old| old.auto),
            None => false,
        };
        self.slots[slot] = Some(BarSlot { thing, auto });
    }

    /// The slot holding this ability or kind of item, if any.
    pub fn slot_of(&self, thing: impl Into<BarThing>) -> Option<usize> {
        let thing = thing.into();
        self.slots
            .iter()
            .position(|held| held.is_some_and(|held| held.thing == thing))
    }
}

/// A slot's place, if there is such a slot.
fn index(slot: u8) -> Result<usize, BarRefusal> {
    let slot = slot as usize;
    if slot < BAR_SLOTS {
        Ok(slot)
    } else {
        Err(BarRefusal::NoSlot)
    }
}
