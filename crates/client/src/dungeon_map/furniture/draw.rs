//! Drawing a piece's parts: each cell draws the share of every upright part
//! inside it (and leaning parts whole, in the cell their middle is in), lit
//! or remembered as the cell is, with a black outline.

use bevy::prelude::*;
use shared::furniture::{Material, Piece};

use super::super::CELL_SIZE;
use super::super::mesh::{MEMORY_SHADE, Surface, corner_light, with_ambient};
use super::super::outlined::{OUTLINE_WIDTH, add_face, grown_face};
use super::super::walls::{Layer, Layers, Scene};
use super::frame::Frame;
use super::{Grain, Part, atlas, colour};

/// How much darker faces turned east or west, and north or south, are than
/// tops: it shows the corners.
const EAST_WEST_SHADE: f32 = 0.72;
const NORTH_SOUTH_SHADE: f32 = 0.86;

/// What every part of a piece is drawn with, at one cell.
struct Drawing<'a> {
    scene: &'a Scene<'a>,
    cell: (i64, i64),
    /// The cell's north-west corner.
    cell_low: Vec3,
    layer: Layer,
    tint: Vec3,
    tile: usize,
    end_tile: usize,
    /// How high the piece's level stands: its highest cell's ground.
    level: f32,
}

/// Draws the share of `parts` (of `piece`) at `(x, y)`. `beyond`: how far
/// past the cell's low and high edges its share reaches (a seat's front
/// under a table's edge).
pub(super) fn draw_parts(
    layers: &mut Layers,
    scene: &Scene,
    (piece, parts): (&Piece, &[Part]),
    (x, y): (i64, i64),
    beyond: (Vec3, Vec3),
) {
    let frame = Frame::of(piece);
    let tile = atlas::tile(piece.material);
    let drawing = Drawing {
        scene,
        cell: (x, y),
        cell_low: Vec3::new(
            (x as f32 - 0.5) * CELL_SIZE,
            0.0,
            (y as f32 - 0.5) * CELL_SIZE,
        ),
        layer: if scene.sight.is_seen(x, y) {
            Layer::Solid
        } else {
            Layer::Memory
        },
        tint: colour::of(piece.material),
        tile,
        end_tile: match piece.material {
            Material::Wood(_) => atlas::END_GRAIN,
            Material::Stone(_) => tile,
        },
        level: piece
            .cells()
            .map(|(cx, cy)| scene.ground(cx, cy))
            .fold(f32::MIN, f32::max),
    };
    let cell_high = drawing.cell_low + Vec3::new(CELL_SIZE, 0.0, CELL_SIZE);
    for part in parts {
        let (a, b) = (frame.place(part.low), frame.place(part.high));
        let (low, high) = (a.min(b), a.max(b));
        let lean = frame.turn(Vec3::new(part.lean.x, 0.0, part.lean.y));
        let grain = match part.grain {
            Grain::Along => axis_of(frame.turn(Vec3::X)),
            Grain::Across => axis_of(frame.turn(Vec3::Z)),
            Grain::Up => 1,
        };
        let world = WorldPart {
            low,
            high,
            lean,
            grain,
            shade: part.shade,
            flipped: part.flipped,
        };
        if lean != Vec3::ZERO {
            // Whole, by the piece's cell nearest its middle.
            let middle = frame.place(part.middle());
            let cell_of = |m: f32, at: i64, size: u8| {
                ((m / CELL_SIZE + 0.5).floor() as i64).clamp(at, at + i64::from(size) - 1)
            };
            if cell_of(middle.x, piece.at.0, piece.size.0) == x
                && cell_of(middle.z, piece.at.1, piece.size.1) == y
            {
                drawing.box_faces(layers, &world, (low, high));
            }
            continue;
        }
        let (from, to) = (drawing.cell_low - beyond.0, cell_high + beyond.1);
        let clipped_low = low.max(Vec3::new(from.x, low.y, from.z));
        let clipped_high = high.min(Vec3::new(to.x, high.y, to.z));
        if clipped_low.x < clipped_high.x && clipped_low.z < clipped_high.z {
            drawing.box_faces(layers, &world, (clipped_low, clipped_high));
        }
    }
}

