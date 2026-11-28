//! How a trade marks an item (trading plan PR 4, `ItemMark`): something
//! carried that is in a trade, and everything on a trade window's sides, has
//! a small bag in its top left corner, inside its frame (trading tweaks PR 3).

use bevy::prelude::*;

use super::{CELL, ItemMark};

/// The bag's cloth, its edge and its tie.
const BAG: Color = Color::srgb(0.62, 0.44, 0.24);
const BAG_EDGE: Color = Color::BLACK;
/// How big the bag is, across, in logical pixels.
const SIZE: f32 = 14.0;

/// After the cells: the bag on something being sold. Like the count, not
/// what the pointer is on.
pub(super) fn spawn_trade_mark(
    parent: &mut ChildSpawnerCommands,
    mark: ItemMark,
    corner: (f32, f32),
    edge: f32,
) {
    match mark {
        ItemMark::None => {}
        ItemMark::Selling => {
            let top = corner.1 * CELL + edge + 1.0;
            spawn_bag(parent, corner.0 * CELL + edge + 1.0, top);
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
