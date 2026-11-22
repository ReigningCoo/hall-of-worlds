//! `ViewGrid`: how the current view lays out its grid in 3D.

use bevy::prelude::*;

use crate::common::{DrawnGround, HallBend};

/// How the current view lays out its grid in 3D, set by the view when it
/// becomes current. Grid position `(x, y)` is drawn at
/// `(x × cell_size, floor, y × cell_size)`; the camera looks at the floor and
/// tokens stand on it. `raised`: the wilderness's cells are raised by their
/// ground's shape (`common::DrawnGround`, flat in buildings), and so are the
/// camera and tokens. In the Hall of Worlds everything is drawn bent and
/// twisted (`common::HallBend`): `at` and `floor_at` say where, and the bend's
/// `unshift` and `unbend_ray` lead back.
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct ViewGrid {
    pub cell_size: f32,
    pub floor: f32,
    pub raised: bool,
}

impl ViewGrid {
    /// The floor's height at grid position `(x, y)`, in world units: raised
    /// by the ground's shape, and by the Hall's bend and twist.
    pub fn floor_at(&self, heights: &DrawnGround, bend: &HallBend, x: f32, y: f32) -> f32 {
        self.at(heights, bend, x, y).y
    }

    /// Where grid position `(x, y)` is drawn on the floor, in world units:
    /// on its ground, and moved and turned with the Hall (`HallBend::shift`).
    pub fn at(&self, heights: &DrawnGround, bend: &HallBend, x: f32, y: f32) -> Vec3 {
        let ground = match self.raised {
            true => heights.drawn_between(x, y) * self.cell_size,
            false => 0.0,
        };
        let straight = Vec3::new(x, 0.0, y) * self.cell_size + Vec3::Y * (self.floor + ground);
        bend.shift(straight, self.cell_size)
    }
}

impl Default for ViewGrid {
    fn default() -> Self {
        Self {
            cell_size: 1.0,
            floor: 0.0,
            raised: false,
        }
    }
}

#[cfg(test)]
mod tests;
