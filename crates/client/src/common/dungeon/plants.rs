//! The plants, felled trees and trees' kinds on cells the server has sent.

use shared::protocol::LevelChunk;

use super::CurrentDungeon;

impl CurrentDungeon {
    /// The chunk's herb patches, plants, felled trees and trees' kinds,
    /// replacing any its cells had: felled ones growing back walk as the
    /// server's do.
    pub(super) fn add_plants(&mut self, chunk: &LevelChunk) {
        for (x, y) in chunk.positions() {
            self.level.remove_herb(x, y);
            self.level.remove_fruit(x, y);
            self.level.remove_vegetable(x, y);
            self.level.set_felled(x, y, None);
            self.level.set_tree_kind(x, y, None);
        }
        for seen in &chunk.herbs {
            self.level.set_herb(seen.x, seen.y, seen.patch.clone());
        }
        for seen in &chunk.fruit {
            self.level.set_fruit(seen.x, seen.y, seen.plant.clone());
        }
        for seen in &chunk.vegetables {
            self.level.set_vegetable(seen.x, seen.y, seen.plant.clone());
        }
        for seen in &chunk.felled {
            self.level.set_felled(seen.x, seen.y, Some(seen.stage));
        }
        for seen in &chunk.trees {
            self.level.set_tree_kind(seen.x, seen.y, Some(seen.kind));
        }
    }
}
