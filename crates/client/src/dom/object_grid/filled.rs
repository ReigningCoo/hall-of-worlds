//! A grid drawing only the cells things are in (`GridShown::only_filled`, a
//! shop's stock): the grid's colour behind each thing, as one block when its
//! shape is a whole rectangle (cells apart leave gaps at most interface
//! sizes), and the grid's lines on each of its cells.

use bevy::prelude::*;

use super::GridShown;
use crate::common::TakesGifts;
use crate::widgets::{CELL, GRID_COLOUR, GRID_LINE, at_cell};

/// Behind the things, in the layer they are drawn in.
pub(super) fn spawn_filled(layer: &mut ChildSpawnerCommands, shown: &GridShown) {
    let gifts = shown.takes_gifts.map(TakesGifts);
    let mut spawn = |bundle: (Node, BackgroundColor, BorderColor)| {
        let mut made = layer.spawn(bundle);
        if let Some(gifts) = gifts {
            made.insert(gifts);
        }
    };
    for thing in &shown.things {
        let shape = thing.item.shape().turned(thing.rotation);
        let corner = (thing.x as f32, thing.y as f32);
        let whole = shape.cells().count() == usize::from(shape.width) * usize::from(shape.height);
        if whole {
            let block = Node {
                position_type: PositionType::Absolute,
                left: px(corner.0 * CELL),
                top: px(corner.1 * CELL),
                width: px(f32::from(shape.width) * CELL),
                height: px(f32::from(shape.height) * CELL),
                ..default()
            };
            let none = BorderColor::from(Color::NONE);
            spawn((block, BackgroundColor(GRID_COLOUR), none));
        }
        for (x, y) in shape.cells() {
            let cell = at_cell(corner.0 + f32::from(x), corner.1 + f32::from(y));
            let behind = match whole {
                true => Color::NONE,
                false => GRID_COLOUR,
            };
            spawn((cell, BackgroundColor(behind), BorderColor::from(GRID_LINE)));
        }
    }
}
