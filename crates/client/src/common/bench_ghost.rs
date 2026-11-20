//! Debug: the crafting bench Create Crafting Bench would make, shown as a
//! ghost under the pointer while Bench Mode (`ClickMode::PlaceBench`) is on.

use bevy::prelude::*;
use shared::protocol::DevBench;

/// What Create Crafting Bench's panel has set up (its piece's facing, size and
/// craft, and the tools), and where `dungeon_map` has it now: the piece's
/// north-west cell under the pointer, and whether the server would take it
/// there. The panel sends `order` as it is when a cell is clicked.
#[derive(Resource, Default, Clone, Debug, PartialEq)]
pub struct BenchGhost {
    /// Set while Bench Mode is on; `None` otherwise.
    pub order: Option<DevBench>,
    /// Whether every cell of it is free floor, as the server checks
    /// (`shared::furniture::free_for_bench`, off stairs); `None` while the
    /// pointer isn't over the level, when no ghost is drawn.
    pub fits: Option<bool>,
}
