//! Which wild tree (`tree::TreeKind`) stands on each `Cell::Tree` of a level,
//! standing or felled. The server rolls them where the level is made
//! (`Wilderness::tree_kinds`, `Wilderness::site_tree_kind`) and sends them with
//! the cells, so the client draws each kind as it is.

use super::Level;
use crate::tree::TreeKind;

impl Level {
    /// The kind of the wild tree at `(x, y)`, if it is known.
    pub fn tree_kind_at(&self, x: i64, y: i64) -> Option<TreeKind> {
        self.tree_kinds.get(&(x, y)).copied()
    }

    /// Sets (or with `None` forgets) the kind of the wild tree at `(x, y)`.
    pub fn set_tree_kind(&mut self, x: i64, y: i64, kind: Option<TreeKind>) {
        match kind {
            Some(kind) => self.tree_kinds.insert((x, y), kind),
            None => self.tree_kinds.remove(&(x, y)),
        };
    }
}
