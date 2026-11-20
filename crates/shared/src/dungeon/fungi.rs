//! Fungus patches on a level (`shared::fungus`), by cell: what grows there and
//! when it was last picked. They never block anything. A patch on a wild
//! tree's cell (`Cell::Tree`) feeds on the tree once felled: it is there only
//! while the felled tree is `Cleared` (the stump gone, nothing come up yet),
//! so both sides agree from the felled stages sent with the cells.

use super::{Cell, Level};
use crate::fungus::FungusPatch;
use crate::tree::TreeStage;

impl Level {
    /// The fungus patch growing at `(x, y)` now, if any.
    pub fn fungus_at(&self, x: i64, y: i64) -> Option<&FungusPatch> {
        self.fungi.get(&(x, y)).filter(|_| self.fungus_shows(x, y))
    }

    /// Every fungus patch growing now, by cell.
    pub fn fungus_patches(&self) -> impl Iterator<Item = ((i64, i64), &FungusPatch)> + '_ {
        (self.fungi.iter())
            .filter(|&(&(x, y), _)| self.fungus_shows(x, y))
            .map(|(&at, patch)| (at, patch))
    }

    /// The fungus patch kept at `(x, y)`, growing now or waiting for its tree
    /// to be felled: for copying levels, not for showing.
    pub fn kept_fungus_at(&self, x: i64, y: i64) -> Option<&FungusPatch> {
        self.fungi.get(&(x, y))
    }

    /// Every fungus patch kept, growing now or not, by cell.
    pub fn kept_fungi(&self) -> impl Iterator<Item = ((i64, i64), &FungusPatch)> + '_ {
        self.fungi.iter().map(|(&at, patch)| (at, patch))
    }

    /// Puts a fungus patch at `(x, y)`, replacing any there.
    pub fn set_fungus(&mut self, x: i64, y: i64, patch: FungusPatch) {
        self.fungi.insert((x, y), patch);
    }

    /// Takes away the fungus patch at `(x, y)`, if any.
    pub fn remove_fungus(&mut self, x: i64, y: i64) {
        self.fungi.remove(&(x, y));
    }

    /// Notes when the patch at `(x, y)` was last picked, if there is one there.
    pub fn set_fungus_picked(&mut self, x: i64, y: i64, picked: Option<u64>) {
        if let Some(patch) = self.fungi.get_mut(&(x, y)) {
            patch.picked = picked;
        }
    }

    /// Whether a patch kept at `(x, y)` grows now: always, but on a wild
    /// tree's cell only while the felled tree is cleared.
    fn fungus_shows(&self, x: i64, y: i64) -> bool {
        self.cell(x, y) != Cell::Tree || self.felled_at(x, y) == Some(TreeStage::Cleared)
    }
}
