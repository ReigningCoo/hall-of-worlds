//! World Gates stand on their cell's ground, raised by its height
//! (`common::DrawnGround`; none in the Hall), and moved with the Hall's bend
//! (`common::HallBend`).

use bevy::prelude::*;

use super::place::Gates;
use crate::common::{DrawnGround, HallBend};
use crate::dungeon_map::CELL_SIZE;

/// Puts each gate on its cell's ground whenever gates, heights or the bend
/// change.
pub(super) fn raise_gates(
    (heights, bend): (Res<DrawnGround>, Res<HallBend>),
    gates: Res<Gates>,
    mut places: Query<&mut Transform>,
) {
    if !heights.is_changed() && !gates.is_changed() && !bend.is_changed() {
        return;
    }
    for (&(x, y), &frame) in gates.standing() {
        if let Ok(mut place) = places.get_mut(frame) {
            let ground = heights.drawn(x, y) * CELL_SIZE;
            let straight = Vec3::new(x as f32, 0.0, y as f32) * CELL_SIZE + Vec3::Y * ground;
            let drawn = bend.shift(straight, CELL_SIZE);
            if place.translation != drawn {
                place.translation = drawn;
            }
        }
    }
}
