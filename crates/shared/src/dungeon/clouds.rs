//! Clouds of steam from spells on a level (magic plan PR 11), by cell: sight
//! and lines (`line_is_clear`, `spots_are_clear`) pass into a cloud's cell
//! but not through it. The server keeps them for a few seconds and sets
//! these; the client is told of them (`protocol::SpellNews::Clouds`) and draws
//! them.

use super::Level;

impl Level {
    /// Whether a cloud of steam hangs over `(x, y)`.
    pub fn cloud_at(&self, x: i64, y: i64) -> bool {
        self.clouds.contains(&(x, y))
    }

    /// Sets whether a cloud hangs over `(x, y)`.
    pub fn set_cloud(&mut self, x: i64, y: i64, cloud: bool) {
        if cloud {
            self.clouds.insert((x, y));
        } else {
            self.clouds.remove(&(x, y));
        }
    }

    /// Every cell under a cloud.
    pub fn clouds(&self) -> impl Iterator<Item = (i64, i64)> + '_ {
        self.clouds.iter().copied()
    }
}
