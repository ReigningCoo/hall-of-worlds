//! The Hall of Worlds drawn bent: the walkway is perfectly straight by the
//! rules, but drawn swinging gently sideways and up and down along its length,
//! by each column's place in the whole Hall. Drawing only: never the rules,
//! the server, steps or `DrawnGround` (which moves are priced on).

use bevy::prelude::*;
use shared::hall::HALL_LENGTH;
use shared::rng::Seed;

/// The sideways swing's layers: columns between their knots, and the most
/// each moves the walkway either way, in cells. Each layer turns no faster
/// than `3 × most ÷ columns` a column (smoothstep's steepest is 1.5 × the
/// difference between two knots), so together under 1/32 of a cell, a pixel
/// of the marble's picture.
const SIDEWAYS: [(i64, f32); 2] = [(256, 1.1), (64, 0.35)];
/// The same for the height: under 1/64 of a cell a column.
const UP: [(i64, f32); 2] = [(256, 0.3), (64, 0.1)];
/// Where the knots' values come from (any fixed number: the same Hall for all).
const SEED: Seed = Seed::new(0x4841_4c4c_4245_4e44);

/// Whether the level shown is drawn bent, and where its stretch is in the
/// Hall: on in the Hall of Worlds (`hall_view` sets it), off elsewhere, where
/// every offset is nothing.
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

    /// How far a place at grid `x` (in cells, as the client numbers the
    /// stretch) is drawn moved: across the walkway (grid `y`, `.x` here) and
    /// up (`.y`), in cells.
    pub fn offset(&self, x: f32) -> Vec2 {
        let Some(origin) = self.origin else {
            return Vec2::ZERO;
        };
        let whole = x.floor();
        let column = (whole as i64).wrapping_add(origin).rem_euclid(HALL_LENGTH);
        let part = x - whole;
        Vec2::new(
            swing(&SIDEWAYS, 0, column, part),
            swing(&UP, 1, column, part),
        )
    }

    /// A place in the view's world units (`cell` units a cell) moved to where
    /// it is drawn.
    pub fn shift(&self, at: Vec3, cell: f32) -> Vec3 {
        let offset = self.offset(at.x / cell) * cell;
        at + Vec3::new(0.0, offset.y, offset.x)
    }

    /// A place as drawn back where it is by the rules: exact, as the offset
    /// goes by `x` alone, which it never moves.
    pub fn unshift(&self, at: Vec3, cell: f32) -> Vec3 {
        let offset = self.offset(at.x / cell) * cell;
        at - Vec3::new(0.0, offset.y, offset.x)
    }

    /// A ray from the camera into the drawn view turned into one into the
    /// straight Hall: through the place on the straight floor (at `floor`)
    /// under where it meets the bent one, the same way. Things standing on the
    /// floor are reached nearly where they are drawn, as the walkway turns so
    /// slowly. Unchanged when not bent, or looking up.
    pub fn unbend_ray(&self, ray: Ray3d, cell: f32, floor: f32) -> Ray3d {
        let down = ray.direction.y;
        if self.origin.is_none() || down >= -1e-6 {
            return ray;
        }
        // Where the ray meets the bent floor: the offset changes so slowly
        // along it that a few rounds settle it.
        let mut t = (floor - ray.origin.y) / down;
        for _ in 0..4 {
            let x = ray.origin.x + ray.direction.x * t;
            t = (floor + self.offset(x / cell).y * cell - ray.origin.y) / down;
        }
        let straight = self.unshift(ray.origin + ray.direction * t, cell);
        Ray3d::new(straight - ray.direction * t, ray.direction)
    }
}

/// The layers' swings added up at a column (`part` of the way to the next).
fn swing(layers: &[(i64, f32)], which: i64, column: i64, part: f32) -> f32 {
    (layers.iter().zip(0..))
        .map(|(&(every, most), layer)| {
            let knots = HALL_LENGTH / every;
            let knot = column / every;
            let along = ((column % every) as f32 + part) / every as f32;
            let eased = along * along * (3.0 - 2.0 * along);
            let from = knot_value(which, layer, knot);
            let to = knot_value(which, layer, (knot + 1) % knots);
            most * (from + (to - from) * eased)
        })
        .sum()
}

/// A knot's value, from -1 to 1.
fn knot_value(which: i64, layer: i64, knot: i64) -> f32 {
    let hash = SEED.at(&[which, layer, knot]);
    (hash >> 40) as f32 / (1u64 << 23) as f32 - 1.0
}

#[cfg(test)]
mod tests;
