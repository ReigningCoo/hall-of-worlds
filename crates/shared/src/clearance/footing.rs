//! Where a creature's middle may stand.

use crate::dungeon::{Cell, Level};

/// Whether a creature's middle may stand in cell `(x, y)`: walkable ground, or
/// a fence's or tree's cell (a fruit plant's with a trunk too) beside the
/// fence or trunk (they block only where
/// they stand: `cell_blockers`). Liquid and holes can only be jumped.
pub fn has_footing(level: &Level, x: i64, y: i64) -> bool {
    level.is_walkable(x, y)
        || matches!(level.cell(x, y), Cell::Fence | Cell::Tree)
        || level.has_trunk(x, y)
}

/// What a creature can stand on besides walkable ground (`Gait::footing`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Footing {
    /// Walkable ground only.
    #[default]
    Walks,
    /// Deep water too, swum (`Gait::swims_deep`).
    Swims,
    /// Holes and water too, floated over (a spell's Levitate).
    Floats,
}

/// `has_footing` for someone who may swim deep water (`Gait::swims_deep`):
/// water too for them (never lava), as `movement::step_on_level_by` allows;
/// someone floating (Levitate), holes and water.
pub fn has_footing_by(level: &Level, footing: Footing, x: i64, y: i64) -> bool {
    has_footing(level, x, y)
        || match footing {
            Footing::Walks => false,
            Footing::Swims => level.cell(x, y) == Cell::Water,
            Footing::Floats => matches!(level.cell(x, y), Cell::Water | Cell::Hole),
        }
}
