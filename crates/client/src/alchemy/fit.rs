//! Start and the product below the steps once the process is wider than the
//! room beside them (`widgets::SideBelow`): the panel grows taller instead.

use bevy::prelude::*;

use super::bench::Bench;
use super::boxes::BOX;
use crate::dom::Dom;
use crate::widgets::SideBelow;

/// The panel's halves, as `markup::ALCHEMY` writes them.
const HALVES: SideBelow = SideBelow {
    body: "alchemy/body",
    steps: "alchemy/left",
    steps_width: 480.0,
    side_width: 220.0,
};

/// The gap between boxes in the process row (`rows::row_node`).
const GAP: f32 = 8.0;

/// How wide the process row is: each step's box and the end box.
pub(super) fn process_width(steps: usize) -> f32 {
    let boxes = steps as f32 + 1.0;
    boxes * BOX + (boxes - 1.0) * GAP
}

/// Lays the halves out again when the process's width passes the room
/// beside the side, or the panel opens.
pub(super) fn fit_side(bench: Res<Bench>, mut dom: Dom, mut laid: Local<Option<(Entity, u32)>>) {
    let Some(document) = bench.document else {
        *laid = None;
        return;
    };
    let wide = process_width(bench.steps.len());
    let now = (document, if HALVES.below(wide) { wide as u32 } else { 0 });
    if *laid == Some(now) {
        return;
    }
    HALVES.place(&mut dom, wide);
    *laid = Some(now);
}
