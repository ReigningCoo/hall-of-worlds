//! Clouds of steam a spell left (magic plan PR 11, `SpellNews::Clouds`): a
//! few pale puffs over each cell, fading as the cloud clears.

use bevy::prelude::*;
use shared::spot::Spot;

use super::super::CELL_SIZE;
use super::fade::Grow;
use super::paint::Paint;
use super::place::spot_drawn;
use crate::common::DrawnGround;

/// Puffs over a cell, and their size, in cells.
const PUFFS: usize = 3;
const PUFF: f32 = 0.55;

/// Steam's pale grey.
const STEAM: LinearRgba = LinearRgba::new(0.75, 0.78, 0.8, 1.0);

/// A cloud over `cell` lasting `seconds`.
pub(super) fn cloud(paint: &mut Paint, ground: &DrawnGround, cell: (i64, i64), seconds: f32) {
    let ball = paint.shared().ball;
    let middle = spot_drawn(ground, Spot::middle_of(cell), 0.5);
    for _ in 0..PUFFS {
        let off = Vec3::new(
            paint.random() - 0.5,
            paint.random() * 0.4,
            paint.random() - 0.5,
        );
        let size = PUFF * (0.8 + 0.4 * paint.random()) * CELL_SIZE;
        let transform =
            Transform::from_translation(middle + off * CELL_SIZE).with_scale(Vec3::splat(size));
        paint.fading(ball.clone(), transform, STEAM, (seconds, Grow::Steady));
    }
}
