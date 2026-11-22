//! Bags stand on the ground where they are, raised where it slopes
//! (`common::DrawnGround`), and moved with the Hall's bend (`common::HallBend`).

use bevy::prelude::*;

use super::{Bags, in_cells};
use crate::common::{DrawnGround, HallBend};
use crate::dungeon_map::CELL_SIZE;

/// Puts each bag on the ground under it whenever bags, heights or the bend
/// change.
pub(super) fn raise_bags(
    (heights, bend): (Res<DrawnGround>, Res<HallBend>),
    bags: Res<Bags>,
    mut places: Query<&mut Transform>,
) {
    if !heights.is_changed() && !bags.is_changed() && !bend.is_changed() {
        return;
    }
    for (&at, bag) in &bags.0 {
        if let Ok(mut place) = places.get_mut(bag.root) {
            let (x, y) = in_cells(at);
            let ground = heights.drawn_between(x, y) * CELL_SIZE;
            let drawn = bend.shift(Vec3::new(x * CELL_SIZE, ground, y * CELL_SIZE), CELL_SIZE);
            if place.translation != drawn {
                place.translation = drawn;
            }
        }
    }
}
