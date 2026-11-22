//! Furniture with places (tavern furniture plan PR 1, owner's design): pieces
//! holding things in fixed places, each place taking only the right item,
//! like a crafting bench's tools. What a piece with places is (`Holder`) and
//! what lies on one (`Held`), kept and saved by the server by the piece's
//! cell, put on and taken off through one panel and one pair of messages
//! (`protocol::BenchAsk`). Crafting benches hold their tools
//! (`crafting_bench::BenchTools`), cask racks their casks
//! (`cask_rack::RackCasks`).

use serde::{Deserialize, Serialize};

use super::Furniture;
use crate::cask_rack::{CaskRack, RackCasks};
use crate::crafting_bench::{BenchTools, Craft, NotTaken};
use crate::id::Id;
use crate::item::Item;

/// What a piece with places is.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Holder {
    Bench(Craft),
    Casks(CaskRack),
}

impl Holder {
    /// What `furniture` holds things as, if it has places.
    pub fn of(furniture: Furniture) -> Option<Self> {
        match furniture {
            Furniture::CraftingBench { craft } => Some(Self::Bench(craft)),
            Furniture::CaskRack { rack } => Some(Self::Casks(rack)),
            _ => None,
        }
    }

    /// Its name, for its panel's title: "Alchemy Bench", "Pin Rack".
    pub fn name(self) -> String {
        match self {
            Self::Bench(craft) => craft.name().to_owned(),
            Self::Casks(rack) => rack.name(),
        }
    }
}

/// What lies on a piece with places, the server's to keep.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Held {
    Bench(BenchTools),
    Casks(RackCasks),
}

impl Held {
    /// Nothing on a piece that is `holder`.
    pub fn new(holder: Holder) -> Self {
        match holder {
            Holder::Bench(craft) => Self::Bench(BenchTools::new(craft)),
            Holder::Casks(rack) => Self::Casks(RackCasks::new(rack)),
        }
    }

    /// The piece it lies on.
    pub fn holder(&self) -> Holder {
        match self {
            Self::Bench(tools) => Holder::Bench(tools.craft),
            Self::Casks(casks) => Holder::Casks(casks.rack),
        }
    }

    /// A bench's tools, if it is a bench's.
    pub fn bench(&self) -> Option<&BenchTools> {
        match self {
            Self::Bench(tools) => Some(tools),
            Self::Casks(_) => None,
        }
    }

    /// Everything on it, in its places' order.
    pub fn items(&self) -> Box<dyn Iterator<Item = &Item> + '_> {
        match self {
            Self::Bench(tools) => Box::new(tools.items()),
            Self::Casks(casks) => Box::new(casks.items()),
        }
    }

    /// Puts `item` in its place, giving back what it swapped with (a bench's
    /// tool of the same kind); anything it won't take gets the one
    /// `NotTaken`, never saying why.
    pub fn put(&mut self, item: Item) -> Result<Option<Item>, NotTaken> {
        match self {
            Self::Bench(tools) => tools.put(item),
            Self::Casks(casks) => casks.put(item).map(|()| None),
        }
    }

    /// Takes `id` off, if it is on it.
    pub fn take(&mut self, id: Id) -> Option<Item> {
        match self {
            Self::Bench(tools) => tools.take(id),
            Self::Casks(casks) => casks.take(id),
        }
    }
}
