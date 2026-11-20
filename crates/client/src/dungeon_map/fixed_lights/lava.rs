//! Lava as fixed lights: pools glow orange from their rim, the lava cells with
//! at least one cell of something else among their eight neighbours (walls
//! included). Inner lava gives no light of its own: a rim cell is always
//! nearer anything it could light, and lights of one colour don't add up, so a
//! big pool is no brighter than a strip.

use bevy::prelude::*;
use shared::dungeon::{Cell, Level};

use super::FixedLight;

/// How far lava's glow reaches, in cells.
const LAVA_RADIUS: u32 = 4;

/// Lava's glow as linear RGB: a deep orange.
const LAVA_GLOW: Vec3 = Vec3::new(1.0, 0.45, 0.1);

/// The lights the level's lava gives: one at each rim cell.
pub(super) fn lava_lights(level: &Level) -> Vec<FixedLight> {
    let (min_x, min_y, width, height) = level.bounds();
    let mut lights = Vec::new();
    for y in min_y..min_y + height {
        for x in min_x..min_x + width {
            if level.cell(x, y) == Cell::Lava && is_rim(level, x, y) {
                lights.push(FixedLight {
                    cell: (x, y),
                    radius: LAVA_RADIUS,
                    colour: LAVA_GLOW,
                });
            }
        }
    }
    lights
}

/// Whether any of the eight cells round `(x, y)` is not lava.
fn is_rim(level: &Level, x: i64, y: i64) -> bool {
    (-1..=1).any(|dy| (-1..=1).any(|dx| level.cell(x + dx, y + dy) != Cell::Lava))
}
