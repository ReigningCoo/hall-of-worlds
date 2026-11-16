//! A stone or dressed-block wall's cell: its run to the wall's cells beside it
//! (and on into a building's cell it meets, up to its wall's face), and the
//! parapet along its outer side or sides, away from the place's middle.

use bevy::prelude::*;
use shared::dungeon::Level;
use shared::site::WallLook;

use super::super::town;
use super::super::walls::{Layers, Scene, half_thickness};
use super::join::meets;
use super::parapet::{OVERHANG, Parapet};
use super::{Build, HALF_CELL, SIDES, block};
use crate::dungeon_map::CELL_SIZE;

/// The way out of a cell towards one of its sides.
pub(super) fn way(side: usize) -> Vec3 {
    let (dx, dy) = SIDES[side];
    Vec3::new(dx as f32, 0.0, dy as f32)
}

/// Which sides of the wall's rectangle `(x, y)` lies on: its outer sides.
pub(super) fn outer_sides(wall: &WallLook, (x, y): (i64, i64)) -> [bool; 4] {
    let rect = wall.rect;
    [
        y == rect.y,
        x == rect.x + rect.width - 1,
        y == rect.y + rect.height - 1,
        x == rect.x,
    ]
}

/// The sides at either end of a parapet along side `side`: west and east,
/// or north and south.
fn ends(side: usize) -> (usize, usize) {
    if side.is_multiple_of(2) {
        (3, 1)
    } else {
        (0, 2)
    }
}

/// The boxes of a stone wall's cell, each as its smallest and largest corners;
/// `building_half` is half the thickness of buildings' walls.
pub(super) fn stone_boxes(
    wall: &WallLook,
    level: &Level,
    build: &Build,
    building_half: f32,
    (x, y): (i64, i64),
) -> Vec<(Vec3, Vec3)> {
    let centre = town::cell_centre(x, y);
    let (half, height) = (build.half, build.height);
    let reach: [Option<f32>; 4] = std::array::from_fn(|side| {
        let (dx, dy) = SIDES[side];
        let other = (x + dx, y + dy);
        if wall.has(other.0, other.1) {
            Some(HALF_CELL)
        } else if meets(wall, level, (x, y), other) {
            Some(CELL_SIZE - building_half)
        } else {
            None
        }
    });
    // The middle, then arms out to where each side's run ends, as thick as
    // the wall across.
    let around = |to: Vec3, margin: Vec3| {
        let high = centre.max(to) + margin + Vec3::Y * height;
        (centre.min(to) - margin, high)
    };
    let mut boxes = vec![around(centre, Vec3::new(half, 0.0, half))];
    for side in 0..4 {
        if let Some(reach) = reach[side] {
            let across = (Vec3::new(1.0, 0.0, 1.0) - way(side).abs()) * half;
            boxes.push(around(centre + way(side) * reach, across));
        }
    }
    let outer = outer_sides(wall, (x, y));
    let extent = |side: usize| {
        if outer[side] {
            half + OVERHANG
        } else {
            reach[side].unwrap_or(half)
        }
    };
    for side in (0..4).filter(|&side| outer[side]) {
        let (before, after) = ends(side);
        let parapet = Parapet {
            foot: centre + Vec3::Y * height,
            outward: way(side),
            face: half,
            span: (-extent(before), extent(after)),
        };
        boxes.extend(parapet.boxes());
    }
    boxes
}

/// Draws a stone wall's cell.
pub(super) fn build_stone_run(
    layers: &mut Layers,
    scene: &Scene,
    wall: &WallLook,
    build: &Build,
    (x, y): (i64, i64),
) {
    let layer = town::flat_layer(scene, x, y);
    let building_half = half_thickness(scene.shape, 0.0);
    for corners in stone_boxes(wall, scene.level, build, building_half, (x, y)) {
        block(layers, scene, layer, build, corners);
    }
}
