//! A gatehouse: a stone gate standing between two towers (a castle's gate
//! towers). Thick walls fill each side of it, from the leaf's hinge to the
//! tower, and the wall's walk is carried over the arch, with the parapet along
//! its outer side, up to the towers' wall tops (owner, 2026-11-16).

use bevy::prelude::*;
use shared::dungeon::Level;
use shared::site::{Gate, WallLook, WallMaterial};

use super::super::town;
use super::super::walls::{Layers, Scene};
use super::parapet::Parapet;
use super::stone::{outer_sides, way};
use super::{Build, HALF_CELL, block};
use crate::dungeon_map::WALL_HEIGHT;

/// Where the leaf is hinged, from the cell's middle (as the gate's posts).
const HINGE: f32 = HALF_CELL * 0.82;

/// The arch's top: the top of a door (`walls::DOOR_TOP`), where the walk's
/// underside starts.
const ARCH_TOP: f32 = WALL_HEIGHT * 0.78;

/// A gatehouse's walk: level with the towers' wall tops beside it.
const WALK: f32 = WALL_HEIGHT;

/// The gate `(x, y)` is in, if it stands between two towers: both its cells
/// have a tower's wall beyond them along the wall.
pub(super) fn gatehouse_gate(wall: &WallLook, level: &Level, (x, y): (i64, i64)) -> Option<Gate> {
    if !matches!(wall.material, WallMaterial::Stone | WallMaterial::Block) {
        return None;
    }
    let gate = (wall.gates.iter()).find(|gate| gate.cells().contains(&(x, y)))?;
    let (dx, dy) = if gate.runs_east_west { (1, 0) } else { (0, 1) };
    let [first, second] = gate.cells();
    let tower = |(cx, cy): (i64, i64)| {
        level.is_wall(cx, cy) && (wall.towers.iter()).any(|tower| tower.contains(cx, cy))
    };
    (tower((first.0 - dx, first.1 - dy)) && tower((second.0 + dx, second.1 + dy))).then_some(*gate)
}

/// The boxes of a gatehouse's cell `(x, y)` in `gate`, each as its smallest
/// and largest corners: the wall beside the leaf, the walk over it, and the
/// parapet on the walk's outer side.
pub(super) fn gatehouse_boxes(
    wall: &WallLook,
    gate: Gate,
    build: &Build,
    (x, y): (i64, i64),
) -> Vec<(Vec3, Vec3)> {
    let centre = town::cell_centre(x, y);
    let along = if gate.runs_east_west {
        Vec3::X
    } else {
        Vec3::Z
    };
    // Towards this cell's tower: away from the other cell.
    let outward = if (x, y) == gate.cells()[0] { -1.0 } else { 1.0 };
    let across = (Vec3::new(1.0, 0.0, 1.0) - along) * build.half;
    let piece = |(from, to): (f32, f32), (low, high): (f32, f32)| {
        let a = centre + along * outward * from - across + Vec3::Y * low;
        let b = centre + along * outward * to + across + Vec3::Y * high;
        (a.min(b), a.max(b))
    };
    let mut boxes = vec![
        piece((HINGE, HALF_CELL), (0.0, WALK)),
        piece((-HALF_CELL, HINGE), (ARCH_TOP, WALK)),
    ];
    let outer = outer_sides(wall, (x, y));
    for side in (0..4).filter(|&side| outer[side]) {
        let parapet = Parapet {
            foot: centre + Vec3::Y * WALK,
            outward: way(side),
            face: build.half,
            span: (-HALF_CELL, HALF_CELL),
        };
        boxes.extend(parapet.boxes());
    }
    boxes
}

/// Draws a gatehouse's cell (not its leaf, which `walls::build_gate_leaf`
/// draws).
pub(super) fn build_gatehouse(
    layers: &mut Layers,
    scene: &Scene,
    wall: &WallLook,
    gate: Gate,
    (x, y): (i64, i64),
) {
    let layer = town::flat_layer(scene, x, y);
    let build = Build::of(wall.material);
    for corners in gatehouse_boxes(wall, gate, &build, (x, y)) {
        block(layers, scene, layer, &build, corners);
    }
}
