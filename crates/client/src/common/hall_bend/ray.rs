//! Clicks in the bent and twisted Hall: a ray into the drawn view turned
//! into one into the straight Hall.

use bevy::prelude::*;

use super::{HallBend, MIDDLE};

/// Rounds of finding where a ray meets the drawn floor. The walkway turns so
/// slowly a column that each round lands far nearer; a few settle it.
const ROUNDS: usize = 6;

impl HallBend {
    /// A ray from the camera into the drawn view turned into one into the
    /// straight Hall: through the place on the straight floor (at `floor`)
    /// under where it meets the drawn one, its way turned back by the twist
    /// there. Things standing on the floor are reached nearly where they are
    /// drawn, as the walkway turns so slowly. Unchanged when not bent, or
    /// looking away from the floor.
    pub fn unbend_ray(&self, ray: Ray3d, cell: f32, floor: f32) -> Ray3d {
        if self.origin.is_none() || ray.direction.y >= -1e-6 {
            return ray;
        }
        // Each column's floor across is a straight line, so the drawn floor
        // there is a plane: meet it, then the plane of the column met, again.
        let mut t = (floor - ray.origin.y) / ray.direction.y;
        let mut x = ray.origin.x + ray.direction.x * t;
        for _ in 0..ROUNDS {
            let middle = self.shift(Vec3::new(x, floor, MIDDLE * cell), cell);
            let up = self.tilt(x / cell) * Vec3::Y;
            let towards = ray.direction.dot(up);
            if towards >= -1e-6 {
                return ray;
            }
            t = (middle - ray.origin).dot(up) / towards;
            x = ray.origin.x + ray.direction.x * t;
        }
        let drawn = ray.origin + ray.direction * t;
        let straight = self.unshift(drawn, cell);
        let way = Dir3::new_unchecked(self.tilt(x / cell).inverse() * *ray.direction);
        Ray3d::new(straight - way * t, way)
    }
}
