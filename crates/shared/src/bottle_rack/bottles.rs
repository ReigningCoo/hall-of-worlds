//! `RackBottles`: the bottles on a wine or spirit rack, one a place, the
//! server's to keep; which it takes, putting one on, taking one off, and their
//! clocks.

use serde::{Deserialize, Serialize};

use super::{BottleRack, BottleRackKind};
use crate::cask_rack::RACK_AGEING_PERCENT;
use crate::crafting_bench::NotTaken;
use crate::id::Id;
use crate::item::{Item, ItemKind};
use crate::item_form::ItemForm;
use crate::potion::Liquid;
use crate::vessel::VesselKind;

/// What lies on a bottle rack: a place for each bottle it holds
/// (`BottleRack::places`), each empty or holding one bottle.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RackBottles {
    pub rack: BottleRack,
    places: Vec<Option<Item>>,
}

impl RackBottles {
    /// An empty one.
    pub fn new(rack: BottleRack) -> Self {
        Self {
            rack,
            places: vec![None; rack.capacity()],
        }
    }

    /// Each place, in `BottleRack::places`' order, empty or with its bottle.
    pub fn places(&self) -> &[Option<Item>] {
        &self.places
    }

    /// The bottles on it, in their places' order.
    pub fn items(&self) -> impl Iterator<Item = &Item> {
        self.places.iter().flatten()
    }

    /// Whether a rack of `kind` is made for `item` (one alone, not a stack):
    /// a wine rack wine bottles, full or empty; a spirit rack those, flasks,
    /// and bottles of a fermented drink or pure alcohol (Claude's reading of
    /// the owner's "bottles of drink").
    pub fn suits(kind: BottleRackKind, item: &Item) -> bool {
        if item.count() != 1 {
            return false;
        }
        match (&item.form, kind) {
            (Some(ItemForm::Vessel(vessel)), BottleRackKind::Wine) => {
                vessel.vessel == VesselKind::WineBottle
            }
            (Some(ItemForm::Vessel(vessel)), BottleRackKind::Spirit) => {
                matches!(vessel.vessel, VesselKind::WineBottle | VesselKind::Flask)
            }
            (Some(ItemForm::Bottle(bottle)), BottleRackKind::Spirit) => {
                item.kind == ItemKind::Bottle
                    && matches!(bottle.liquid, Some(Liquid::Drink(_) | Liquid::Alcohol))
            }
            _ => false,
        }
    }

    /// Whether it takes `item`: one it suits, while a place is free.
    pub fn takes(&self, item: &Item) -> bool {
        Self::suits(self.rack.kind, item) && self.places.iter().any(Option::is_none)
    }

    /// Puts `item` in the first free place, lowest first (nothing is ever
    /// swapped off). Anything else gets the one `NotTaken`, never saying why.
    /// Its clocks are the caller's to lay down.
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

    /// Takes the bottle `id` off, if it is on it.
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

    /// Lets the real time pass until `now` (seconds since 1970) on its bottles
    /// (laid down when put on), sealed drinks ageing `RACK_AGEING_PERCENT` as
    /// fast (owner: 10% faster on a wine or spirit rack). Whether anything a
    /// player sees changed.
    pub fn settle(&mut self, now: u64) -> bool {
        let mut seen = false;
        for item in self.places.iter_mut().flatten() {
            seen |= (item.settle_lying_casks_ageing(now, RACK_AGEING_PERCENT)).seen;
        }
        seen
    }
}
