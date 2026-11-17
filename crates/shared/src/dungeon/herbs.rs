//! Herb patches on a level (`shared::herb`), by cell: what grows there and
//! when it was last picked. They never block anything.

use super::Level;
use crate::herb::HerbPatch;

impl Level {
    /// The herb patch at `(x, y)`, if any.
    pub fn herb_at(&self, x: i64, y: i64) -> Option<&HerbPatch> {
        self.herbs.get(&(x, y))
    }

    /// Every herb patch, by cell.
    pub fn herbs(&self) -> impl Iterator<Item = ((i64, i64), &HerbPatch)> + '_ {
        self.herbs.iter().map(|(&at, patch)| (at, patch))
    }

    /// Puts a herb patch at `(x, y)`, replacing any there.
    pub fn set_herb(&mut self, x: i64, y: i64, patch: HerbPatch) {
        self.herbs.insert((x, y), patch);
    }

    /// Takes away the herb patch at `(x, y)`, if any.
    pub fn remove_herb(&mut self, x: i64, y: i64) {
        self.herbs.remove(&(x, y));
    }

    /// Notes when the patch at `(x, y)` was last picked, if there is one there.
    pub fn set_herb_picked(&mut self, x: i64, y: i64, picked: Option<u64>) {
        if let Some(patch) = self.herbs.get_mut(&(x, y)) {
            patch.picked = picked;
        }
    }
}
