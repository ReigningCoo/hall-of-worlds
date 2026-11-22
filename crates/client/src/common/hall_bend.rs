//! The Hall of Worlds drawn bent and twisted: the walkway is perfectly
//! straight and flat by the rules, but drawn swinging gently sideways and up
//! and down along its length, and turned round its middle line, by each
//! column's place in the whole Hall. Drawing only: never the rules, the
//! server, steps or `DrawnGround` (which moves are priced on).
//!
//! The twist here is the whole Hall's, the same for everyone; the view rolls
//! by the twist at the player's column (`tilt`), so the floor under the player
//! is always level on screen and the Hall rolls away ahead and behind
//! (`view3d`). Meshes are moved once, never as the player walks.

use bevy::prelude::*;
use shared::hall::{HALL_LENGTH, WALKWAY_ROWS};

mod noise;
mod ray;

use noise::{SIDEWAYS, TWIST, UP, swing};

/// The walkway's middle line, which the twist turns round: across, in cells
/// (the middle row), on the straight floor.
const MIDDLE: f32 = (WALKWAY_ROWS - 1) as f32 / 2.0;

/// Whether the level shown is drawn bent, and where its stretch is in the
/// Hall: on in the Hall of Worlds (`hall_view` sets it), off elsewhere, where
/// every offset and twist is nothing.
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct HallBend {
    /// The Hall's column of the stretch's column 0 (`LevelKind::Hall`'s).
    origin: Option<i64>,
}

impl HallBend {
    /// Bent, for a stretch of the Hall whose column 0 is the Hall's `origin`.
    pub fn on(origin: i64) -> Self {
        Self {
            origin: Some(origin),
        }
    }

    /// The Hall's column at grid `x` and how far on towards the next, or
    /// nothing when not bent.
    fn column(&self, x: f32) -> Option<(i64, f32)> {
        let origin = self.origin?;
        let whole = x.floor();
        let column = (whole as i64).wrapping_add(origin).rem_euclid(HALL_LENGTH);
        Some((column, x - whole))
    }

    /// How far a place at grid `x` (in cells, as the client numbers the
    /// stretch) is drawn moved: across the walkway (grid `y`, `.x` here) and
    /// up (`.y`), in cells.
    pub fn offset(&self, x: f32) -> Vec2 {
        self.column(x).map_or(Vec2::ZERO, |(column, part)| {
            Vec2::new(
                swing(&SIDEWAYS, 0, column, part),
                swing(&UP, 1, column, part),
            )
        })
    }

    /// How far the walkway is turned round its middle line at grid `x`, in
    /// radians (the whole Hall's: the view rolls by the player's own).
    pub fn twist(&self, x: f32) -> f32 {
        self.column(x).map_or(0.0, |(column, part)| {
            swing(&TWIST, 2, column, part).to_radians()
        })
    }

    /// The twist at grid `x` as a rotation (about the walkway's length):
    /// what turns the floor's up there, and the view's roll at the player.
    pub fn tilt(&self, x: f32) -> Quat {
        Quat::from_rotation_x(self.twist(x))
    }

    /// Where the middle line the twist turns round is, at a place `at` in the
    /// view's world units (`cell` units a cell), before it is moved.
    fn axis(at: Vec3, cell: f32) -> Vec3 {
        Vec3::new(at.x, 0.0, MIDDLE * cell)
    }

    /// How far a place at world `x` is moved, in world units.
    fn moved(&self, x: f32, cell: f32) -> Vec3 {
        let offset = self.offset(x / cell) * cell;
        Vec3::new(0.0, offset.y, offset.x)
    }

    /// A place in the view's world units (`cell` units a cell) turned round
    /// the middle line and moved to where it is drawn. Its `x` never changes.
    pub fn shift(&self, at: Vec3, cell: f32) -> Vec3 {
        let axis = Self::axis(at, cell);
        axis + self.tilt(at.x / cell) * (at - axis) + self.moved(at.x, cell)
    }

    /// A place as drawn back where it is by the rules: exact, as neither the
    /// turn nor the offset moves `x`, which they go by.
    pub fn unshift(&self, at: Vec3, cell: f32) -> Vec3 {
        let axis = Self::axis(at, cell);
        let unmoved = at - self.moved(at.x, cell);
        axis + self.tilt(at.x / cell).inverse() * (unmoved - axis)
    }
}

#[cfg(test)]
mod tests;
