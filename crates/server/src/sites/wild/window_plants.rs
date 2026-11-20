//! The plants and fungi copied into a wilderness window: from each block's
//! cells (`block_plants`), and from a set-in site's first level
//! (`cutout_plants`), its herbs, fruit, vegetables and fungi. Fungi on trees
//! are copied whether growing now or not, for when the tree is felled.

use shared::dungeon::Level;

use crate::wilderness::WildChunk;

/// Copies what grows on cell `(wrapped, y)` of block `made` to `(x, y)`.
pub(super) fn block_plants(level: &mut Level, made: &Level, (x, y): (i64, i64), wrapped: i64) {
    if let Some(patch) = made.herb_at(wrapped, y) {
        level.set_herb(x, y, patch.clone());
    }
    if let Some(plant) = made.fruit_at(wrapped, y) {
        level.set_fruit(x, y, plant.clone());
    }
    if let Some(plant) = made.vegetable_at(wrapped, y) {
        level.set_vegetable(x, y, plant.clone());
    }
    if let Some(patch) = made.kept_fungus_at(wrapped, y) {
        level.set_fungus(x, y, patch.clone());
    }
}

/// Copies what grows on a set-in site's first level, `turn` cells round.
pub(super) fn cutout_plants(level: &mut Level, chunk: &WildChunk, turn: i64) {
    for ((x, y), patch) in &chunk.herbs {
        level.set_herb(x + turn, *y, patch.clone());
    }
    for ((x, y), plant) in &chunk.fruit {
        level.set_fruit(x + turn, *y, plant.clone());
    }
    for ((x, y), plant) in &chunk.vegetables {
        level.set_vegetable(x + turn, *y, plant.clone());
    }
    for ((x, y), patch) in &chunk.fungi {
        level.set_fungus(x + turn, *y, patch.clone());
    }
}
