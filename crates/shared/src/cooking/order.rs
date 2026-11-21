//! A cooking process as asked for (cooking plan PR 9): which carried stacks,
//! liquids and intermediates go into each step, where it is worked, and how
//! many times to run it; checked against what is carried and the kitchen the
//! cook is at with the same rules on both sides (`Inventory::check_cooking`).

use serde::{Deserialize, Serialize};

use super::{Heat, Refused, Tool};
use crate::id::Id;
use crate::item::{Item, ItemKind};

/// The most times one order may be run without the cook stepping in, as
/// Alchemy's.
pub const MOST_REPEATS: u32 = crate::alchemy::MOST_REPEATS;

/// A process asked for.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CookOrder {
    /// A cell of the hearth to work at, which the cook must be at
    /// (`crafting_bench::hearth_faced`): its heat, Oven and tools, and the
    /// intermediates in a Range's storage. `None` works with what is
    /// carried, a town's virtual hearth, or a campfire faced.
    pub bench: Option<(i64, i64)>,
    /// Rows in the grid, 1 to `MOST_ROWS`.
    pub rows: u8,
    /// Each row's source space, by row: a carried intermediate, or one in
    /// the storage of the hearth worked at. Used up whole.
    pub sources: Vec<Option<Id>>,
    pub steps: Vec<OrderStep>,
    /// A carried culture (a Yeast item of any strain), put into an output
    /// that ferments (`food::Rest::culture`), one used a run only then.
    #[serde(default)]
    pub culture: Option<Id>,
    /// 1 to `MOST_REPEATS`: each run its own, with its own rolls.
    pub count: u32,
}

/// One step as asked for: its tool and place in the grid (as
/// `cooking::Step`), and what is put into it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrderStep {
    pub tool: Tool,
    pub column: u8,
    pub row: u8,
    pub height: u8,
    pub inputs: Vec<OrderInput>,
}

/// Something put into a step.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderInput {
    /// `count` of a carried stack of raw food (herbs, fruit, vegetables,
    /// fungi, flour, eggs), each its own weight.
    Food { stack: Id, count: u32 },
    /// `ml` of water, milk or honey from a carried bottle (whole bottles,
    /// 100 ml each, given back empty), vessel or cask.
    Liquid { stack: Id, ml: u32 },
    /// `ml` of water drawn from the water the cook faces.
    WaterFaced { ml: u32 },
}

/// Where the cook is working, as the server finds it (and the client's
/// preview the same way).
#[derive(Clone, Copy, Debug, Default)]
pub struct Kitchen<'a> {
    /// A lit hearth or a town: `Hearth`; a lit campfire faced: `Fire`;
    /// else `Cold`.
    pub heat: Heat,
    /// Whether an Oven is to hand: a lit hearth's, or a town's.
    pub oven: bool,
    /// The tools the hearth or town has, used before carried ones.
    pub tools: &'a [ItemKind],
    /// The storage of the hearth worked at (a Range's): intermediates there
    /// can start rows.
    pub storage: &'a [Item],
    /// Whether the cook faces water to draw from.
    pub water_faced: bool,
}

/// Why an order can't be run with what is carried and the kitchen.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OrderRefused {
    /// The count is 0 or over `MOST_REPEATS`.
    Count,
    /// This tool isn't carried, nor to hand (an Oven: no lit hearth).
    Tool(Tool),
    /// A source isn't a carried intermediate, nor one in the storage.
    Source,
    /// A food stack isn't carried raw food with culinary data.
    Food,
    /// A liquid isn't a carried bottle, vessel or cask of water, milk or
    /// honey, or a bottle not taken whole.
    Liquid,
    /// No water faced.
    NoWaterFaced,
    /// The culture isn't a carried Yeast item.
    Culture,
    /// A stack holds fewer than one run uses (or a container less liquid).
    TooFew,
    /// The rules refused the process.
    Rules(Refused),
}
