//! Start and the outputs below the grid once it is wider than the room
//! beside them (`widgets::SideBelow`): the panel grows taller instead.

use bevy::prelude::*;

use super::bench::Bench;
use super::boxes::BOX;
use crate::dom::Dom;
use crate::widgets::SideBelow;

/// The Cooking page's halves, as `markup::COOKING` writes them.
const HALVES: SideBelow = SideBelow {
    body: "cooking/body",
    steps: "cooking/left",
    steps_width: 520.0,
    side_width: 240.0,
};

/// The gap between the grid's columns (`grid`).
const GAP: f32 = 8.0;

/// How wide the grid is with `columns` shown.
pub(super) fn grid_width(columns: u8) -> f32 {
    let columns = f32::from(columns);
    columns * BOX + (columns - 1.0) * GAP
}

/// Lays the halves out again when the grid's width passes the room beside
/// the side, or the panel opens.
pub(super) fn fit_side(bench: Res<Bench>, mut dom: Dom, mut laid: Local<Option<(Entity, u32)>>) {
    let Some(document) = bench.document else {
        *laid = None;
        return;
    };
    let wide = grid_width(bench.columns_shown());
    let now = (document, if HALVES.below(wide) { wide as u32 } else { 0 });
    if *laid == Some(now) {
        return;
    }
    HALVES.place(&mut dom, wide);
    *laid = Some(now);
}
