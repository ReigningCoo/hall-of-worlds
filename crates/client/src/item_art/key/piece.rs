//! `Piece`: one simple solid of a key, in the key's own space, which lies flat in
//! the x-z plane with its shank along +x and its bow's middle at the origin.

use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;

/// What a piece is.
#[derive(Clone, Copy, Debug)]
pub(super) enum Shape {
    /// A box this big.
    Block(Vec3),
    /// A cylinder about its own y axis.
    Rod {
        radius: f32,
        length: f32,
    },
    /// A torus about its own y axis, between these radii.
    Loop {
        inner: f32,
        outer: f32,
    },
    Bead(f32),
}

/// Whether a piece is the key's metal, or the dark inside of a hollow pipe.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Finish {
    Metal,
    Hole,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct Piece {
    pub(super) shape: Shape,
    pub(super) at: Transform,
    pub(super) finish: Finish,
}

impl Piece {
    pub(super) fn new(shape: Shape, at: Transform) -> Self {
        Self {
            shape,
            at,
            finish: Finish::Metal,
        }
    }

    /// A box from `from` to `to` in x and z, centred on y = 0.
    pub(super) fn block(from: Vec2, to: Vec2, thick: f32) -> Self {
        let size = (to - from).abs();
        let middle = (from + to) / 2.0;
        Self::new(
            Shape::Block(Vec3::new(size.x, thick, size.y)),
            Transform::from_xyz(middle.x, 0.0, middle.y),
        )
    }

    /// A flat bar `wide` across from `from` to `to` (x, z).
    pub(super) fn bar(from: Vec2, to: Vec2, wide: f32, thick: f32) -> Self {
        let way = to - from;
        let middle = (from + to) / 2.0;
        Self::new(
            Shape::Block(Vec3::new(way.length() + wide, thick, wide)),
            Transform::from_xyz(middle.x, 0.0, middle.y)
                .with_rotation(Quat::from_rotation_y((-way.y).atan2(way.x))),
        )
    }

    /// A flat ring lying in the key's plane, round `middle` (x, z).
    pub(super) fn flat_loop(middle: Vec2, outer: f32, wide: f32) -> Self {
        Self::new(
            Shape::Loop {
                inner: outer - wide,
                outer,
            },
            Transform::from_xyz(middle.x, 0.0, middle.y),
        )
    }

    /// A rod along x from `from` to `to`.
    pub(super) fn along(from: f32, to: f32, radius: f32) -> Self {
        Self::new(
            Shape::Rod {
                radius,
                length: to - from,
            },
            Transform::from_xyz((from + to) / 2.0, 0.0, 0.0)
                .with_rotation(Quat::from_rotation_z(-FRAC_PI_2)),
        )
    }

    /// A ring round the shank at `x`.
    pub(super) fn round_shank(x: f32, outer: f32, wide: f32) -> Self {
        Self::new(
            Shape::Loop {
                inner: outer - wide,
                outer,
            },
            Transform::from_xyz(x, 0.0, 0.0).with_rotation(Quat::from_rotation_z(-FRAC_PI_2)),
        )
    }

    /// How far its own shape reaches along each of its own axes.
    fn half(&self) -> Vec3 {
        match self.shape {
            Shape::Block(size) => size / 2.0,
            Shape::Rod { radius, length } => Vec3::new(radius, length / 2.0, radius),
            Shape::Loop { inner, outer } => Vec3::new(outer, (outer - inner) / 2.0, outer),
            Shape::Bead(radius) => Vec3::splat(radius),
        }
    }

    /// The corners of a box round it, placed by `whole` after its own transform.
    pub(super) fn corners(&self, whole: Transform) -> [Vec3; 8] {
        let half = self.half();
        let placed = whole * self.at;
        std::array::from_fn(|corner| {
            let sign = |bit: usize| if corner & bit == 0 { -1.0 } else { 1.0 };
            placed.transform_point(half * Vec3::new(sign(1), sign(2), sign(4)))
        })
    }

    pub(super) fn mesh(&self) -> Mesh {
        match self.shape {
            Shape::Block(size) => Cuboid::from_size(size).into(),
            Shape::Rod { radius, length } => Cylinder::new(radius, length).into(),
            Shape::Loop { inner, outer } => Torus::new(inner, outer).into(),
            Shape::Bead(radius) => Sphere::new(radius).into(),
        }
    }
}
