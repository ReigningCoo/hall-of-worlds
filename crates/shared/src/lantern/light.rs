//! How far a carrier's light reaches (owner): a lit lantern, held or tied,
//! lights 10 cells; without one there is no light at all.

use crate::dungeon::CARRIED_LIGHT;
use crate::item::Inventory;

/// How far, in cells, a lit lantern's light reaches.
pub const LANTERN_LIGHT: u32 = 10;

impl Inventory {
    /// How far its carrier's light reaches, in cells: `LANTERN_LIGHT` with a
    /// lit lantern (`lantern_lit`), else `CARRIED_LIGHT`.
    pub fn light_reach(&self) -> u32 {
        match self.lantern_lit() {
            true => LANTERN_LIGHT,
            false => CARRIED_LIGHT,
        }
    }
}
