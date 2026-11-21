//! What every cooked food's model is built with: its colours as materials,
//! the crockery's, and a short way to add a piece.

use bevy::prelude::*;
use shared::herb::Rgb;

use bevy::mesh::VertexAttributeValues;

use super::DishLook;
use crate::item_art::{CAMERA_AT, CAMERA_UP, Part};

/// The materials and pieces of one model being built.
pub(super) struct Kit<'a> {
    pub(super) meshes: &'a mut Assets<Mesh>,
    /// The food's own colour, a little shiny.
    pub(super) food: Handle<StandardMaterial>,
    /// The second colour, for pieces on or in it.
    pub(super) accent: Handle<StandardMaterial>,
    /// The tint browned further, for crusts and grill marks.
    pub(super) crust: Handle<StandardMaterial>,
    /// The tint paler, for flour dust, cream and cut faces.
    pub(super) pale: Handle<StandardMaterial>,
    /// Glazed white crockery: plates, bowls, sauce boats.
    pub(super) crockery: Handle<StandardMaterial>,
    /// Brown earthenware: pie dishes, jars, crocks.
    pub(super) earthenware: Handle<StandardMaterial>,
    /// Pale wood: boards and skewers.
    pub(super) wood: Handle<StandardMaterial>,
    /// Undyed linen and string.
    pub(super) linen: Handle<StandardMaterial>,
    pub(super) parts: Vec<Part>,
}

fn material(red: f32, green: f32, blue: f32, roughness: f32) -> StandardMaterial {
    StandardMaterial {
        base_color: Color::srgb(red, green, blue),
        perceptual_roughness: roughness,
        reflectance: 0.25,
        ..default()
    }
}

fn of(Rgb(red, green, blue): Rgb, times: f32, roughness: f32) -> StandardMaterial {
    let part = |byte: u8| (f32::from(byte) / 255.0 * times).min(1.0);
    material(part(red), part(green), part(blue), roughness)
}

/// A colour `share` of the way to white.
fn paler(Rgb(red, green, blue): Rgb, share: f32) -> Rgb {
    let lift = |byte: u8| (f32::from(byte) + (255.0 - f32::from(byte)) * share) as u8;
    Rgb(lift(red), lift(green), lift(blue))
}

impl<'a> Kit<'a> {
    pub(super) fn new(
        look: &DishLook,
        meshes: &'a mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
    ) -> Self {
        Self {
            meshes,
            food: materials.add(of(look.tint, 1.0, 0.55)),
            accent: materials.add(of(look.accent, 1.0, 0.6)),
            crust: materials.add(of(look.tint, 0.72, 0.75)),
            pale: materials.add(of(paler(look.tint, 0.55), 1.0, 0.8)),
            crockery: materials.add(material(0.93, 0.91, 0.86, 0.3)),
            earthenware: materials.add(material(0.6, 0.36, 0.22, 0.75)),
            wood: materials.add(material(0.5, 0.34, 0.2, 0.8)),
            linen: materials.add(material(0.86, 0.82, 0.72, 0.95)),
            parts: Vec::new(),
        }
    }

    /// Adds a piece.
    pub(super) fn add(
        &mut self,
        mesh: impl Into<Mesh>,
        material: &Handle<StandardMaterial>,
        transform: Transform,
    ) {
        let mesh = self.meshes.add(mesh.into());
        self.parts.push(Part {
            mesh,
            material: material.clone(),
            transform,
        });
    }

    /// A round dish standing on the table: a low wall from `bottom` to `top`
    /// across, `height` tall, with a rolled rim.
    pub(super) fn round_dish(
        &mut self,
        material: &Handle<StandardMaterial>,
        bottom: f32,
        top: f32,
        height: f32,
    ) {
        let frustum = ConicalFrustum {
            radius_top: top,
            radius_bottom: bottom,
            height,
        };
        self.add(
            frustum,
            material,
            Transform::from_xyz(0.0, height / 2.0, 0.0),
        );
        let rim = height.clamp(0.03, 0.08);
        self.add(
            Torus::new(top - rim, top + rim * 0.4),
            material,
            Transform::from_xyz(0.0, height, 0.0),
        );
    }

    /// The pieces grown by `size`, tipped `tip` towards the camera about x so
    /// a little more of their sides shows, then moved to the middle of the picture as the
    /// camera sees it (a tall dish's rim would sit high otherwise).
    pub(super) fn finish(self, tip: f32, size: f32) -> Vec<Part> {
        let turn =
            Transform::from_rotation(Quat::from_rotation_x(tip)).with_scale(Vec3::splat(size));
        let view = Transform::from_translation(CAMERA_AT).looking_at(Vec3::ZERO, CAMERA_UP);
        let axes = [*view.right(), *view.up()];
        let mut low = [f32::MAX; 2];
        let mut high = [f32::MIN; 2];
        for part in &self.parts {
            let Some(VertexAttributeValues::Float32x3(points)) = (self.meshes.get(&part.mesh))
                .and_then(|mesh| mesh.attribute(Mesh::ATTRIBUTE_POSITION))
            else {
                continue;
            };
            for &point in points {
                let at = (turn * part.transform).transform_point(Vec3::from(point));
                for (axis, (low, high)) in axes.iter().zip(low.iter_mut().zip(&mut high)) {
                    *low = low.min(at.dot(*axis));
                    *high = high.max(at.dot(*axis));
                }
            }
        }
        let middle = |i: usize| (low[i] + high[i]) / 2.0;
        let shift = if low[0] <= high[0] {
            -(axes[0] * middle(0) + axes[1] * middle(1))
        } else {
            Vec3::ZERO
        };
        let placed = Transform::from_translation(shift) * turn;
        (self.parts.into_iter())
            .map(|part| Part {
                transform: placed * part.transform,
                ..part
            })
            .collect()
    }
}

/// A point `along` the way round a circle of `radius`, at height `y`.
pub(super) fn round(along: f32, radius: f32, y: f32) -> Vec3 {
    let angle = along * std::f32::consts::TAU;
    Vec3::new(angle.cos() * radius, y, angle.sin() * radius)
}
