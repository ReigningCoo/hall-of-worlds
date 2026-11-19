//! `Pieces`: what plants are built of (stems, leaves and blobs), each a small
//! convex solid, drawn in its colour and again grown and black for its outline
//! (an inverted hull: only its inside faces are drawn).

use std::f32::consts::{PI, TAU};
use std::sync::LazyLock;

use bevy::prelude::*;

use super::triangles::Triangles;

/// How much wider than the plant its black outline is, in world units (as
/// bags' and the maps' other outlines).
pub const OUTLINE_WIDTH: f32 = 0.35;

/// A plant's model being built: its colours, and its outline.
#[derive(Clone, Debug, Default)]
pub struct Pieces {
    pub fill: Triangles,
    pub outline: Triangles,
}

/// A solid's corners and its faces, as corner numbers.
type Solid = (Vec<Vec3>, &'static [[usize; 3]]);

/// A box between two square ends: 4 sides and both ends.
const TUBE_FACES: [[usize; 3]; 12] = [
    [0, 1, 5],
    [0, 5, 4],
    [1, 2, 6],
    [1, 6, 5],
    [2, 3, 7],
    [2, 7, 6],
    [3, 0, 4],
    [3, 4, 7],
    [0, 1, 2],
    [0, 2, 3],
    [4, 5, 6],
    [4, 6, 7],
];

/// Two pyramids back to back: base and tip (or bottom and top), with four
/// corners round the middle.
const DOUBLE_PYRAMID_FACES: [[usize; 3]; 8] = [
    [0, 2, 4],
    [0, 4, 3],
    [0, 3, 5],
    [0, 5, 2],
    [1, 2, 4],
    [1, 4, 3],
    [1, 3, 5],
    [1, 5, 2],
];

/// Corners round a ball, and rings of them between its poles.
const BALL_AROUND: usize = 7;
const BALL_RINGS: usize = 3;

/// A ball's faces: its corners are the bottom pole, the rings from the bottom
/// up, then the top pole.
static BALL_FACES: LazyLock<Vec<[usize; 3]>> = LazyLock::new(|| {
    let ring = |r: usize, i: usize| 1 + r * BALL_AROUND + i % BALL_AROUND;
    let top = 1 + BALL_RINGS * BALL_AROUND;
    let mut faces = Vec::new();
    for i in 0..BALL_AROUND {
        faces.push([0, ring(0, i), ring(0, i + 1)]);
        for r in 0..BALL_RINGS - 1 {
            faces.push([ring(r, i), ring(r + 1, i), ring(r + 1, i + 1)]);
            faces.push([ring(r, i), ring(r + 1, i + 1), ring(r, i + 1)]);
        }
        faces.push([top, ring(BALL_RINGS - 1, i), ring(BALL_RINGS - 1, i + 1)]);
    }
    faces
});

/// A cone's faces: its corners are the point, then round its base.
static CONE_FACES: LazyLock<Vec<[usize; 3]>> = LazyLock::new(|| {
    let ring = |i: usize| 1 + i % BALL_AROUND;
    let mut faces = Vec::new();
    for i in 0..BALL_AROUND {
        faces.push([0, ring(i), ring(i + 1)]);
        if i >= 2 {
            faces.push([1, ring(i - 1), ring(i)]);
        }
    }
    faces
});

impl Pieces {
    /// Adds a solid made by `solid`, given how much to grow it: in `colour`,
    /// and grown and black as its outline.
    fn add(&mut self, solid: impl Fn(f32) -> Solid, colour: Vec3) {
        for (grow, into) in [(0.0, &mut self.fill), (OUTLINE_WIDTH, &mut self.outline)] {
            let (corners, faces) = solid(grow);
            let inside = corners.iter().sum::<Vec3>() / corners.len() as f32;
            for face in faces {
                let triangle = face.map(|corner| corners[corner]);
                into.add(triangle, inside, colour, grow == 0.0);
            }
        }
    }

    /// A stem from `from` to `to`, `radius` thick at `from` narrowing to
    /// `tip` at `to`.
    pub fn stem(&mut self, from: Vec3, to: Vec3, (radius, tip): (f32, f32), colour: Vec3) {
        let along = (to - from).normalize_or(Vec3::Y);
        let side = along.any_orthonormal_vector();
        let other = along.cross(side);
        self.add(
            move |grow| {
                let ring = |at: Vec3, r: f32| {
                    let r = r + grow;
                    [side * r, other * r, -side * r, -other * r].map(|offset| at + offset)
                };
                let mut corners = ring(from - along * grow, radius).to_vec();
                corners.extend(ring(to + along * grow, tip));
                (corners, &TUBE_FACES)
            },
            colour,
        );
    }

    /// A leaf from `base` along `towards` (any length: `length` says how far),
    /// `width` across, lying as flat as it can (its face up), with a ridge
    /// `thickness` deep. Widest a third of the way out.
    pub fn leaf(
        &mut self,
        base: Vec3,
        towards: Vec3,
        (length, width, thickness): (f32, f32, f32),
        colour: Vec3,
    ) {
        let along = towards.normalize_or(Vec3::X);
        let across = along.cross(Vec3::Y).try_normalize().unwrap_or(Vec3::X);
        let face = across.cross(along);
        self.add(
            move |grow| {
                let middle = base + along * length * 0.35;
                let half = width / 2.0 + grow;
                let deep = thickness / 2.0 + grow;
                let corners = vec![
                    base - along * grow,
                    base + along * (length + grow),
                    middle + across * half,
                    middle - across * half,
                    middle + face * deep,
                    middle - face * deep,
                ];
                (corners, &DOUBLE_PYRAMID_FACES)
            },
            colour,
        );
    }

    /// A rounded lump (a flower head, a berry, a mass of leaves): `size` its
    /// half widths east, up and south.
    pub fn blob(&mut self, centre: Vec3, size: Vec3, colour: Vec3) {
        self.add(
            move |grow| {
                let s = size + Vec3::splat(grow);
                let corners = vec![
                    centre - Vec3::Y * s.y,
                    centre + Vec3::Y * s.y,
                    centre + Vec3::X * s.x,
                    centre - Vec3::X * s.x,
                    centre + Vec3::Z * s.z,
                    centre - Vec3::Z * s.z,
                ];
                (corners, &DOUBLE_PYRAMID_FACES)
            },
            colour,
        );
    }

    /// A blob turned a little about the upright, so heads side by side don't
    /// all line up.
    pub fn blob_turned(&mut self, centre: Vec3, size: Vec3, turn: f32, colour: Vec3) {
        let (sin, cos) = turn.sin_cos();
        let east = Vec3::new(cos, 0.0, sin);
        let south = Vec3::new(-sin, 0.0, cos);
        self.add(
            move |grow| {
                let s = size + Vec3::splat(grow);
                let corners = vec![
                    centre - Vec3::Y * s.y,
                    centre + Vec3::Y * s.y,
                    centre + east * s.x,
                    centre - east * s.x,
                    centre + south * s.z,
                    centre - south * s.z,
                ];
                (corners, &DOUBLE_PYRAMID_FACES)
            },
            colour,
        );
    }

    /// A cone (a conifer's tier of needles): its base round `base`, `radius`
    /// wide, its point `height` above, turned `turn` about the upright.
    pub fn cone(&mut self, base: Vec3, (radius, height): (f32, f32), turn: f32, colour: Vec3) {
        self.add(
            move |grow| {
                let mut corners = vec![base + Vec3::Y * (height + grow * 1.5)];
                let low = base - Vec3::Y * grow;
                for i in 0..BALL_AROUND {
                    let (z, x) = (turn + i as f32 * TAU / BALL_AROUND as f32).sin_cos();
                    corners.push(low + Vec3::new(x, 0.0, z) * (radius + grow));
                }
                (corners, CONE_FACES.as_slice())
            },
            colour,
        );
    }

    /// A ball (a fruit, a crown of leaves): `size` its half widths east, up
    /// and south, turned `turn` about the upright. Rounder than a blob.
    pub fn ball(&mut self, centre: Vec3, size: Vec3, turn: f32, colour: Vec3) {
        let (sin, cos) = turn.sin_cos();
        let east = Vec3::new(cos, 0.0, sin);
        let south = Vec3::new(-sin, 0.0, cos);
        self.add(
            move |grow| {
                let s = size + Vec3::splat(grow);
                let mut corners = vec![centre - Vec3::Y * s.y];
                for r in 0..BALL_RINGS {
                    let up = -PI / 2.0 + (r + 1) as f32 * PI / (BALL_RINGS + 1) as f32;
                    for i in 0..BALL_AROUND {
                        let round = (i as f32 + 0.5 * r as f32) * TAU / BALL_AROUND as f32;
                        let (y, flat) = up.sin_cos();
                        let (z, x) = round.sin_cos();
                        corners.push(
                            centre
                                + east * x * flat * s.x
                                + Vec3::Y * y * s.y
                                + south * z * flat * s.z,
                        );
                    }
                }
                corners.push(centre + Vec3::Y * s.y);
                (corners, BALL_FACES.as_slice())
            },
            colour,
        );
    }
}
