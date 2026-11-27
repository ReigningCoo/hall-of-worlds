//! The action bar: ten slots, each holding an active ability, a kind of item
//! used from there (food), a particular potion, a spell, or nothing, kept per character. Keys 1 to 0 use them,
//! and a slot with an ability can be set to use it by itself (`auto`). The
//! client changes it with `BarChange`s, which the server checks with the same
//! `ActionBar::apply`.

use serde::{Deserialize, Serialize};

use super::Ability;
use super::bar_slot::{BarSlot, BarThing, usable_from_bar};
use crate::item::ItemKind;
use crate::potion::PotionKey;
use crate::spell::{SpellKey, Spellbook};

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

/// What new characters' bars start with, beyond Attack, in the order it was
/// added: older bars are given each once. The Heal ability first, now the
/// Heal spell (magic plan PR 5); then Magic, so the spell design panel is
/// to hand (owner, 2026-11-27).
fn given() -> [BarThing; 2] {
    [BarThing::heal(), BarThing::Ability(Ability::Magic)]
}

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
    /// Put a particular potion in a slot, from a grid of items, replacing what
    /// was there (brewing decision 22). If it was already in another slot, it
    /// moves.
    PlacePotion { slot: u8, potion: PotionKey },
    /// Put a spell from the spellbook in a slot, replacing what was there
    /// (magic plan PR 5). If it was already in another slot, it moves.
    PlaceSpell { slot: u8, spell: SpellKey },
}

/// Why a change was refused.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BarRefusal {
    /// There is no such slot.
    NoSlot,
    /// Passive abilities can't go on the bar.
    Passive,
    /// The creature doesn't know it, or the spell isn't in its book.
    Unknown,
    /// There is nothing in the slot to make automatic.
    Empty,
    /// It can't be made automatic (`BarThing::can_be_automatic`).
    NotAutomatic,
    /// That kind of item can't be used from the bar (`usable_from_bar`), or
    /// the potion holds nothing.
    NotUsable,
}

/// A new character's: the Heal spell in the first slot and Attack in the
/// second, so with both automatic, healing comes first (Claude's suggestion,
/// 2026-11-07), then Magic, which opens the spell design panel.
impl Default for ActionBar {
    fn default() -> Self {
        let mut slots = [None; BAR_SLOTS];
        let things = [
            BarThing::heal(),
            BarThing::Ability(Ability::Attack),
            BarThing::Ability(Ability::Magic),
        ];
        for (slot, thing) in things.into_iter().enumerate() {
            slots[slot] = Some(BarSlot::new(thing));
        }
        Self {
            slots,
            given: given().len() as u8,
        }
    }
}

impl ActionBar {
    /// Gives a bar saved before some of `given` were added each of those, in
    /// the first empty slot (none if it is full, or already there). Also
    /// clears a thing found twice, keeping the first: a bar saved with both
    /// Grind and Brew loads both as Alchemy.
    pub fn catch_up(&mut self) {
        for slot in 1..BAR_SLOTS {
            let thing = self.slots[slot].map(|it| it.thing);
            if thing.is_some_and(|thing| {
                self.slots[..slot]
                    .iter()
                    .flatten()
                    .any(|it| it.thing == thing)
            }) {
                self.slots[slot] = None;
            }
        }
        let given = given();
        for &thing in given.iter().skip(usize::from(self.given)) {
            let empty = self.slots.iter().position(Option::is_none);
            if let (None, Some(slot)) = (self.slot_of(thing), empty) {
                self.slots[slot] = Some(BarSlot::new(thing));
            }
        }
        self.given = given.len() as u8;
    }

    /// Makes the change for a creature that knows `known` (`ability::known`)
    /// and keeps `book`, or says why not (and changes nothing). An ability it
    /// no longer knows (after shapeshifting) stays where it is, only it can't
    /// be used; so does a spell taken out of the book.
    pub fn apply(
        &mut self,
        change: BarChange,
        known: &[Ability],
        book: &Spellbook,
    ) -> Result<(), BarRefusal> {
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
            BarChange::PlacePotion { slot, potion } => {
                let slot = index(slot)?;
                if potion.is_empty() {
                    return Err(BarRefusal::NotUsable);
                }
                self.place(slot, BarThing::Potion(potion));
            }
            BarChange::PlaceSpell { slot, spell } => {
                let slot = index(slot)?;
                if book.by_key(spell).is_none() {
                    return Err(BarRefusal::Unknown);
                }
                self.place(slot, BarThing::Spell(spell));
            }
            BarChange::Clear { slot } => self.slots[index(slot)?] = None,
            BarChange::Swap { a, b } => {
                let (a, b) = (index(a)?, index(b)?);
                self.slots.swap(a, b);
            }
            BarChange::ToggleAuto { slot } => {
                let held = self.slots[index(slot)?].as_mut().ok_or(BarRefusal::Empty)?;
                if !held.auto && !held.thing.can_be_automatic(book) {
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

    /// The slot holding this ability, kind of item, potion or spell, if any.
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
