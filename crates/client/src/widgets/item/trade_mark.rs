//! How a trade marks an item (trading plan PR 4, `ItemMark`): something being
//! bought and not yet paid for has a solid background behind it (over plain
//! cells, a wash of it), and something carried that is being sold a small bag
//! in its bottom left corner, inside its frame.

use bevy::prelude::*;
use shared::item::Shape;

use super::{CELL, ItemMark, at_cell};

/// Behind an unpaid item: a warm gold, unlike any grid's colour.
const UNPAID: Color = Color::srgb(0.85, 0.62, 0.18);
/// Over plain cells, letting their colour through.
const UNPAID_WASH: Color = Color::srgba(0.85, 0.62, 0.18, 0.45);
/// The bag's cloth, its edge and its tie.
const BAG: Color = Color::srgb(0.62, 0.44, 0.24);
const BAG_EDGE: Color = Color::BLACK;
/// How big the bag is, across, in logical pixels.
const SIZE: f32 = 14.0;

/// Before the picture and cells: the solid background of an unpaid item.
pub(super) fn spawn_unpaid_background(
    parent: &mut ChildSpawnerCommands,
    mark: ItemMark,
    shape: Shape,
    corner: (f32, f32),
) {
    if mark != ItemMark::Unpaid {
        return;
    }
    for (x, y) in shape.cells() {
        let cell = at_cell(corner.0 + x as f32, corner.1 + y as f32);
        parent.spawn((cell, BackgroundColor(UNPAID)));
    }
}

/// After the cells: a wash over plain (unpictured) unpaid cells, or the bag
/// on something being sold. Like the count, not what the pointer is on.
pub(super) fn spawn_trade_mark(
    parent: &mut ChildSpawnerCommands,
    mark: ItemMark,
    (shape, drawn): (Shape, bool),
    corner: (f32, f32),
    edge: f32,
) {
    match mark {
        ItemMark::None => {}
        ItemMark::Unpaid if !drawn => {
            for (x, y) in shape.cells() {
                let cell = at_cell(corner.0 + x as f32, corner.1 + y as f32);
                parent.spawn((cell, BackgroundColor(UNPAID_WASH)));
            }
        }
        ItemMark::Unpaid => {}
        ItemMark::Selling => {
            let bottom = (corner.1 + shape.height as f32) * CELL - edge;
            spawn_bag(parent, corner.0 * CELL + edge + 1.0, bottom - SIZE - 1.0);
        }
    }
}

/// A small bag, its top left at `(left, top)`: a round body and a tie above.
fn spawn_bag(parent: &mut ChildSpawnerCommands, left: f32, top: f32) {
    let body = Node {
        position_type: PositionType::Absolute,
        left: px(left),
        top: px(top + SIZE * 0.3),
        width: px(SIZE),
        height: px(SIZE * 0.7),
        border: UiRect::all(px(1.0)),
        border_radius: BorderRadius::all(px(SIZE * 0.35)),
        ..default()
    };
    parent.spawn((body, BackgroundColor(BAG), BorderColor::from(BAG_EDGE)));
    let neck = Node {
        position_type: PositionType::Absolute,
        left: px(left + SIZE * 0.3),
        top: px(top),
        width: px(SIZE * 0.4),
        height: px(SIZE * 0.4),
        border: UiRect::all(px(1.0)),
        border_radius: BorderRadius::all(px(SIZE * 0.1)),
        ..default()
    };
    parent.spawn((neck, BackgroundColor(BAG), BorderColor::from(BAG_EDGE)));
}
