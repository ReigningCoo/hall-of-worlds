//! Trees felled on a level (woodcutting plan), by cell: how far each has grown
//! back (`tree::TreeStage`), from its stump to half grown. A grown tree has no
//! entry: it is the cell's own tree again. While felled, a wild tree's cell can
//! be walked on and a fruit tree's trunk is gone (`has_trunk`), and moving
//! over it costs what its stage costs (`move_cost_per_mille`). The server keeps
//! when each was felled and sets the stages; the client is sent them with the
//! cells.

use super::{Cell, Level};
use crate::tree::TreeStage;

impl Level {
    /// How far the tree felled at `(x, y)` has grown back, if one was felled
    /// there and isn't grown again.
    pub fn felled_at(&self, x: i64, y: i64) -> Option<TreeStage> {
        self.felled.get(&(x, y)).copied()
    }

    /// Sets how far the tree at `(x, y)` has grown back: `None` or `Grown`
    /// leaves the cell's own tree standing.
    pub fn set_felled(&mut self, x: i64, y: i64, stage: Option<TreeStage>) {
        match stage {
            Some(stage) if stage != TreeStage::Grown => self.felled.insert((x, y), stage),
            _ => self.felled.remove(&(x, y)),
        };
    }

    /// Every tree felled and not grown again, by cell.
    pub fn felled_trees(&self) -> impl Iterator<Item = ((i64, i64), TreeStage)> + '_ {
        self.felled.iter().map(|(&at, &stage)| (at, stage))
    }

    /// Whether a standing tree is at `(x, y)`, in the way and able to be
    /// felled: a wild one (`Cell::Tree`) or a fruit plant with a trunk, and
    /// not felled.
    pub fn stands_tree(&self, x: i64, y: i64) -> bool {
        (self.cell(x, y) == Cell::Tree && self.felled_at(x, y).is_none()) || self.has_trunk(x, y)
    }

    /// How much longer a move over `(x, y)` takes than over open floor, in
    /// thousandths: its furniture's (`furniture_cost_per_mille`) times a
    /// felled tree's growing back (`TreeStage::cost_per_mille`).
    pub fn move_cost_per_mille(&self, x: i64, y: i64) -> u32 {
        let tree = (self.felled_at(x, y))
            .and_then(TreeStage::cost_per_mille)
            .unwrap_or(1000);
        self.furniture_cost_per_mille(x, y) * tree / 1000
    }
}
