//! A click (or tap) on a fruit plant's model means the plant's own cell. A
//! tree's crown stands well above the ground, so where a click on it reaches
//! the floor is usually a cell or two behind. Each plant is taken to be an
//! upright cylinder as tall and wide as its model is drawn grown
//! (`plants::fruit_plant_size`); the nearest the ray goes into wins.

use bevy::prelude::*;
use shared::dungeon::{Level, Sight};
use shared::fruit::Fruits;

use super::CELL_SIZE;
use super::ground_hit::Ground;
use super::mesh::WallFace;
use super::picking::hits_face;
use crate::plants::fruit_plant_size;

/// The seen fruit plant whose model the ray reaches first, if no solid wall
/// piece and no ground is in the way.
pub fn fruit_clicked(
    ray: Ray3d,
    level: &Level,
    sight: &Sight,
    solid_faces: &[WallFace],
    ground: Ground,
) -> Option<(i64, i64)> {
    let fruits = Fruits::built_in();
    let floor = ground
        .hit(ray)
        .map_or(f32::INFINITY, |(distance, _, _)| distance);
    let (distance, cell) = (level.fruit_plants())
        .filter(|&((x, y), _)| sight.is_seen(x, y))
        .filter_map(|((x, y), plant)| {
            let (height, radius) = fruit_plant_size(fruits.get(&plant.fruit)?);
            let foot = Vec3::new(x as f32 * CELL_SIZE, ground.at(x, y), y as f32 * CELL_SIZE);
            let distance = into_cylinder(ray, foot, height, radius)?;
            Some((distance, (x, y)))
        })
        .filter(|&(distance, _)| distance <= floor + CELL_SIZE * 0.1)
        .min_by(|a, b| a.0.total_cmp(&b.0))?;
    let direction = *ray.direction;
    let blocked = solid_faces
        .iter()
        .any(|face| hits_face(ray.origin, direction, face).is_some_and(|d| d < distance));
    (!blocked).then_some(cell)
}

/// How far along the ray it first goes into the upright cylinder standing on
/// `foot`, `height` tall and `radius` round, if it does.
pub(super) fn into_cylinder(ray: Ray3d, foot: Vec3, height: f32, radius: f32) -> Option<f32> {
    let (origin, direction) = (ray.origin - foot, *ray.direction);
    let flat = Vec2::new(direction.x, direction.z);
    let from = Vec2::new(origin.x, origin.z);
    let a = flat.length_squared();
    let inside = |t: f32| {
        let y = origin.y + direction.y * t;
        (t >= 0.0 && (0.0..=height).contains(&y)).then_some(t)
    };
    // Through the top: where the ray comes down to the top's height.
    let top = (direction.y < 0.0)
        .then(|| (height - origin.y) / direction.y)
        .filter(|&t| (from + flat * t).length() <= radius)
        .and_then(inside);
    if a <= f32::EPSILON {
        return top;
    }
    let b = 2.0 * from.dot(flat);
    let c = from.length_squared() - radius * radius;
    let disc = b * b - 4.0 * a * c;
    if disc < 0.0 {
        return None;
    }
    let side = inside((-b - disc.sqrt()) / (2.0 * a));
    match (side, top) {
        (Some(s), Some(t)) => Some(s.min(t)),
        (s, t) => s.or(t),
    }
}

#[cfg(test)]
mod tests;
