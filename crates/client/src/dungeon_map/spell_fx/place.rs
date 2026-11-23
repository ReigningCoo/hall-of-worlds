//! Where on screen a spell's spots are: in the level's units, on the drawn
//! ground, at about chest height, and which way a heading points.

use std::f32::consts::TAU;

use bevy::prelude::*;
use shared::facing::{Heading, TURN};
use shared::spot::{CELL_SPAN, Spot};

use super::super::CELL_SIZE;
use crate::common::DrawnGround;

/// How high spells fly and spread over the ground, in cells.
pub(super) const CHEST: f32 = 0.6;

/// A length in units as one on screen.
pub(super) fn units_drawn(units: u64) -> f32 {
    units as f32 / CELL_SPAN as f32 * CELL_SIZE
}

/// `spot` on the drawn ground, raised `above` cells.
pub(super) fn spot_drawn(ground: &DrawnGround, spot: Spot, above: f32) -> Vec3 {
    let span = CELL_SPAN as f32;
    let (x, y) = (spot.x as f32 / span, spot.y as f32 / span);
    Vec3::new(x, ground.drawn_between(x, y) + above, y) * CELL_SIZE
}

/// A heading's angle in radians, from east towards south as on screen.
pub(super) fn radians(heading: Heading) -> f32 {
    f32::from(heading.0) / TURN as f32 * TAU
}

/// The way a heading points, flat on the ground.
pub(super) fn way(heading: Heading) -> Vec3 {
    let angle = radians(heading);
    Vec3::new(angle.cos(), 0.0, angle.sin())
}
