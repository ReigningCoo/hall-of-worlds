//! `Kit`: what a herb's picture is built with: materials by colour, and blobs
//! (squashed spheres) and rods (cylinders between two points), the whole turned
//! a little by its variant.

use bevy::mesh::VertexAttributeValues;
use bevy::prelude::*;

use super::super::Part;

/// A small, fixed wobble for the `n`th piece's `k`th measure, -1 to 1: the
/// same every time, so a variant always looks the same.
pub(in crate::item_art) fn wobble(n: usize, variant: u8, k: u32) -> f32 {
    let mut x = (n as u32).wrapping_mul(0x9E37_79B9) ^ u32::from(variant).wrapping_mul(0x85EB_CA6B);
    x ^= k.wrapping_mul(0xC2B2_AE35);
    x ^= x >> 15;
    x = x.wrapping_mul(0x2C1B_3C6D);
    x ^= x >> 12;
    (x % 2001) as f32 / 1000.0 - 1.0
}

/// The way out at angle `a` from straight up the picture (towards its top, -z),
/// and the turn about the up axis that lays a blob's length (x) along it.
pub(in crate::item_art) fn along(a: f32) -> (Vec3, f32) {
    (
        Vec3::new(a.sin(), 0.0, -a.cos()),
        std::f32::consts::FRAC_PI_2 - a,
    )
}

/// Building a herb's parts.
pub(in crate::item_art) struct Kit<'a> {
    meshes: &'a mut Assets<Mesh>,
    materials: &'a mut Assets<StandardMaterial>,
    parts: Vec<Part>,
    /// The whole turned about the up axis, by variant.
    turn: Quat,
}

impl<'a> Kit<'a> {
    pub(in crate::item_art) fn new(
        meshes: &'a mut Assets<Mesh>,
        materials: &'a mut Assets<StandardMaterial>,
        variant: u8,
    ) -> Self {
        let turn = Quat::from_rotation_y((f32::from(variant) - 1.0) * 0.35);
        Self {
            meshes,
            materials,
            parts: Vec::new(),
            turn,
        }
    }

    /// A matte material of this colour, a little shiny if `shine`.
    pub(in crate::item_art) fn material(
        &mut self,
        colour: Color,
        shine: bool,
    ) -> Handle<StandardMaterial> {
        self.materials.add(StandardMaterial {
            base_color: colour,
            perceptual_roughness: if shine { 0.35 } else { 0.85 },
            reflectance: if shine { 0.4 } else { 0.12 },
            ..default()
        })
    }

    /// A sphere squashed to `scale` (half sizes), turned `yaw` about the up axis
    /// and then tipped `pitch` about its own length (x).
    pub(in crate::item_art) fn blob(
        &mut self,
        material: &Handle<StandardMaterial>,
        at: Vec3,
        scale: Vec3,
        (yaw, pitch): (f32, f32),
    ) {
        let mesh = self.meshes.add(Sphere::new(1.0).mesh().uv(16, 10));
        let rotation = Quat::from_rotation_y(yaw) * Quat::from_rotation_x(pitch);
        self.push(
            mesh,
            material,
            Transform::from_translation(at)
                .with_rotation(rotation)
                .with_scale(scale),
        );
    }

    /// A rod of this radius from `from` to `to`.
    pub(in crate::item_art) fn rod(
        &mut self,
        material: &Handle<StandardMaterial>,
        from: Vec3,
        to: Vec3,
        radius: f32,
    ) {
        let length = from.distance(to);
        if length <= f32::EPSILON {
            return;
        }
        let mesh = self.meshes.add(Cylinder::new(radius, length));
        let rotation = Quat::from_rotation_arc(Vec3::Y, (to - from) / length);
        let transform = Transform::from_translation((from + to) / 2.0).with_rotation(rotation);
        self.push(mesh, material, transform);
    }

