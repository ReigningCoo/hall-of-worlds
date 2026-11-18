//! A miller's mill (owner, 2026-11-18): over its round tower, a cone of a
//! roof far taller than other roofs, with a black outline; before its door,
//! the lower end of one sail (its whip, the bar along its edge and the slats
//! between), fading out with height so the rest of the windmill is only
//! suggested. Looks only: it blocks nothing.

use bevy::prelude::*;
use shared::site::BuildingLook;

use super::super::mesh::Surface;
use super::super::outlined::{OUTLINE_WIDTH, add_face, grown_face};
use super::super::walls::{Layer, Layers, Scene};
use super::super::{CELL_SIZE, WALL_HEIGHT};
use super::{FENCE_TILE, ROOF_RISE, WALL_OUT, atlas_uv, emit, layer_colour, light_at, roof_tile};

/// How far the roof rises above the walls, and how far its eaves reach past
/// them.
pub(super) const MILL_ROOF_RISE: f32 = CELL_SIZE * 5.0;
const EAVES: f32 = CELL_SIZE * 0.2;

/// Pieces round the cone and up it, each about a cell, so each takes one
/// whole tile of the picture.
const AROUND: usize = 24;
const UP: usize = 5;

/// The sail: how far before the eaves it stands, from how high to how high
/// (where it has faded out), its width, and its bars' and slats' sizes.
const SAIL_OUT: f32 = CELL_SIZE * 0.25;
const SAIL_LOW: f32 = WALL_HEIGHT * 1.1;
const SAIL_HIGH: f32 = WALL_HEIGHT + MILL_ROOF_RISE * 0.7;
const SAIL_WIDTH: f32 = CELL_SIZE * 0.9;
const BAR: f32 = CELL_SIZE * 0.06;
const SLAT: f32 = CELL_SIZE * 0.04;
const SLAT_EVERY: f32 = CELL_SIZE * 0.3;

/// The light on the cone comes from above and the south east a little, so
/// its round shows.
const LIT_FROM: Vec3 = Vec3::new(0.3, 1.0, 0.5);

/// The middle of a mill at the foot of its walls, and its walls' reach from
/// it.
fn middle_and_reach(building: &BuildingLook) -> (Vec3, f32) {
    let x0 = building.rects.iter().map(|r| r.x).min().unwrap_or(0);
    let y0 = building.rects.iter().map(|r| r.y).min().unwrap_or(0);
    let x1 = building
        .rects
        .iter()
        .map(|r| r.x + r.width)
        .max()
        .unwrap_or(1);
    let y1 = building
        .rects
        .iter()
        .map(|r| r.y + r.height)
        .max()
        .unwrap_or(1);
    let middle = Vec3::new(
        (x0 + x1 - 1) as f32 / 2.0 * CELL_SIZE,
        0.0,
        (y0 + y1 - 1) as f32 / 2.0 * CELL_SIZE,
    );
    let half_cells = ((x1 - x0).min(y1 - y0) - 1) as f32 / 2.0;
    (middle, half_cells * CELL_SIZE + WALL_OUT)
}

/// The mill's roof and sail, over the building drawn as usual.
pub(super) fn build_mill(
    layers: &mut Layers,
    scene: &Scene,
    building: &BuildingLook,
    layer: Layer,
) {
    let (middle, reach) = middle_and_reach(building);
    let radius = reach + EAVES;
    // Its foot a little below the plain roof's top, which it hides.
    let foot = WALL_HEIGHT + ROOF_RISE * 0.25;
    let point = |turn: usize, up: usize| {
        let angle = turn as f32 / AROUND as f32 * std::f32::consts::TAU;
        let share = up as f32 / UP as f32;
        let out = radius * (1.0 - share);
        middle
            + Vec3::new(
                angle.cos() * out,
                foot + MILL_ROOF_RISE * share,
                angle.sin() * out,
            )
    };
    let tile = roof_tile(building.roof);
    let shade = layer_colour(layer);
    for turn in 0..AROUND {
        for up in 0..UP {
            let corners = [
                point(turn, up),
                point(turn + 1, up),
                point(turn + 1, up + 1),
                point(turn, up + 1),
            ];
            let normal = (corners[1] - corners[0])
                .cross(corners[3] - corners[0])
                .normalize_or_zero();
            let centre = corners.iter().copied().sum::<Vec3>() / 4.0;
            let normal = if normal.dot(centre - middle) < 0.0 {
                -normal
            } else {
                normal
            };
            let lit = 0.7 + 0.3 * normal.dot(LIT_FROM.normalize()).max(0.0);
            let uvs =
                [(0.0, 1.0), (1.0, 1.0), (1.0, 0.0), (0.0, 0.0)].map(|(u, v)| atlas_uv(tile, u, v));
            emit(layers, scene, layer, corners, normal, uvs, [shade * lit; 4]);
            if layer != Layer::SeeThrough {
                let hull = grown_face(corners, normal, OUTLINE_WIDTH);
                add_face(&mut layers.outlined.black, hull, normal);
            }
        }
    }
    build_sail(layers, scene, building, layer, (middle, radius));
}

