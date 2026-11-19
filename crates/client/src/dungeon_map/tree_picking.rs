//! A click (or tap) on a wild tree's model means the tree's own cell, as for
//! fruit plants (`fruit_picking`): each tree is taken to be an upright
//! cylinder as tall and wide as it is drawn (`plants::tree_size`), the nearest
//! the ray goes into winning. Hover names it by its kind and how it has grown
//! back; a click sends the player to the walkable cell beside it.

use bevy::prelude::*;
use shared::dungeon::{Cell, Level, Sight};
use shared::tree::TreeStage;

use super::CELL_SIZE;
use super::fruit_picking::into_cylinder;
use super::ground_hit::Ground;
use super::mesh::WallFace;
use super::picking::hits_face;
use crate::plants::tree_size;

/// The seen wild tree whose model the ray reaches first, if no solid wall
/// piece and no ground is in the way.
pub fn tree_clicked(
    ray: Ray3d,
    level: &Level,
    sight: &Sight,
    solid_faces: &[WallFace],
    ground: Ground,
) -> Option<(i64, i64)> {
    let floor = ground
        .hit(ray)
        .map_or(f32::INFINITY, |(distance, _, _)| distance);
    let (min_x, min_y, width, height) = level.bounds();
    let (distance, cell) = (min_y..min_y + height)
        .flat_map(|y| (min_x..min_x + width).map(move |x| (x, y)))
        .filter(|&(x, y)| level.cell(x, y) == Cell::Tree && sight.is_seen(x, y))
        .filter_map(|(x, y)| {
            let kind = level.tree_kind_at(x, y)?;
            let stage = level.felled_at(x, y).unwrap_or(TreeStage::Grown);
            let (high, radius) = tree_size(kind, stage)?;
            let foot = Vec3::new(x as f32 * CELL_SIZE, ground.at(x, y), y as f32 * CELL_SIZE);
            Some((into_cylinder(ray, foot, high, radius)?, (x, y)))
        })
        .filter(|&(distance, _)| distance <= floor + CELL_SIZE * 0.1)
        .min_by(|a, b| a.0.total_cmp(&b.0))?;
    let direction = *ray.direction;
    let blocked = solid_faces
        .iter()
        .any(|face| hits_face(ray.origin, direction, face).is_some_and(|d| d < distance));
    (!blocked).then_some(cell)
}

/// What the wild tree at `(x, y)` is called on hover: its kind, and how it has
/// grown back if felled ("Oak", "Oak, stump", "Oak, sapling").
pub(super) fn tree_name(level: &Level, (x, y): (i64, i64)) -> Option<String> {
    if level.cell(x, y) != Cell::Tree {
        return None;
    }
    let name = level.tree_kind_at(x, y)?.name();
    Some(match level.felled_at(x, y) {
        None | Some(TreeStage::Grown) => name.to_owned(),
        Some(stage) => format!("{name}, {}", stage_name(stage)),
    })
}

/// How a felled tree has grown back, in words.
pub(super) fn stage_name(stage: TreeStage) -> &'static str {
    match stage {
        TreeStage::Stump => "stump",
        TreeStage::Cleared => "cleared",
        TreeStage::Sapling => "sapling",
        TreeStage::Young => "young",
        TreeStage::HalfGrown => "half grown",
        TreeStage::Grown => "grown",
    }
}
