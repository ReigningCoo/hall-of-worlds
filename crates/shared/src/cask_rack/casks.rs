//! `RackCasks`: the casks on a rack, cradle or stand, one a place, the
//! server's to keep; putting one on, taking one off, and their clocks.

use serde::{Deserialize, Serialize};

use super::{CaskRack, RACK_AGEING_PERCENT};
use crate::crafting_bench::NotTaken;
use crate::id::Id;
use crate::item::{Item, ItemKind};
use crate::item_form::ItemForm;

/// What lies on a cask rack: a place for each cask it holds
/// (`CaskRack::places`), each empty or holding a cask of its size.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RackCasks {
    pub rack: CaskRack,
    places: Vec<Option<Item>>,
}

impl RackCasks {
    /// An empty one.
    pub fn new(rack: CaskRack) -> Self {
        Self {
            rack,
            places: vec![None; rack.capacity()],
        }
    }

    /// Each place, in `CaskRack::places`' order, empty or with its cask.
    pub fn places(&self) -> &[Option<Item>] {
        &self.places
    }

    /// The casks on it, in their places' order.
    pub fn items(&self) -> impl Iterator<Item = &Item> {
        self.places.iter().flatten()
    }

    /// Whether it takes `item`: a cask of its size, while a place is free.
    pub fn takes(&self, item: &Item) -> bool {
        let size = match &item.form {
            Some(ItemForm::Cask(cask)) => Some(cask.size),
            _ => None,
        };
        item.kind == ItemKind::Cask
            && size == Some(self.rack.size)
            && self.places.iter().any(Option::is_none)
    }

    /// Lays `item` in the first free place, lowest first (nothing is ever
    /// swapped off). Anything else gets the one `NotTaken`, never saying why
    /// (as benches). Its clocks are the caller's to lay down.
    pub fn put(&mut self, item: Item) -> Result<(), NotTaken> {
        if !self.takes(&item) {
            return Err(NotTaken);
        }
        let free = self.places.iter_mut().find(|place| place.is_none());
        let Some(free) = free else {
            return Err(NotTaken);
        };
        *free = Some(item);
        Ok(())
    }

    /// Takes the cask `id` off, if it is on it.
    pub fn take(&mut self, id: Id) -> Option<Item> {
        let mut places = self.places.iter_mut();
        let place = places.find(|place| place.as_ref().is_some_and(|item| item.id == id))?;
        place.take()
    }

    /// Puts `item` back in the place of the one with its id (poured from at
    /// a bar). Whether it was there.
    pub fn replace(&mut self, item: Item) -> bool {
        let mut places = self.places.iter_mut().flatten();
        let Some(place) = places.find(|place| place.id == item.id) else {
            return false;
        };
        *place = item;
        true
    }

    /// Lets the real time pass until `now` (seconds since 1970) on its casks
    /// (laid down when put on, `Item::lay_casks_down`), sealed drinks ageing
    /// `RACK_AGEING_PERCENT` as fast. Whether anything a player sees changed.
    pub fn settle(&mut self, now: u64) -> bool {
        let mut seen = false;
        for item in self.places.iter_mut().flatten() {
            seen |= (item.settle_lying_casks_ageing(now, RACK_AGEING_PERCENT)).seen;
        }
        seen
    }
}
