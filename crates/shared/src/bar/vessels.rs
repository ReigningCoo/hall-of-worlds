//! `BarVessels`: the drinking vessels on a bar counter, one a place, the
//! server's to keep; which it takes, putting one on, taking one off, putting
//! one back changed after pouring, and their clocks.

use serde::{Deserialize, Serialize};

use super::BarCounter;
use crate::crafting_bench::NotTaken;
use crate::id::Id;
use crate::item::{Item, ItemKind};
use crate::vessel::VesselKind;

/// What lies on a bar counter: a place for each vessel it holds
/// (`BarCounter::places`), each empty or holding one.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BarVessels {
    pub counter: BarCounter,
    places: Vec<Option<Item>>,
}

impl BarVessels {
    /// An empty one.
    pub fn new(counter: BarCounter) -> Self {
        Self {
            counter,
            places: vec![None; counter.capacity()],
        }
    }

    /// Each place, in `BarCounter::places`' order, empty or with its vessel.
    pub fn places(&self) -> &[Option<Item>] {
        &self.places
    }

    /// The vessels on it, in their places' order.
    pub fn items(&self) -> impl Iterator<Item = &Item> {
        self.places.iter().flatten()
    }

    /// Whether a bar is made for `item`: a drinking vessel (tankard, cup,
    /// goblet, drinking horn or jug), full or empty.
    pub fn suits(item: &Item) -> bool {
        let drinking = |kind| {
            use VesselKind::*;
            matches!(kind, Tankard | Cup | Goblet | DrinkingHorn | Jug)
        };
        item.kind == ItemKind::Vessel && item.vessel().is_some_and(|form| drinking(form.vessel))
    }

    /// Whether it takes `item`: one it suits, while a place is free.
    pub fn takes(&self, item: &Item) -> bool {
        Self::suits(item) && self.places.iter().any(Option::is_none)
    }

    /// Puts `item` in the first free place (nothing is ever swapped off).
    /// Anything else gets the one `NotTaken`, never saying why.
    pub fn put(&mut self, item: Item) -> Result<(), NotTaken> {
        if !self.takes(&item) {
            return Err(NotTaken);
        }
        let Some(free) = self.places.iter_mut().find(|place| place.is_none()) else {
            return Err(NotTaken);
        };
        *free = Some(item);
        Ok(())
    }

    /// Takes the vessel `id` off, if it is on it.
    pub fn take(&mut self, id: Id) -> Option<Item> {
        let mut places = self.places.iter_mut();
        let place = places.find(|place| place.as_ref().is_some_and(|item| item.id == id))?;
        place.take()
    }

    /// Puts `item` back in the place of the one with its id (poured from at
    /// the bar). Whether it was there.
    pub fn replace(&mut self, item: Item) -> bool {
        let mut places = self.places.iter_mut().flatten();
        let Some(place) = places.find(|place| place.id == item.id) else {
            return false;
        };
        *place = item;
        true
    }

    /// Lets the real time pass until `now` (seconds since 1970) on its
    /// vessels as on the ground (a bar ages nothing faster). Whether anything
    /// a player sees changed.
    pub fn settle(&mut self, now: u64) -> bool {
        let mut seen = false;
        for item in self.places.iter_mut().flatten() {
            seen |= (item.settle_lying_casks_ageing(now, 100)).seen;
        }
        seen
    }
}
