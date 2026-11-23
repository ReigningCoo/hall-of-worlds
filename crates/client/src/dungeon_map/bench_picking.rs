//! A click (or tap) on a piece with places (`furniture::Holder`: a crafting
//! bench, cask, wine or spirit rack, the last standing on a table or bar
//! counter, or a bar counter), or what
//! lies on it: each is taken as a box over its
//! cells, from the ground to above its shelf's tools or its casks.
//! `click.rs` sends `BenchClicked` for it; `hover.rs` names it.

use bevy::prelude::*;
use shared::dungeon::{Level, Sight};
use shared::furniture::{Holder, Piece};

use super::CELL_SIZE;
use super::ground_hit::Ground;
use super::mesh::WallFace;
use super::picking::hits_face;

/// How high a bench's box reaches: its shelf and the tallest tool on it.
const BENCH_HEIGHT: f32 = CELL_SIZE * 0.95;

/// The remembered crafting bench the ray reaches first, if no solid wall
/// piece and no ground is in the way.
pub(super) fn bench_clicked(
    ray: Ray3d,
    level: &Level,
    seen: &Sight,
    solid_faces: &[WallFace],
    ground: Ground,
) -> Option<Piece> {
    let floor = (ground.hit(ray)).map_or(f32::INFINITY, |(distance, _, _)| distance);
    let (distance, piece) = (level.furniture().chain(level.furniture_on_top()))
        .filter(|piece| Holder::of(piece.furniture).is_some())
        .filter(|piece| piece.cells().any(|(x, y)| seen.is_seen(x, y)))
        .filter_map(|piece| {
            let foot = (piece.cells().map(|(x, y)| ground.at(x, y))).fold(f32::MIN, f32::max);
            let (x, y) = piece.at;
            let low = Vec3::new(x as f32 - 0.5, 0.0, y as f32 - 0.5) * CELL_SIZE;
            let (w, h) = (f32::from(piece.size.0), f32::from(piece.size.1));
            let high = low + Vec3::new(w * CELL_SIZE, 0.0, h * CELL_SIZE);
            let low = low.with_y(foot);
            let tall = (super::furniture::rack_height(piece))
                .or_else(|| super::furniture::bottle_rack_height(piece))
                .or_else(|| super::furniture::bar_height(piece))
                .or_else(|| storage_height(piece))
                .unwrap_or(BENCH_HEIGHT);
            // A spirit rack on a counter stands that much higher.
            let under = level.furniture_at(piece.at.0, piece.at.1);
            let lifted = piece.furniture.stands_on_furniture();
            let tall = tall
                + under
                    .filter(|_| lifted)
                    .map_or(0.0, super::furniture::lift_on);
            let high = high.with_y(foot + tall);
            Some((into_box(ray, low, high)?, *piece))
        })
        .filter(|&(distance, _)| distance <= floor + CELL_SIZE * 0.1)
        .min_by(|a, b| a.0.total_cmp(&b.0))?;
    let direction = *ray.direction;
    let blocked = (solid_faces.iter())
        .any(|face| hits_face(ray.origin, direction, face).is_some_and(|d| d < distance));
    (!blocked).then_some(piece)
}

/// How tall a chest or wardrobe is, for picking.
fn storage_height(piece: &Piece) -> Option<f32> {
    match piece.furniture {
        shared::furniture::Furniture::Chest { .. } => Some(super::decor::CHEST_HEIGHT),
        shared::furniture::Furniture::Wardrobe { .. } => Some(super::decor::WARDROBE_HEIGHT),
        _ => None,
    }
}

/// How far along the ray it first goes into the box from `low` to `high`.
fn into_box(ray: Ray3d, low: Vec3, high: Vec3) -> Option<f32> {
    let direction = *ray.direction;
    let (mut near, mut far) = (0.0f32, f32::INFINITY);
    for axis in 0..3 {
        let (origin, step) = (ray.origin[axis], direction[axis]);
        if step.abs() <= f32::EPSILON {
            if origin < low[axis] || origin > high[axis] {
                return None;
            }
            continue;
        }
        let (a, b) = ((low[axis] - origin) / step, (high[axis] - origin) / step);
        near = near.max(a.min(b));
        far = far.min(a.max(b));
    }
    (near <= far).then_some(near)
}

#[cfg(test)]
mod tests;
