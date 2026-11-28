//! The receive box (trading plan PR 5): what the other side of a settled
//! trade gave, kept with the character until taken out. It takes no room or
//! weight, and while anything is in it no trade (shops' too) can start, so it
//! can't be used as storage.

use serde::{Deserialize, Serialize};

use crate::id::Id;
use crate::item::{Holder, Inventory, Item, Rotation, StackLimits};

/// Things received in a trade, not yet taken out.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Received {
    pub items: Vec<Item>,
}

/// Something in the receive box (`item`), and the place in the inventory it
/// is to be taken out to.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Place {
    pub item: Id,
    pub to: Holder,
    pub x: u8,
    pub y: u8,
    pub rotation: Rotation,
}

/// How each character takes trades with players (Settings › Gameplay,
/// "Trading"), kept on the character by the server.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum TradingSetting {
    /// A trade asked for opens straight away.
    Allow,
    /// A trade asked for is put to them first, to Allow or Deny.
    #[default]
    Ask,
    /// Nobody can trade with them: they show greyed out.
    Never,
}

impl Received {
    /// Whether it is empty, so a trade may start.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// One thing taken out, to the place `to` says (its `item` the one taken).
    /// `false`, and nothing changed, if it isn't here or doesn't go there.
    pub fn take(&mut self, to: &Place, carried: &mut Inventory) -> bool {
        let Some(at) = self.items.iter().position(|item| item.id == to.item) else {
            return false;
        };
        let item = self.items[at].clone();
        if (carried.add(item, to.to, to.x, to.y, to.rotation)).is_err() {
            return false;
        }
        self.items.remove(at);
        true
    }

    /// Everything that fits taken out, wherever it goes (`Inventory::give`);
    /// what doesn't stays. How many were taken.
    pub fn take_all(
        &mut self,
        carried: &mut Inventory,
        limits: StackLimits,
        new_id: &mut impl FnMut() -> Id,
    ) -> usize {
        let before = self.items.len();
        let items = std::mem::take(&mut self.items);
        for item in items {
            if let Err(item) = carried.give(item, limits, new_id) {
                self.items.push(item);
            }
        }
        before - self.items.len()
    }
}
