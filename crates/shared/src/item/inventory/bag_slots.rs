//! Bag slots (owner, 2026-11-15): a worn belt gives slots for bags tied to it, and
//! each tied bag's inside adds to the end of the own grid, laid out in rows of its
//! width, so the last row may be short. What lies there is in `Inventory::items`
//! like the rest; a tied bag holds nothing. A holder (a keyring) adds no cells:
//! it keeps its own grid, which works while tied. Tying and untying: `tying.rs`.

use crate::id::Id;
use crate::item::{Inventory, Item, ItemKind, Quality};

/// How many bags a belt of this quality takes: Poor and Common 1, then one more for
/// each quality above (Artifact 6).
pub fn belt_slots(quality: Option<Quality>) -> usize {
    let quality = quality.unwrap_or_default();
    let rank = Quality::ALL.iter().position(|&q| q == quality).unwrap_or(1);
    rank.max(1)
}

/// The cells a tied bag adds to the own grid: all of its inside, or none for a
/// holder, which keeps its own grid (`ContainerDef::holder`).
pub(super) fn tied_cells(bag: &Item) -> u32 {
    match bag.kind.def().container {
        Some(inside) if !inside.holder => inside.width as u32 * inside.height as u32,
        _ => 0,
    }
}

impl Inventory {
    /// How many bag slots the worn belt gives: none without one.
    pub fn bag_slot_count(&self) -> usize {
        self.worn
            .iter()
            .filter(|item| item.kind == ItemKind::Belt)
            .map(|belt| belt_slots(belt.quality))
            .max()
            .unwrap_or(0)
    }

    /// Whether this is a bag tied to the belt.
    pub fn is_tied(&self, id: Id) -> bool {
        self.bag_slots.iter().any(|bag| bag.id == id)
    }

    /// Whether this is a tied bag whose inside is the own grid's end, so no grid
    /// of its own: tied, and not a holder.
    pub fn adds_to_grid(&self, id: Id) -> bool {
        self.bag_slots
            .iter()
            .any(|bag| bag.id == id && tied_cells(bag) > 0)
    }

    /// The tied holders (`ContainerDef::holder`) that take this kind of item.
    pub fn holders_for(&self, kind: ItemKind) -> impl Iterator<Item = &Item> {
        self.bag_slots.iter().filter(move |bag| {
            bag.kind
                .def()
                .container
                .is_some_and(|inside| inside.holder && inside.takes(kind))
        })
    }

    /// How many cells the own grid has, with every tied bag's (a player's own
    /// from Strength: `own_cells`).
    pub fn grid_cells(&self) -> u32 {
        self.own_cells() + self.bag_slots.iter().map(tied_cells).sum::<u32>()
    }

    /// The own grid's width and rows, with the tied bags' cells: the last row may
    /// be short (`has_cell`).
    pub(super) fn combined_size(&self) -> (u8, u8) {
        let width = self.size.0.max(1);
        let rows = self.grid_cells().div_ceil(width as u32);
        (width, rows.min(u8::MAX as u32) as u8)
    }

    /// Whether the own grid has this cell: the last row's are there only as far
    /// as the cells reach.
    pub fn has_cell(&self, x: u8, y: u8) -> bool {
        let (width, _) = self.combined_size();
        x < width && (y as u32 * width as u32 + x as u32) < self.grid_cells()
    }

    /// Whether everything in the own grid lies on cells it has.
    pub(crate) fn all_on_cells(&self) -> bool {
        self.items
            .iter()
            .flat_map(|placed| placed.cells())
            .all(|(x, y)| self.has_cell(x, y))
    }

    /// How many grams lighter tied bags make what lies in the own grid, which
    /// weighs `grams` (`grams_off`).
    pub(super) fn grams_off(&self, grams: u32) -> u32 {
        let shares = self.bag_slots.iter().filter_map(|bag| {
            let inside = bag.kind.def().container?;
            Some((inside.lighter_by, tied_cells(bag)))
        });
        grams_off(grams, shares, self.grid_cells())
    }
}

/// How many of `grams` lying in a grid of `cells` cells tied bags take off: each
/// bag's percentage (`lighter_by`) for its share of the cells, in whole numbers,
/// so a 50% bag giving 16 cells of 40 takes off 20%.
pub fn grams_off(grams: u32, bags: impl Iterator<Item = (u8, u32)>, cells: u32) -> u32 {
    let share: u64 = bags.map(|(percent, own)| percent as u64 * own as u64).sum();
    let cells = cells.max(1) as u64;
    (grams as u64 * share / (100 * cells)).min(grams as u64) as u32
}
