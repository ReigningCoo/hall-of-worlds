//! How each kind of site is furnished (Claude's first guesses for the owner
//! to tune).

use crate::crafting_bench::always_lit;
use crate::poi::PoiKind;

/// Chances and amounts for one kind of site.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Setting {
    /// In percent: how likely a room of a floor above the ground is to be a
    /// bedroom.
    pub bedroom_upstairs_percent: u64,
    /// In percent: the same on the ground floor or below.
    pub bedroom_downstairs_percent: u64,
    /// In percent: how likely the great hall is to have a fireplace.
    pub fireplace_percent: u64,
    /// In percent: how likely the cellar is to have a wine rack.
    pub wine_rack_percent: u64,
    /// About one thing hanging on a wall per this many cells of a room.
    pub cells_per_hanging: usize,
    /// Whether people live there: its sconces, chandelier and candles are lit
    /// (only if it is not a ruin), and its fireplace always burns.
    pub lit: bool,
}

impl Setting {
    /// The setting for `kind`: castles and towers, none for other kinds.
    pub fn for_kind(kind: PoiKind) -> Option<Self> {
        let (upstairs, fireplace, wine_rack, cells_per_hanging) = match kind {
            PoiKind::Castle => (70, 90, 60, 10),
            PoiKind::StoneTower => (60, 60, 30, 14),
            PoiKind::WoodenTower => (60, 30, 20, 16),
            _ => return None,
        };
        Some(Self {
            bedroom_upstairs_percent: upstairs,
            bedroom_downstairs_percent: 20,
            fireplace_percent: fireplace,
            wine_rack_percent: wine_rack,
            cells_per_hanging,
            lit: always_lit(kind),
        })
    }
}
