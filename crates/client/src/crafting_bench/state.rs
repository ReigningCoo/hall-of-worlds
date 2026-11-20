//! What the panel and the walk to a bench keep, and where a bench is reached
//! from.

use bevy::prelude::*;
use shared::dungeon::Level;
use shared::furniture::Piece;
use shared::potion::COLLECT_REACH;
use shared::spot::Spot;

use crate::widgets::StockGrid;

/// The bench panel open now, if any (one at a time).
#[derive(Resource, Default)]
pub(super) struct BenchPanel(pub(super) Option<OpenBench>);

/// An open bench panel.
pub(super) struct OpenBench {
    /// The bench, as it was when opened.
    pub(super) piece: Piece,
    /// Its document.
    pub(super) panel: Entity,
    /// Its grid's node, once built.
    pub(super) holder: Option<Entity>,
    /// Its tools, where they are in the grid.
    pub(super) grid: StockGrid,
}

/// The bench the player is walking up to, and the cell in front of it they
/// are going to.
#[derive(Resource, Default)]
pub(super) struct BenchHeading(pub(super) Option<(Piece, (i64, i64))>);

/// Whether someone at `at` can reach `piece`: any of its cells' middles
/// within `COLLECT_REACH`, as the server checks.
pub(super) fn in_reach(at: Spot, piece: &Piece) -> bool {
    let reach = u128::from(COLLECT_REACH).pow(2);
    (piece.cells()).any(|cell| at.distance_squared(Spot::middle_of(cell)) <= reach)
}

/// The cell of `piece` nearest `at`.
pub(super) fn nearest_cell(at: Spot, piece: &Piece) -> (i64, i64) {
    let distance = |cell: &(i64, i64)| at.distance_squared(Spot::middle_of(*cell));
    (piece.cells()).min_by_key(distance).unwrap_or(piece.at)
}

/// The walkable cell in front of `piece` (on the side it faces) nearest `at`,
/// to walk to; `None` if every one is blocked.
pub(super) fn front_cell(level: &Level, at: Spot, piece: &Piece) -> Option<(i64, i64)> {
    let fronts = piece.cells().map(|cell| piece.facing.beside(cell));
    let fronts = fronts.filter(|&(x, y)| level.is_walkable(x, y) && !piece.covers((x, y)));
    fronts.min_by_key(|cell| at.distance_squared(Spot::middle_of(*cell)))
}
