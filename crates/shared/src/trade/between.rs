//! Trades between two characters, players or a player and an NPC (trading
//! plan PR 5): each side up to `SIDE_CELLS` × `SIDE_CELLS`, what is offered
//! staying carried until both are locked in, any change to a side unlocking
//! it. Each side is kept by its own character (the server's client), so the
//! state either is told is both sides put together (`Between`).

use serde::{Deserialize, Serialize};

use super::Refusal;
use crate::id::Id;
use crate::item::{Inventory, Item, Rotation};

/// How many cells wide and high each side of a trade is.
pub const SIDE_CELLS: u8 = 8;

/// Something carried put on one's side of a trade, where it lies there.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Offer {
    pub item: Id,
    pub x: u8,
    pub y: u8,
    pub rotation: Rotation,
}

/// One side of a trade: what its character offers, and whether they are
/// locked in (the Trade toggle).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Side {
    pub offers: Vec<Offer>,
    pub locked: bool,
}

/// A trade between two as one side is told it: their own side and the
/// other's (whose items come in `TradeStock`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Between {
    /// Who the trade is with: a player's character or an NPC.
    pub with: Id,
    pub mine: Side,
    pub theirs: Side,
}

impl Side {
    /// Whether `item` is offered.
    pub fn has(&self, item: Id) -> bool {
        self.offers.iter().any(|offer| offer.item == item)
    }

    /// `offer` put on this side (moved, if it is there already), checked
    /// against what is `carried`: it must be carried, tradeable, and fit in
    /// the side beside the rest. Unlocks the side.
    pub fn offer(&mut self, offer: Offer, carried: &Inventory) -> Result<(), Refusal> {
        let item = carried.find(offer.item).ok_or(Refusal::NotCarried)?;
        if !for_trade(item) {
            return Err(Refusal::NotForSale);
        }
        // Inside something else offered: it goes with that already.
        let within = |other: &Offer| {
            other.item != offer.item
                && carried
                    .find(other.item)
                    .is_some_and(|outer| holds(outer, offer.item) || holds(item, other.item))
        };
        if self.offers.iter().any(within) {
            return Err(Refusal::AlreadyIn);
        }
        let mut taken = [[false; SIDE_CELLS as usize]; SIDE_CELLS as usize];
        for other in self.offers.iter().filter(|other| other.item != offer.item) {
            let shape = carried.find(other.item).ok_or(Refusal::NotCarried)?.shape();
            for (x, y) in cells(shape.turned(other.rotation), other.x, other.y) {
                taken[y as usize][x as usize] = true;
            }
        }
        let shape = item.shape().turned(offer.rotation);
        let fits = (offer.x as u32 + shape.width as u32 <= SIDE_CELLS as u32)
            && (offer.y as u32 + shape.height as u32 <= SIDE_CELLS as u32)
            && cells(shape, offer.x, offer.y).all(|(x, y)| !taken[y as usize][x as usize]);
        if !fits {
            return Err(Refusal::NoRoom);
        }
        self.offers.retain(|other| other.item != offer.item);
        self.offers.push(offer);
        self.locked = false;
        Ok(())
    }

    /// `item` taken back off this side, unlocking it if it was there.
    pub fn take_back(&mut self, item: Id) {
        if self.has(item) {
            self.offers.retain(|offer| offer.item != item);
            self.locked = false;
        }
    }

    /// Whether everything offered is still carried and tradeable: what
    /// settling checks first.
    pub fn still_carried(&self, carried: &Inventory) -> bool {
        (self.offers.iter()).all(|offer| carried.find(offer.item).is_some_and(for_trade))
    }

    /// What is offered, taken out of `carried` (with all in it), in order:
    /// `None`, and `carried` as it was, if anything has gone.
    pub fn hand_over(&self, carried: &mut Inventory) -> Option<Vec<Item>> {
        let mut trial = carried.clone();
        let items = self.offers.iter().map(|offer| trial.hand_over(offer.item));
        let items = items.collect::<Option<Vec<Item>>>()?;
        *carried = trial;
        Some(items)
    }
}

/// Whether something can be traded to another character: anything but a
/// quest item or something holding one (quest items are the character's
/// own). Coins can, unlike to a shop.
pub fn for_trade(item: &Item) -> bool {
    !item.holds_quest_item()
}

/// Whether `outer` has `id` inside it, at any depth.
fn holds(outer: &Item, id: Id) -> bool {
    (outer.contents.iter()).any(|placed| placed.item.id == id || holds(&placed.item, id))
}

/// The side's cells a shape as it lies takes up, from (`left`, `top`).
fn cells(shape: crate::item::Shape, left: u8, top: u8) -> impl Iterator<Item = (u8, u8)> {
    let inside = move |&(x, y): &(u8, u8)| x < SIDE_CELLS && y < SIDE_CELLS;
    (shape.cells().map(move |(x, y)| (left + x, top + y))).filter(inside)
}
