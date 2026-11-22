//! A decor model being put together from simple solids, each moved, turned
//! and coloured, then made into its meshes: its colours, a black outline
//! (grown out from each solid's middle, so its edges leave no cracks), a sepia
//! copy for memory, and flames in their own colours whatever the light.

use bevy::prelude::*;

use crate::plants::Shell;

/// How much of the maps' outline width small decor's outlines take.
const THIN: f32 = 0.6;

/// A model being built, in its own frame.
#[derive(Default)]
pub(super) struct Model {
    fill: Shell,
    /// The same solids with normals pointing out from each one's middle.
    hull: Shell,
    flames: Shell,
}

/// A model's meshes.
#[derive(Clone, Debug)]
pub(in crate::dungeon_map) struct DecorMeshes {
    pub(super) fill: Handle<Mesh>,
    pub(super) outline: Handle<Mesh>,
    pub(super) memory: Handle<Mesh>,
    /// Its flames, if it has any.
    pub(super) flames: Option<Handle<Mesh>>,
}

impl Model {
    /// A box `size` across, its middle at `at`, turned by `turn`.
    pub(super) fn cuboid(&mut self, at: Vec3, size: Vec3, turn: Quat, colour: Vec3) {
        self.solid(&Mesh::from(Cuboid::from_size(size)), at, turn, colour);
    }

    /// A box from `low` to `high`, unturned.
    pub(super) fn block(&mut self, low: Vec3, high: Vec3, colour: Vec3) {
        self.cuboid(
            (low + high) / 2.0,
            (high - low).abs(),
            Quat::IDENTITY,
            colour,
        );
    }

    /// An upright cylinder `radius` round and `height` tall, its middle at
    /// `at`, turned by `turn`.
    pub(super) fn cylinder(
        &mut self,
        at: Vec3,
        (radius, height): (f32, f32),
        turn: Quat,
        colour: Vec3,
    ) {
        self.solid(&Mesh::from(Cylinder::new(radius, height)), at, turn, colour);
    }

    /// A square rod `thick` across from `from` to `to`.
    pub(super) fn stick(&mut self, (from, to): (Vec3, Vec3), thick: f32, colour: Vec3) {
        let along = to - from;
        let turn = Quat::from_rotation_arc(Vec3::Y, along.normalize_or(Vec3::Y));
        let size = Vec3::new(thick, along.length(), thick);
        self.cuboid((from + to) / 2.0, size, turn, colour);
    }

    /// A cone `radius` round at its foot and `height` tall, point up, its
    /// middle at `at`.
    pub(super) fn cone(&mut self, at: Vec3, (radius, height): (f32, f32), colour: Vec3) {
        let mesh = Mesh::from(Cone::new(radius, height));
        self.solid(&mesh, at, Quat::IDENTITY, colour);
    }

    /// A ring lying flat, `radius` to the middle of its rod, `thick` across,
    /// its middle at `at`.
    pub(super) fn hoop(&mut self, at: Vec3, (radius, thick): (f32, f32), colour: Vec3) {
        let half = thick / 2.0;
        let mesh = Mesh::from(Torus::new(radius - half, radius + half));
        self.solid(&mesh, at, Quat::IDENTITY, colour);
    }

    /// A ball `size` across each way (squashed to an egg), its middle at `at`.
    pub(super) fn ball(&mut self, at: Vec3, size: Vec3, colour: Vec3) {
        let mesh = Sphere::new(0.5).mesh().uv(10, 6);
        let place = Transform::from_translation(at).with_scale(size);
        self.fill.add(&mesh, place, colour);
        self.add_hull(&mesh, place);
    }

    /// A flat oval `size` across each way (its height `size.y`), its middle at
    /// `at`.
    pub(super) fn oval(&mut self, at: Vec3, size: Vec3, colour: Vec3) {
        let mesh = Mesh::from(Cylinder::new(0.5, 1.0));
        let place = Transform::from_translation(at).with_scale(size);
        self.fill.add(&mesh, place, colour);
        self.add_hull(&mesh, place);
    }

    /// A candle's flame over `at` (the top of its wick), `size` tall.
    pub(super) fn flame(&mut self, at: Vec3, size: f32) {
        let mesh = Sphere::new(0.5).mesh().uv(8, 5);
        let shape = Vec3::new(size * 0.45, size, size * 0.45);
        let place = Transform::from_translation(at + Vec3::Y * size / 2.0).with_scale(shape);
        self.flames.add(&mesh, place, Vec3::new(1.0, 0.75, 0.25));
    }

    fn solid(&mut self, mesh: &Mesh, at: Vec3, turn: Quat, colour: Vec3) {
        let place = Transform::from_translation(at).with_rotation(turn);
        self.fill.add(mesh, place, colour);
        self.add_hull(mesh, place);
    }

    /// The solid again for the outline, every point's normal pointing out
    /// from its middle, squashed to its shape.
    fn add_hull(&mut self, mesh: &Mesh, place: Transform) {
        let from = self.hull.positions.len();
        self.hull.add(mesh, Transform::IDENTITY, Vec3::ZERO);
        let points = &self.hull.positions[from..];
        let most = points.iter().fold(Vec3::splat(1e-6), |m, p| m.max(p.abs()));
        let matrix = place.to_matrix();
        let turn = Mat3::from_mat4(matrix);
        for i in from..self.hull.positions.len() {
            let local = self.hull.positions[i];
            self.hull.normals[i] = (turn * (local / most)).normalize_or(Vec3::Y);
            self.hull.positions[i] = matrix.transform_point3(local);
        }
    }

    /// Into meshes.
    pub(super) fn build(self, meshes: &mut Assets<Mesh>) -> DecorMeshes {
        DecorMeshes {
            fill: meshes.add(self.fill.fill()),
            outline: meshes.add(self.hull.outline(THIN)),
            memory: meshes.add(self.fill.memory()),
            flames: (!self.flames.is_empty()).then(|| meshes.add(self.flames.fill())),
        }
    }
}

/// A colour (linear) from sRGB.
pub(super) fn srgb(r: f32, g: f32, b: f32) -> Vec3 {
    let linear = Color::srgb(r, g, b).to_linear();
    Vec3::new(linear.red, linear.green, linear.blue)
}
