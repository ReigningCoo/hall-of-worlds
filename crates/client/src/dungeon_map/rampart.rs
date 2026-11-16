//! Walls round sites (`shared::site::WallLook`: a walled fort's), drawn by
//! what they are made of rather than the place's theme, and lower than a
//! building's walls: logs side by side, boards on posts, rough stone or dressed
//! blocks with a parapet along the outer edge of their top (`parapet`). Pieces go in the town mesh (atlas pictures, lit
//! where they stand, like fences) and are draped over the ground. A gate's two
//! doors are drawn as one gate: a post at each side and the two leaves
//! (`walls::build_gate_leaf`) meeting in the middle; between two towers, a
//! gatehouse (`gatehouse`) in place of the posts.

use shared::site::{WallLook, WallMaterial};

use super::CELL_SIZE;
use super::WALL_HEIGHT;
use super::walls::{Layers, Scene, build_gate_leaf};

// Split into files (`cargo xtask lint` keeps files small).
mod block;
mod gatehouse;
mod join;
mod parapet;
mod pieces;
mod stone;

use block::block;
use gatehouse::build_gatehouse;
pub use join::without_ramparts;
pub(super) use parapet::{OVERHANG, Parapet, THICKNESS};
use pieces::{build_gate_post, build_run};

const HALF_CELL: f32 = CELL_SIZE / 2.0;

/// The four ways out of a cell, as steps (`y` south): north, east, south,
/// west.
const SIDES: [(i64, i64); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

/// Atlas tiles (`town::atlas`) for each material: fence wood, boards, the
/// stone wall, dressed stone.
const LOG_TILE: usize = 45;
const PLANK_TILE: usize = 2;
const STONE_TILE: usize = 12;
const BLOCK_TILE: usize = 5;

/// Sides are a little darker than tops, and wood darker than its picture.
const SIDE_SHADE: f32 = 0.8;
const LOG_SHADE: f32 = 0.75;

/// How a material's wall is built: its height, half its thickness, and its
/// picture.
struct Build {
    height: f32,
    half: f32,
    tile: usize,
    shade: f32,
}

impl Build {
    fn of(material: WallMaterial) -> Self {
        let (height, half, tile, shade) = match material {
            WallMaterial::Log => (0.9, 0.15, LOG_TILE, LOG_SHADE),
            WallMaterial::Plank => (0.85, 0.07, PLANK_TILE, 1.0),
            WallMaterial::Stone => (0.85, 0.3, STONE_TILE, 1.0),
            WallMaterial::Block => (0.95, 0.3, BLOCK_TILE, 1.0),
        };
        Self {
            height: height * WALL_HEIGHT,
            half: half * CELL_SIZE,
            tile,
            shade,
        }
    }
}

/// The wall round a site that `(x, y)` is part of, if any.
fn wall_at<'a>(scene: &Scene<'a>, x: i64, y: i64) -> Option<&'a WallLook> {
    scene.walls.iter().find(|wall| wall.has(x, y))
}

/// Draws the cell at `(x, y)` if it is part of a wall round a site (its gates
/// too): whether it was.
pub fn build_rampart(layers: &mut Layers, scene: &Scene, (x, y): (i64, i64)) -> bool {
    let Some(wall) = wall_at(scene, x, y) else {
        return false;
    };
    if wall.is_gate(x, y) {
        build_gate_leaf(layers, scene, (x, y));
        match gatehouse::gatehouse_gate(wall, scene.level, (x, y)) {
            Some(gate) => build_gatehouse(layers, scene, wall, gate, (x, y)),
            None => build_gate_post(layers, scene, wall, (x, y)),
        }
    } else {
        build_run(layers, scene, wall, (x, y));
    }
    true
}

#[cfg(test)]
mod tests;