    /// A cone of this radius from its base at `from` to its tip at `to`.
    pub(in crate::item_art) fn cone(
        &mut self,
        material: &Handle<StandardMaterial>,
        from: Vec3,
        to: Vec3,
        radius: f32,
    ) {
        let length = from.distance(to);
        if length <= f32::EPSILON {
            return;
        }
        let mesh = self.meshes.add(Cone::new(radius, length));
        let rotation = Quat::from_rotation_arc(Vec3::Y, (to - from) / length);
        let transform = Transform::from_translation((from + to) / 2.0).with_rotation(rotation);
        self.push(mesh, material, transform);
    }

    /// The whole turned `angle` about the up axis instead of by variant.
    pub(in crate::item_art) fn turned(mut self, angle: f32) -> Self {
        self.turn = Quat::from_rotation_y(angle);
        self
    }

    /// Any other mesh, placed by `transform`.
    pub(in crate::item_art) fn mesh(
        &mut self,
        mesh: Mesh,
        material: &Handle<StandardMaterial>,
        transform: Transform,
    ) {
        let mesh = self.meshes.add(mesh);
        self.push(mesh, material, transform);
    }

    fn push(
        &mut self,
        mesh: Handle<Mesh>,
        material: &Handle<StandardMaterial>,
        transform: Transform,
    ) {
        let transform = Transform::from_rotation(self.turn) * transform;
        self.parts.push(Part {
            mesh,
            material: material.clone(),
            transform,
        });
    }

    /// A matte material seen from both sides, for thin sheets (leaves, husks).
    pub(in crate::item_art) fn sheet(&mut self, colour: Color) -> Handle<StandardMaterial> {
        self.materials.add(StandardMaterial {
            base_color: colour,
            perceptual_roughness: 0.8,
            reflectance: 0.12,
            double_sided: true,
            cull_mode: None,
            ..default()
        })
    }

    /// Shrinks everything built about the middle, if need be, so it reaches no
    /// further than `half` along x and z.
    pub(in crate::item_art) fn fit(&mut self, half: Vec2) {
        let shrink = (half / self.reach()).min_element();
        if shrink < 1.0 {
            self.scale(shrink);
        }
    }

    /// Scales everything built about the middle, up or down, so it just reaches
    /// `half` along x or z.
    pub(in crate::item_art) fn fill(&mut self, half: Vec2) {
        let scale = (half / self.reach()).min_element();
        self.scale(scale);
    }

    /// Moves everything built so the middle of what it covers along x and z is
    /// at the middle of the picture.
    pub(in crate::item_art) fn centre(&mut self) {
        let (low, high) = self.bounds();
        let middle = (low + high) / 2.0;
        let shift = Vec3::new(-middle.x, 0.0, -middle.y);
        for part in &mut self.parts {
            part.transform.translation += shift;
        }
    }

    fn scale(&mut self, scale: f32) {
        for part in &mut self.parts {
            part.transform = Transform::from_scale(Vec3::splat(scale)) * part.transform;
        }
    }

    /// How far from the middle anything built reaches along x and z.
    fn reach(&self) -> Vec2 {
        let (low, high) = self.bounds();
        low.abs().max(high.abs()).max(Vec2::splat(f32::EPSILON))
    }

    /// The least and most x and z anything built reaches.
    fn bounds(&self) -> (Vec2, Vec2) {
        let (mut low, mut high) = (Vec2::splat(f32::MAX), Vec2::splat(f32::MIN));
        for part in &self.parts {
            let Some(mesh) = self.meshes.get(&part.mesh) else {
                continue;
            };
            let Some(VertexAttributeValues::Float32x3(points)) =
                mesh.attribute(Mesh::ATTRIBUTE_POSITION)
            else {
                continue;
            };
            for &point in points {
                let at = part.transform.transform_point(Vec3::from(point));
                low = low.min(Vec2::new(at.x, at.z));
                high = high.max(Vec2::new(at.x, at.z));
            }
        }
        if low.x > high.x {
            return (Vec2::ZERO, Vec2::ZERO);
        }
        (low, high)
    }

    /// The parts built.
    pub(in crate::item_art) fn finish(self) -> Vec<Part> {
        self.parts
    }
}
