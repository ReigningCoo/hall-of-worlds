//! `Fireside`: what lies at a fireplace, the server's to keep: its log
//! basket's kindling, a place a stack, and its fire.

use serde::{Deserialize, Serialize};

use super::{Fireplace, KINDLING_PER_PLACE};
use crate::campfire::Campfire;
use crate::crafting_bench::NotTaken;
use crate::id::Id;
use crate::item::{Item, ItemKind};
use crate::material::Material;

/// What lies at a fireplace: a stack of kindling (or nothing) in each of its
/// basket's places (`Fireplace::basket_places`), and its fire of kindling,
/// burning as a campfire's (`campfire::Campfire`) but leaving no Ash by
/// burning (Ash comes from cooking on it, as on a hearth).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Fireside {
    pub fireplace: Fireplace,
    basket: Vec<Option<Item>>,
    /// Lit by a household for good, needing no kindling (for sites' own
    /// fireplaces later, as `crafting_bench::always_lit`).
    #[serde(default)]
    pub always_lit: bool,
    #[serde(default)]
    pub fire: Option<Campfire>,
}

impl Fireside {
    /// An empty basket, the fire unlit.
    pub fn new(fireplace: Fireplace) -> Self {
        Self {
            fireplace,
            basket: vec![None; fireplace.basket_places()],
            always_lit: false,
            fire: None,
        }
    }

    /// Each basket place, in order, empty or with its kindling.
    pub fn places(&self) -> &[Option<Item>] {
        &self.basket
    }

    /// The kindling in its basket, in its places' order.
    pub fn items(&self) -> impl Iterator<Item = &Item> {
        self.basket.iter().flatten()
    }

    /// Whether it burns at `now` (server seconds): always lit, or its fire
    /// still burning.
    pub fn is_lit(&self, now: u64) -> bool {
        self.always_lit || self.fire.as_ref().is_some_and(|fire| fire.is_lit(now))
    }

    /// Puts `item` in the basket's first free place: kindling, at most
    /// `KINDLING_PER_PLACE` pieces. Anything else gets the one `NotTaken`.
    pub fn put(&mut self, item: Item) -> Result<(), NotTaken> {
        if item.kind != ItemKind::Kindling || item.count() > KINDLING_PER_PLACE {
            return Err(NotTaken);
        }
        let free = self.basket.iter_mut().find(|place| place.is_none());
        *free.ok_or(NotTaken)? = Some(item);
        Ok(())
    }

    /// Takes the kindling `id` out of the basket, if it is in it.
    pub fn take(&mut self, id: Id) -> Option<Item> {
        let mut places = self.basket.iter_mut();
        let place = places.find(|place| place.as_ref().is_some_and(|item| item.id == id))?;
        place.take()
    }

    /// Lights its fire at `now` with a piece of `wood`'s kindling, or feeds
    /// it: the pieces it has left after, or `None` if it is always lit or its
    /// fire is full (`campfire::MAX_KINDLING`).
    pub fn burn(&mut self, now: u64, wood: Material) -> Option<usize> {
        if self.always_lit {
            return None;
        }
        if let Some(fire) = self.fire.as_mut().filter(|fire| fire.is_lit(now)) {
            fire.feed(now, wood).ok()?;
            return Some(fire.pieces_left(now));
        }
        let fire = Campfire::lit(now, wood);
        let pieces = fire.pieces_left(now);
        self.fire = Some(fire);
        Some(pieces)
    }
}
