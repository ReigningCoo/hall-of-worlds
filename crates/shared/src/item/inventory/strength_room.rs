//! A player's own grid sized by Strength (owner, 2026-11-17): always 8 wide, with
//! `max(32, Strength / 4)` cells, Strength with every bonus. Cells, not rows: the
//! last row may be short, and tied bags' cells follow on in rows of 8. Mobs keep
//! their kind's grid (`CreatureDef::inventory`).
//!
//! Losing cells works as untying a bag (`tying.rs`): a change the player makes is
//! refused while the cells it would cut hold anything (`MoveRefused::
//! StrengthInUse`); a loss nobody chose (an effect running out) moves what lies
//! there to free room, and drops what has none.

use crate::id::Id;
use crate::item::{Inventory, Item, MoveRefused, StackLimits};

/// A player's own grid is always this wide.
pub const PLAYER_GRID_WIDTH: u8 = 8;

/// The fewest cells a player's own grid has, however weak they are.
pub const PLAYER_MIN_CELLS: u32 = 32;

/// The most cells a player's own grid has: as many rows as a grid can count.
pub const PLAYER_MAX_CELLS: u32 = PLAYER_GRID_WIDTH as u32 * u8::MAX as u32;

/// How many cells a player's own grid has with this much Strength (with its
/// bonuses): a quarter of it, at least `PLAYER_MIN_CELLS`.
pub fn cells_for_strength(strength: u16) -> u32 {
    (u32::from(strength) / 4).clamp(PLAYER_MIN_CELLS, PLAYER_MAX_CELLS)
}

impl Inventory {
    /// How many cells the own grid has, without tied bags': a player's from their
    /// Strength, anyone else's the whole of their kind's grid.
    pub fn own_cells(&self) -> u32 {
        match self.own_cells {
            Some(cells) => u32::from(cells),
            None => u32::from(self.size.0) * u32::from(self.size.1),
        }
    }

    /// Whether the own grid is sized by Strength: a player's.
    pub fn sized_by_strength(&self) -> bool {
        self.own_cells.is_some()
    }

    /// Makes the own grid this many cells, 8 wide, whatever lies there.
    pub(crate) fn set_own_cells(&mut self, cells: u32) {
        let cells = cells.min(PLAYER_MAX_CELLS);
        let rows = cells.div_ceil(u32::from(PLAYER_GRID_WIDTH));
        self.own_cells = Some(cells as u16);
        self.size = (PLAYER_GRID_WIDTH, rows as u8);
    }

    /// Whether the own grid could become this many cells by the player's own
    /// doing: growing always, shrinking only if nothing lies on the cells cut
    /// (at the end of the grid, whichever they were, as for untying a bag).
    pub fn check_own_cells(&self, cells: u32) -> Result<(), MoveRefused> {
        if self.sized_by_strength() && cells < self.own_cells() {
            let mut after = self.clone();
            after.set_own_cells(cells);
            if !after.all_on_cells() {
                return Err(MoveRefused::StrengthInUse);
            }
        }
        Ok(())
    }

    /// Sizes the own grid to this many cells, though nobody chose it (an effect
    /// running out, a character loading): what lies on cells cut goes to free
    /// room, stacking, and what has none is given back, to drop. Nothing for
    /// an inventory not sized by Strength until `start_sizing`.
    pub fn fit_own_cells(
        &mut self,
        cells: u32,
        limits: StackLimits,
        new_id: &mut impl FnMut() -> Id,
    ) -> Vec<Item> {
        let width = self.size.0 == PLAYER_GRID_WIDTH;
        if !self.sized_by_strength() || (width && cells == self.own_cells()) {
            return Vec::new();
        }
        self.set_own_cells(cells);
        let (kept, cut): (Vec<_>, Vec<_>) = std::mem::take(&mut self.items)
            .into_iter()
            .partition(|placed| placed.cells().all(|(x, y)| self.has_cell(x, y)));
        self.items = kept;
        cut.into_iter()
            .filter_map(|placed| self.give(placed.item, limits, new_id).err())
            .collect()
    }

    /// Makes this a player's inventory, sized by Strength from now on: the own
    /// grid, kept as it was, becomes `cells` cells as `fit_own_cells` would.
    pub fn start_sizing(
        &mut self,
        cells: u32,
        limits: StackLimits,
        new_id: &mut impl FnMut() -> Id,
    ) -> Vec<Item> {
        if !self.sized_by_strength() {
            self.own_cells = Some(self.own_cells().min(PLAYER_MAX_CELLS) as u16);
        }
        self.fit_own_cells(cells, limits, new_id)
    }
}