/// A part placed in the world: its whole box, lean, and which axis its grain
/// runs along (0 `x`, 1 `y`, 2 `z`).
struct WorldPart {
    low: Vec3,
    high: Vec3,
    lean: Vec3,
    grain: usize,
    shade: f32,
    flipped: bool,
}

fn axis_of(direction: Vec3) -> usize {
    if direction.x.abs() > 0.5 { 0 } else { 2 }
}

impl Drawing<'_> {
    /// The faces of the share `(low, high)` of a part: its top and sides,
    /// leaving out sides where it was cut at the cell's edge (the next cell
    /// carries on from there).
    fn box_faces(&self, layers: &mut Layers, part: &WorldPart, (low, high): (Vec3, Vec3)) {
        // Each face: the axis it faces along, and whether the high end.
        let faces = [(1, true), (0, true), (0, false), (2, true), (2, false)];
        for (axis, at_high) in faces {
            let cut = if at_high {
                high[axis] != part.high[axis]
            } else {
                low[axis] != part.low[axis]
            };
            if cut {
                continue;
            }
            // The other two axes, going round the face.
            let (p, q) = ((axis + 1) % 3, (axis + 2) % 3);
            let corner = |dp: bool, dq: bool| {
                let mut point = if at_high { high } else { low };
                point[p] = if dp { high[p] } else { low[p] };
                point[q] = if dq { high[q] } else { low[q] };
                point
            };
            let flat = [
                corner(false, false),
                corner(true, false),
                corner(true, true),
                corner(false, true),
            ];
            let mut nominal = Vec3::ZERO;
            nominal[axis] = if at_high { 1.0 } else { -1.0 };
            self.face(layers, part, flat, nominal, (axis, p, q));
        }
    }

    /// One face, from its corners before any lean.
    fn face(
        &self,
        layers: &mut Layers,
        part: &WorldPart,
        flat: [Vec3; 4],
        nominal: Vec3,
        (axis, p, q): (usize, usize, usize),
    ) {
        let corners = flat.map(|c| if c.y >= part.high.y { c + part.lean } else { c });
        let normal = (corners[1] - corners[0])
            .cross(corners[2] - corners[0])
            .normalize_or_zero();
        let normal = if normal.dot(nominal) < 0.0 {
            -normal
        } else {
            normal
        };
        // Grain runs down the picture; across it, a wood's end shows.
        let (tile, u_axis, v_axis) = if part.grain == p {
            (self.tile, q, p)
        } else if part.grain == q {
            (self.tile, p, q)
        } else {
            (self.end_tile, p, q)
        };
        let along = |c: Vec3, k: usize| (c[k] - self.cell_low[k]) / CELL_SIZE;
        let uvs = flat.map(|c| {
            let u = along(c, u_axis);
            atlas::uv(
                tile,
                if part.flipped { 1.0 - u } else { u },
                along(c, v_axis),
            )
        });
        let shade = part.shade
            * match axis {
                0 => EAST_WEST_SHADE,
                2 => NORTH_SOUTH_SHADE,
                _ => 1.0,
            };
        let colours = corners.map(|c| self.colour(c, shade));
        let level = |c: Vec3| self.levelled(c);
        layers.builder(self.layer, Surface::Furniture).free(
            corners.map(level),
            normal,
            uvs,
            colours,
        );
        let hull = grown_face(corners, normal, OUTLINE_WIDTH).map(level);
        add_face(&mut layers.outlined.black, hull, normal);
    }

    /// The colour at a point: lit where it stands, or memory's shade (over its
    /// sepia picture) when remembered.
    fn colour(&self, at: Vec3, shade: f32) -> Vec3 {
        if self.layer == Layer::Memory {
            return Vec3::splat(MEMORY_SHADE * shade);
        }
        let Scene { level, light, .. } = *self.scene;
        let (x, y) = self.cell;
        let here = corner_light(level, light, at).unwrap_or(light.at(x, y));
        with_ambient(here) * self.tint * shade
    }

    /// A point raised so that, once draped over the ground, everything above
    /// the floor stands level at the piece's highest ground, and only feet
    /// follow the slope.
    fn levelled(&self, at: Vec3) -> Vec3 {
        if at.y <= 0.0 {
            return at;
        }
        at + Vec3::Y * (self.level - self.scene.surface(at))
    }
}
