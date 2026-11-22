//! Decor on a level (`shared::decor`): kept by where it goes, rugs also by the
//! cells they cover, so a new rug replaces any it overlaps.

use super::Level;
use crate::decor::{Decor, DecorSlot};
use crate::door::Side;

impl Level {
    /// Every piece of decor, by where it goes.
    pub fn decor(&self) -> impl Iterator<Item = &Decor> + '_ {
        self.decor.values()
    }

    /// The decor sent with the cell `(x, y)` (`Decor::cells`): what hangs on
    /// its wall's faces or on the walls round it facing it, a chandelier over
    /// it, a rug on it.
    pub fn decor_at(&self, x: i64, y: i64) -> Vec<&Decor> {
        let mut found = Vec::new();
        for side in [Side::North, Side::East, Side::South, Side::West] {
            found.extend(self.decor.get(&DecorSlot::Wall((x, y), side)));
            let wall = side.beside((x, y));
            found.extend(self.decor.get(&DecorSlot::Wall(wall, side.opposite())));
        }
        found.extend(self.decor.get(&DecorSlot::Ceiling((x, y))));
        let rug = self.rug_cells.get(&(x, y));
        found.extend(rug.and_then(|&at| self.decor.get(&DecorSlot::Floor(at))));
        found
    }

    /// Puts `decor` on the level, replacing what was in its place (and any
    /// rug a rug overlaps). Nothing checks it fits (`Decor::fits`).
    pub fn place_decor(&mut self, decor: Decor) {
        let slot = decor.slot();
        self.remove_decor(slot);
        if let Decor::Rug { .. } = decor {
            for cell in decor.cells() {
                if let Some(&other) = self.rug_cells.get(&cell) {
                    self.remove_decor(DecorSlot::Floor(other));
                }
            }
            for cell in decor.cells() {
                self.rug_cells.insert(cell, cell_of(slot));
            }
        }
        self.decor.insert(slot, decor);
    }

    /// Takes away the decor in `slot`, if any.
    pub fn remove_decor(&mut self, slot: DecorSlot) {
        if let Some(gone) = self.decor.remove(&slot)
            && let Decor::Rug { .. } = gone
        {
            for cell in gone.cells() {
                self.rug_cells.remove(&cell);
            }
        }
    }
}

/// A rug's north-west cell, from its slot.
fn cell_of(slot: DecorSlot) -> (i64, i64) {
    match slot {
        DecorSlot::Wall(cell, _) | DecorSlot::Ceiling(cell) | DecorSlot::Floor(cell) => cell,
    }
}
