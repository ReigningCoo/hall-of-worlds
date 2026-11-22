//! The grid's small pieces: where a cell goes (`at`), the rows and the
//! spaces under a box, a space's widget id, and the bottle standing for
//! water faced.

use bevy::prelude::*;
use shared::id::Id;
use shared::item::{Item, ItemKind};
use shared::item_form::ItemForm;
use shared::potion::{BottleForm, Liquid, Water};

use super::boxes::BOX;
use super::spaces::Place;

/// A bottle of water standing for the water faced in a space.
pub(super) fn water_shown() -> Item {
    let mut bottle = Item::new(Id::stand_in(0), ItemKind::Bottle);
    let water = Some(Liquid::Water(Water::Plain));
    bottle.form = Some(ItemForm::Bottle(BottleForm::new(water, 0)));
    bottle
}

/// A space's id's name and index: `cooking/source.<row>`,
/// `food.<step × 100 + n>`, `liquid.<step × 100 + n>`, `faced.<step>`,
/// `culture.0`, `ghost.<step × 100 + n>`.
pub(super) fn place_id(place: Place) -> (&'static str, usize) {
    match place {
        Place::Source(row) => ("source", usize::from(row)),
        Place::Food(step, n) => ("food", step * 100 + n),
        Place::Liquid(step, n) => ("liquid", step * 100 + n),
        Place::Faced(step) => ("faced", step),
        Place::Culture => ("culture", 0),
        Place::Ghost(step, n) => ("ghost", step * 100 + n),
    }
}

/// A cell of the grid at `column` (0 the sources'), `row`, `height` rows tall.
pub(super) fn at(column: u8, row: u8, height: u8) -> Node {
    Node {
        grid_column: GridPlacement::start(i16::from(column) + 1),
        grid_row: GridPlacement::start_span(i16::from(row) + 1, u16::from(height)),
        flex_direction: FlexDirection::Column,
        row_gap: px(4),
        ..default()
    }
}

/// A row of boxes, 8 apart.
pub(super) fn row_node() -> Node {
    Node {
        flex_direction: FlexDirection::Row,
        column_gap: px(8),
        min_height: px(BOX),
        ..default()
    }
}

/// A step's spaces, two to a row under its box.
pub(super) fn spaces_node() -> Node {
    Node {
        width: px(BOX),
        flex_wrap: FlexWrap::Wrap,
        ..default()
    }
}
