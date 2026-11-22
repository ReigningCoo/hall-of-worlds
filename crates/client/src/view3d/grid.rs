//! `ViewGrid`: how the current view lays out its grid in 3D.

use bevy::prelude::*;

use crate::common::{DrawnGround, HallBend};

/// How the current view lays out its grid in 3D, set by the view when it
/// becomes current. Grid position `(x, y)` is drawn at
/// `(x × cell_size, floor, y × cell_size)`; the camera looks at the floor and
/// tokens stand on it. `raised`: the wilderness's cells are raised by their
/// ground's shape (`common::DrawnGround`, flat in buildings), and so are the
/// camera and tokens. In the Hall of Worlds everything is drawn bent
/// (`common::HallBend`): `at` and `floor_at` say where, and the bend's
/// `unshift` and `unbend_ray` lead back.
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct ViewGrid {
    pub cell_size: f32,
    pub floor: f32,
    pub raised: bool,
}

impl ViewGrid {
    /// The floor's height at grid position `(x, y)`, in world units: raised
    /// by the ground's shape, and by the Hall's bend.
    pub fn floor_at(&self, heights: &DrawnGround, bend: &HallBend, x: f32, y: f32) -> f32 {
        let ground = match self.raised {
            true => heights.drawn_between(x, y) * self.cell_size,
            false => 0.0,
        };
        self.floor + ground + bend.offset(x).y * self.cell_size
    }

    /// Where grid position `(x, y)` is drawn on the floor, in world units:
    /// on its ground, and moved across the Hall by its bend.
    pub fn at(&self, heights: &DrawnGround, bend: &HallBend, x: f32, y: f32) -> Vec3 {
        let across = (y + bend.offset(x).x) * self.cell_size;
        Vec3::new(
            x * self.cell_size,
            self.floor_at(heights, bend, x, y),
            across,
        )
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