/// The lower end of one sail, above the door, fading out going up.
fn build_sail(
    layers: &mut Layers,
    scene: &Scene,
    building: &BuildingLook,
    layer: Layer,
    (middle, radius): (Vec3, f32),
) {
    let faces = Vec3::new(building.faces.0 as f32, 0.0, building.faces.1 as f32);
    let across = Vec3::new(-faces.z, 0.0, faces.x);
    let front = middle + faces * (radius + SAIL_OUT);
    // The whip up the middle, over the door; the sail's edge to one side.
    let bars = [0.0, SAIL_WIDTH];
    for along in bars {
        let low = front + across * along + Vec3::Y * SAIL_LOW;
        let bar = Bar {
            from: low,
            to: low + Vec3::Y * (SAIL_HIGH - SAIL_LOW),
            side: across * BAR,
            depth: faces * BAR,
        };
        bar.draw(layers, scene, layer);
    }
    let mut height = SAIL_LOW + SLAT_EVERY / 2.0;
    while height < SAIL_HIGH {
        let start = front + Vec3::Y * height;
        let slat = Bar {
            from: start,
            to: start + across * SAIL_WIDTH,
            side: Vec3::Y * SLAT,
            depth: faces * SLAT,
        };
        slat.draw(layers, scene, layer);
        height += SLAT_EVERY;
    }
}

/// A square bar of the sail from `from` to `to`, `side` and `depth` its
/// half widths.
struct Bar {
    from: Vec3,
    to: Vec3,
    side: Vec3,
    depth: Vec3,
}

impl Bar {
    fn draw(&self, layers: &mut Layers, scene: &Scene, layer: Layer) {
        for (out, width) in [
            (self.depth, self.side),
            (-self.depth, self.side),
            (self.side, self.depth),
            (-self.side, self.depth),
        ] {
            let corners = [
                self.from + out - width,
                self.to + out - width,
                self.to + out + width,
                self.from + out + width,
            ];
            let normal = out.normalize_or_zero();
            let uvs = [(0.0, 0.0), (1.0, 0.0), (1.0, 0.2), (0.0, 0.2)]
                .map(|(u, v)| atlas_uv(FENCE_TILE, u, v));
            let fade = corners.map(|c| opacity(c.y));
            if fade.iter().all(|&f| f <= 0.0) {
                continue;
            }
            let shade = layer_colour(layer);
            let colours = corners.map(|c| match layer {
                Layer::Memory => shade,
                _ => shade * light_at(scene, c),
            });
            emit_faded(layers, layer, corners, normal, uvs, colours, fade);
        }
    }
}

/// How much of the sail shows at a height: all at its foot, none at its top.
fn opacity(height: f32) -> f32 {
    (1.0 - (height - SAIL_LOW) / (SAIL_HIGH - SAIL_LOW)).clamp(0.0, 1.0)
}

/// A face on the town's fading surface (in the way, the see-through one).
fn emit_faded(
    layers: &mut Layers,
    layer: Layer,
    positions: [Vec3; 4],
    normal: Vec3,
    uvs: [[f32; 2]; 4],
    colours: [Vec3; 4],
    opacity: [f32; 4],
) {
    let index = Surface::TownBlend as usize;
    let builder = match layer {
        Layer::Solid => &mut layers.solid[index],
        Layer::Memory => &mut layers.memory[index],
        Layer::SeeThrough => &mut layers.stairs_through[index],
    };
    builder.faded(positions, normal, uvs, colours, opacity);
}
