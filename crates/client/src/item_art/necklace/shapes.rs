//! Flat shapes of metal lying on the body, drawn in their own plane (`u` across, `v`
//! down) and placed by `Place`, which lays them over the skin (or flat, for a station
//! on a strand) at a height off it: plates from outlines, bands along lines, and round
//! tubes and blobs in place for what stands up.

use std::f32::consts::TAU;

use bevy::prelude::*;

use super::path::Skin;
use crate::item_art::armour::face;
use crate::item_art::body::Triangle;

/// Where a shape's plane lies.
pub(super) enum Place<'a> {
    /// Hanging from a point in front and lying on the skin below it, or (`free`)
    /// straight down in front of it where the skin falls away.
    Hung {
        skin: &'a Skin<'a>,
        from: Vec3,
        scale: f32,
        free: bool,
    },
    /// A small flat plane at a point: `u` and `v` its ways along, `out` off it.
    Flat {
        at: Vec3,
        u: Vec3,
        v: Vec3,
        out: Vec3,
        scale: f32,
    },
}

impl Place<'_> {
    /// The point at (`u`, `v`) of the plane, `h` off the skin.
    pub fn at(&self, uv: Vec2, h: f32) -> Vec3 {
        let (point, out) = self.surface(uv);
        point + out * h
    }

    /// The way out from the plane there.
    pub fn out(&self, uv: Vec2) -> Vec3 {
        self.surface(uv).1
    }

    /// How much bigger than as made it is drawn.
    pub fn scale(&self) -> f32 {
        match self {
            Self::Hung { scale, .. } | Self::Flat { scale, .. } => *scale,
        }
    }

    fn surface(&self, uv: Vec2) -> (Vec3, Vec3) {
        match self {
            Self::Hung {
                skin,
                from,
                scale,
                free,
            } => {
                let (x, y) = (from.x + uv.x * scale, from.y - uv.y * scale);
                let (point, out) = skin.front(x, y);
                if !free || point.z >= from.z {
                    (point, out)
                } else {
                    (Vec3::new(x, y, from.z), Vec3::Z)
                }
            }
            Self::Flat {
                at,
                u,
                v,
                out,
                scale,
            } => (*at + (*u * uv.x + *v * uv.y) * *scale, *out),
        }
    }
}

/// Points round an ellipse in the plane, `steps` of them, starting at the top.
pub(super) fn ellipse(centre: Vec2, half: Vec2, steps: usize) -> Vec<Vec2> {
    (0..steps)
        .map(|step| {
            let angle = step as f32 / steps as f32 * TAU;
            centre + Vec2::new(angle.sin() * half.x, -angle.cos() * half.y)
        })
        .collect()
}

/// A plate from an outline round its middle (any point of it seen from the middle
/// once), between heights `h` off the skin.
pub(super) fn slab(tris: &mut Vec<Triangle>, place: &Place, outline: &[Vec2], h: (f32, f32)) {
    let middle = outline.iter().copied().sum::<Vec2>() / outline.len() as f32;
    let out = place.out(middle);
    let (top, bottom) = (place.at(middle, h.1), place.at(middle, h.0));
    for (index, &from) in outline.iter().enumerate() {
        let to = outline[(index + 1) % outline.len()];
        let (a, b) = (place.at(from, h.1), place.at(to, h.1));
        let (c, d) = (place.at(from, h.0), place.at(to, h.0));
        face(tris, top, a, b, out);
        face(tris, bottom, c, d, -out);
        let side = (a + b) / 2.0 - top;
        face(tris, a, b, d, side);
        face(tris, a, d, c, side);
    }
}

/// A band of half width `half` along a line in the plane, closed into a loop or not,
/// between heights `h` off the skin.
#[allow(clippy::needless_range_loop)]
pub(super) fn band(
    tris: &mut Vec<Triangle>,
    place: &Place,
    line: &[Vec2],
    half: impl Fn(usize) -> f32,
    h: (f32, f32),
    closed: bool,
) {
    let count = line.len();
    let side = |index: usize| {
        let (before, after) = if closed {
            (line[(index + count - 1) % count], line[(index + 1) % count])
        } else {
            (
                line[index.saturating_sub(1)],
                line[(index + 1).min(count - 1)],
            )
        };
        let along = (after - before).normalize_or(Vec2::X);
        Vec2::new(-along.y, along.x) * half(index)
    };
    let corners = |index: usize, height: f32| {
        let (point, across) = (line[index], side(index));
        (
            place.at(point + across, height),
            place.at(point - across, height),
        )
    };
    let spans = if closed { count } else { count - 1 };
    for index in 0..spans {
        let next = (index + 1) % count;
        let out = place.out(line[index]);
        let ((a, b), (c, d)) = (corners(index, h.1), corners(next, h.1));
        face(tris, a, c, d, out);
        face(tris, a, d, b, out);
        let ((e, f), (g, k)) = (corners(index, h.0), corners(next, h.0));
        face(tris, e, g, k, -out);
        face(tris, e, k, f, -out);
        // The two edges.
        let across = place.at(line[index] + side(index), h.1) - place.at(line[index], h.1);
        face(tris, a, c, g, across);
        face(tris, a, g, e, across);
        face(tris, b, d, k, -across);
        face(tris, b, k, f, -across);
    }
}

/// A round tube along a line of points, closed at both ends.
pub(super) fn tube(tris: &mut Vec<Triangle>, points: &[Vec3], radius: f32, sides: usize) {
    let count = points.len();
    let ring = |index: usize| -> Vec<Vec3> {
        let along = (points[(index + 1).min(count - 1)] - points[index.saturating_sub(1)])
            .normalize_or(Vec3::Y);
        let first = along.any_orthonormal_vector();
        let second = along.cross(first);
        (0..sides)
            .map(|side| {
                let angle = side as f32 / sides as f32 * TAU;
                points[index] + (first * angle.cos() + second * angle.sin()) * radius
            })
            .collect()
    };
    let rings: Vec<Vec<Vec3>> = (0..count).map(ring).collect();
    for index in 0..count - 1 {
        for side in 0..sides {
            let next = (side + 1) % sides;
            let out = rings[index][side] - points[index];
            let (a, b) = (rings[index][side], rings[index][next]);
            let (c, d) = (rings[index + 1][side], rings[index + 1][next]);
            face(tris, a, b, d, out);
            face(tris, a, d, c, out);
        }
    }
    for (index, inward) in [(0, 1), (count - 1, count - 2)] {
        let away = points[index] - points[inward];
        for side in 0..sides {
            let (a, b) = (rings[index][side], rings[index][(side + 1) % sides]);
            face(tris, points[index], a, b, away);
        }
    }
}

/// A blob round a middle, with three half axes.
pub(super) fn blob(tris: &mut Vec<Triangle>, middle: Vec3, axes: [Vec3; 3]) {
    const ROWS: usize = 6;
    const ROUND: usize = 10;
    let point = |row: usize, column: usize| {
        let down = row as f32 / ROWS as f32 * std::f32::consts::PI;
        let round = column as f32 / ROUND as f32 * TAU;
        middle + axes[0] * down.cos() + (axes[1] * round.cos() + axes[2] * round.sin()) * down.sin()
    };
    for row in 0..ROWS {
        for column in 0..ROUND {
            let quad = [
                point(row, column),
                point(row, column + 1),
                point(row + 1, column + 1),
                point(row + 1, column),
            ];
            let out = (quad[0] + quad[2]) / 2.0 - middle;
            face(tris, quad[0], quad[1], quad[2], out);
            face(tris, quad[0], quad[2], quad[3], out);
        }
    }
}
