//! Battlements on a building's top storey (castle battlements PR 2): the
//! rampart's parapet (`rampart::Parapet`) along every side of a wall's top that
//! looks out over open air (`Cell::Air`, never a ruin's `Cell::Hole`), where
//! no storey stands on the wall (`Scene::above`).

use bevy::prelude::*;
use shared::dungeon::Cell;

use super::super::WALL_HEIGHT;
use super::super::mesh::Surface;
use super::super::rampart::{OVERHANG, Parapet, THICKNESS};
use super::{HALF_CELL, Layer, Layers, QUARTERS, SIDEWAYS_SHADE, TOP_SHADE, Tile, half_thickness};

/// The four ways out of a cell (`y` south): north, east, south, west.
const SIDES: [(i64, i64); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

/// Which sides of the wall at `(x, y)` get a parapet: those over open air,
/// on a wall with nothing standing on it.
pub(super) fn battlement_sides(tile: &Tile, (x, y): (i64, i64)) -> [bool; 4] {
    let scene = tile.scene;
    let open = scene.above.is_open(x, y);
    SIDES.map(|(dx, dy)| open && scene.level.cell(x + dx, y + dy) == Cell::Air)
}

/// The parapets of the wall at `(x, y)` (its arms `joins`, as north, east,
/// south, west), each running the length of its side, mitred where two meet
/// at an outer corner, and meeting round an inner one.
pub(super) fn battlements(tile: &Tile, (x, y): (i64, i64)) -> Vec<Parapet> {
    let outer = battlement_sides(tile, (x, y));
    let half = half_thickness(tile.scene.shape, WALL_HEIGHT);
    let extent = |side: usize| {
        if outer[side] {
            half + OVERHANG
        } else if tile.joins[side] {
            HALF_CELL
        } else {
            half
        }
    };
    let foot = tile.centre + Vec3::Y * WALL_HEIGHT;
    let mut parapets: Vec<Parapet> = (0..4)
        .filter(|&side| outer[side])
        .map(|side| {
            let (dx, dy) = SIDES[side];
            let (before, after) = if side % 2 == 0 { (3, 1) } else { (0, 2) };
            Parapet {
                foot,
                outward: Vec3::new(dx as f32, 0.0, dy as f32),
                face: half,
                span: (-extent(before), extent(after)),
            }
        })
        .collect();
    // An inner corner (open air only round the corner, the wall running on
    // both ways): the parapets of the walls beside it come on into this cell
    // until they meet.
    if tile.scene.above.is_open(x, y) {
        for (sx, sz) in QUARTERS {
            let sides = (if sx > 0.0 { 1 } else { 3 }, if sz > 0.0 { 2 } else { 0 });
            let corner = (x + sx as i64, y + sz as i64);
            if !tile.joins[sides.0]
                || !tile.joins[sides.1]
                || tile.scene.level.cell(corner.0, corner.1) != Cell::Air
            {
                continue;
            }
            let reach = |sign: f32| {
                let (a, b) = (sign * HALF_CELL, sign * (half + OVERHANG - THICKNESS));
                (a.min(b), a.max(b))
            };
            parapets.push(Parapet {
                foot,
                outward: Vec3::X * sx,
                face: half,
                span: reach(sz),
            });
            parapets.push(Parapet {
                foot,
                outward: Vec3::Z * sz,
                face: half,
                span: reach(sx),
            });
        }
    }
    parapets
}

impl Tile<'_> {
    /// Draws the battlements on this wall's top, if it has any.
    pub(super) fn build_battlements(&self, layers: &mut Layers) {
        let parapets = battlements(self, (self.x, self.y));
        if parapets.is_empty() {
            return;
        }
        let in_sight = QUARTERS
            .iter()
            .any(|&quarter| self.quarter_shows(self.scene.sight, quarter));
        let layer = if in_sight {
            Layer::Solid
        } else {
            Layer::Memory
        };
        for parapet in &parapets {
            for corners in parapet.boxes() {
                self.wall_box(layers, layer, corners);
            }
        }
    }

    /// A box of the wall's picture from `low` to `high` (its bottom isn't
    /// drawn), each face showing as much of the picture as it is big.
    fn wall_box(&self, layers: &mut Layers, layer: Layer, (l, h): (Vec3, Vec3)) {
        let size = (h - l) / (2.0 * HALF_CELL);
        let uvs = |u: f32, v: f32| [[0.0, v], [u, v], [u, 0.0], [0.0, 0.0]];
        let at = Vec3::new;
        let faces = [
            (
                [
                    at(l.x, h.y, h.z),
                    at(h.x, h.y, h.z),
                    at(h.x, h.y, l.z),
                    at(l.x, h.y, l.z),
                ],
                Vec3::Y,
                TOP_SHADE,
                uvs(size.x, size.z),
            ),
            (
                [
                    at(l.x, l.y, h.z),
                    at(h.x, l.y, h.z),
                    at(h.x, h.y, h.z),
                    at(l.x, h.y, h.z),
                ],
                Vec3::Z,
                1.0,
                uvs(size.x, size.y),
            ),
            (
                [
                    at(h.x, l.y, l.z),
                    at(l.x, l.y, l.z),
                    at(l.x, h.y, l.z),
                    at(h.x, h.y, l.z),
                ],
                Vec3::NEG_Z,
                1.0,
                uvs(size.x, size.y),
            ),
            (
                [
                    at(h.x, l.y, h.z),
                    at(h.x, l.y, l.z),
                    at(h.x, h.y, l.z),
                    at(h.x, h.y, h.z),
                ],
                Vec3::X,
                SIDEWAYS_SHADE,
                uvs(size.z, size.y),
            ),
            (
                [
                    at(l.x, l.y, l.z),
                    at(l.x, l.y, h.z),
                    at(l.x, h.y, h.z),
                    at(l.x, h.y, l.z),
                ],
                Vec3::NEG_X,
                SIDEWAYS_SHADE,
                uvs(size.z, size.y),
            ),
        ];
        for (corners, normal, shade, uvs) in faces {
            let corners = [0, 1, 2, 3].map(|i| (corners[i], uvs[i], shade));
            self.add(layers, layer, Surface::Wall, normal, corners);
        }
    }
}

#[cfg(test)]
mod tests;
