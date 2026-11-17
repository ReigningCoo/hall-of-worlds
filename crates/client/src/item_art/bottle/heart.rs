//! The heart bottle's body: a heart's outline seen from the front, puffed out
//! front and back like a pillow. Its neck is lathed (`profiles.rs`), rising
//! from the dip between the lobes.

use bevy::prelude::*;
use std::f32::consts::{FRAC_PI_2, PI, TAU};

use super::surface::{AROUND, Surface};

/// Rows from the back's middle to the front's.
const ROWS: usize = 14;
/// How far it bulges out front and back.
const DEPTH: f32 = 15.0;
/// The heart's size against its usual outline, which is 32 across.
const SCALE: f32 = 2.2;
/// The point it is puffed out from, between the lobes and the tip.
const MIDDLE: Vec2 = Vec2::new(0.0, 30.0);

/// A point on the heart's outline, its tip on the ground at the middle.
fn outline(t: f32) -> Vec2 {
    let x = 16.0 * t.sin().powi(3);
    let y = 13.0 * t.cos() - 5.0 * (2.0 * t).cos() - 2.0 * (3.0 * t).cos() - (4.0 * t).cos();
    // The tip is at y = -17 when t is half way round.
    Vec2::new(x, y + 17.0) * SCALE
}

/// How high the dip between the lobes is, where the neck comes out.
#[cfg(test)]
pub(super) fn dip() -> f32 {
    outline(0.0).y
}

/// The pillow round the outline.
pub(super) fn heart_body() -> Surface {
    let edge: Vec<Vec2> = (0..AROUND)
        .map(|i| outline(i as f32 / AROUND as f32 * TAU))
        .collect();
    let mut rows = Vec::new();
    for row in 0..=ROWS {
        let v = -FRAC_PI_2 + row as f32 / ROWS as f32 * PI;
        let ring = edge
            .iter()
            .map(|&p| {
                let at = MIDDLE + (p - MIDDLE) * v.cos();
                Vec3::new(at.x, at.y, DEPTH * v.sin())
            })
            .collect();
        rows.push(ring);
    }
    let faceted = vec![0.0; rows.len()];
    Surface { rows, faceted }
}
