//! `Inventory`: everything a character carries, in their own grid and what they wear.

mod bag_slots;
mod changes;
mod coins;
mod cost;
mod kinds;
mod lookup;
mod parts;
mod room;
mod rules;
mod sort;
mod stacking;
mod strength_room;
mod tying;

pub use bag_slots::{belt_slots, grams_off};
pub use strength_room::{
    PLAYER_GRID_WIDTH, PLAYER_MAX_CELLS, PLAYER_MIN_CELLS, cells_for_strength,
};

use serde::{Deserialize, Serialize};

use crate::creature::CreatureKind;
use crate::item::item::weight_of;
use crate::item::{INVENTORY_SIZE, Item, Placed};

/// Everything a character carries: what is in their own grid and what they wear.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Inventory {
    /// What kind of creature is carrying it all, which decides what they can wear.
    pub creature: CreatureKind,
    /// They have had their first clothes.
    pub outfitted: bool,
    /// What is in their own grid, not worn.
    pub items: Vec<Placed>,
    /// How big their own grid is, width and rows: a mob's from their kind of
    /// creature, a player's 8 wide with as many rows as `own_cells` needs.
    pub size: (u8, u8),
    /// A player's own cells, from their Strength (`strength_room.rs`): the last
    /// row may be short. `None` for a mob, whose grid is all of `size`.
    #[serde(default)]
    pub own_cells: Option<u16>,
    /// What they have on: see `shared::wear`.
    pub worn: Vec<Item>,
    /// What they last threw away: it can be taken back until something else is
    /// thrown away, which destroys it. It weighs nothing, being left behind.
    #[serde(default)]
    pub bin: Option<Item>,
    /// Bags tied to the worn belt, in slot order, each empty: their insides add to
    /// the end of the own grid (`bag_slots.rs`). Players' only: mobs never tie any.
    #[serde(default)]
    pub bag_slots: Vec<Item>,
}

impl Default for Inventory {
    fn default() -> Self {
        Self {
            creature: CreatureKind::default(),
            outfitted: false,
            items: Vec::new(),
            size: INVENTORY_SIZE,
            own_cells: None,
            worn: Vec::new(),
            bin: None,
            bag_slots: Vec::new(),
        }
    }
}

impl Inventory {
    /// What it all weighs, in grams: what lies in the own grid made lighter by any
    /// tied bags for their share of it (`grams_off`), then what is worn and the tied
    /// bags themselves.
    pub fn total_weight(&self) -> u32 {
        let grid = weight_of(&self.items);
        let worn = self.worn.iter().chain(&self.bag_slots);
        grid - self.grams_off(grid) + worn.map(Item::total_weight).sum::<u32>()
    }

    /// What the creature weighs with all it carries and wears, in grams.
    pub fn weight_with_creature(&self) -> u32 {
        self.creature.def().weight + self.total_weight()
    }
}
