//! A lantern's fuel (lantern task PR 6, owner): a bar along the bottom of
//! its picture, as a health bar, emptying as the oil burns. It stays full
//! while more than `FULL_FOR` is left, so it only moves in the last hour.

use bevy::prelude::*;
use shared::item::{Item, Shape};

use super::CELL;

/// Burning left at which the bar starts to empty: an hour (owner).
pub const FULL_FOR: u32 = 3600;

/// The bar's height, and how far it sits inside the item's edge.
const HEIGHT: f32 = 4.0;
const INSIDE: f32 = 2.0;

/// The empty part of the bar, and the oil left.
const TRACK: Color = Color::srgba(0.05, 0.04, 0.03, 0.8);
const OIL: Color = Color::srgb(0.98, 0.72, 0.15);

/// How full a lantern's bar is, 0 to 1, or `None` for anything else.
pub fn fuel_share(item: &Item) -> Option<f32> {
    let lantern = item.lantern()?;
    Some((lantern.seconds_left().min(FULL_FOR) as f32) / FULL_FOR as f32)
}

/// The bar over an item's cells, if it has fuel to show. `edge` is how thick
/// its frame is. Not what the pointer is on, as the count.
pub(super) fn spawn_fuel(
    parent: &mut ChildSpawnerCommands,
    fuel: Option<f32>,
    shape: Shape,
    corner: (f32, f32),
    edge: f32,
) {
    let Some(share) = fuel else {
        return;
    };
    let inside = edge + INSIDE;
    let bottom = (corner.1 + shape.height as f32) * CELL - inside - HEIGHT;
    let node = Node {
        position_type: PositionType::Absolute,
        left: px(corner.0 * CELL + inside),
        top: px(bottom),
        width: px(shape.width as f32 * CELL - inside * 2.0),
        height: px(HEIGHT),
        ..default()
    };
    parent
        .spawn((node, BackgroundColor(TRACK)))
        .with_child((fill(share), BackgroundColor(OIL)));
}

/// The oil left, filling the bar from the left.
fn fill(share: f32) -> Node {
    Node {
        width: percent(share * 100.0),
        height: percent(100.0),
        ..default()
    }
}

/// The bar in a slot, filling its bottom inside its edge.
pub fn fuel_in_slot(parent: &mut ChildSpawnerCommands, item: &Item, edge: f32) {
    let Some(share) = fuel_share(item) else {
        return;
    };
    let inside = edge + INSIDE;
    let node = Node {
        position_type: PositionType::Absolute,
        left: px(inside),
        right: px(inside),
        bottom: px(inside),
        height: px(HEIGHT),
        ..default()
    };
    parent
        .spawn((node, BackgroundColor(TRACK)))
        .with_child((fill(share), BackgroundColor(OIL)));
}
