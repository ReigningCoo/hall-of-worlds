//! The glass: drawn twice, as gems are. First the inside of the far side, solid
//! and pale, which is what is seen through an empty bottle (and what a liquid
//! is drawn in front of); then the near side, nearly clear in the middle and
//! tinted towards its edges and up the neck, with a streak of light down the
//! left. So the tint never changes how a liquid in the middle reads.

use bevy::asset::RenderAssetUsages;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;
use bevy::render::render_resource::Face;

use super::look::Glass;
use super::surface::Surface;

/// How see-through the near side is where it faces the camera, and at its edges.
const MIDDLE_ALPHA: f32 = 0.1;
const EDGE_ALPHA: f32 = 0.8;
/// More tint up the neck, where glass is seen through more of itself.
const NECK_ALPHA: f32 = 0.35;
/// How much of the glass's shade the far side takes, against white.
const FAR_TINT: f32 = 0.35;
/// Which way the streak of light faces, round the bottle from the front.
const STREAK: Vec2 = Vec2::new(-0.55, 0.83);

/// The glass's shade.
pub(super) fn shade(glass: Glass) -> Color {
    match glass {
        Glass::Clear => Color::srgb(0.78, 0.9, 0.94),
        Glass::PaleGreen => Color::srgb(0.55, 0.82, 0.55),
        Glass::Amber => Color::srgb(0.8, 0.55, 0.06),
        Glass::Cobalt => Color::srgb(0.14, 0.28, 0.85),
        Glass::Smoky => Color::srgb(0.38, 0.38, 0.4),
        Glass::Violet => Color::srgb(0.52, 0.3, 0.78),
        Glass::Rose => Color::srgb(0.9, 0.5, 0.62),
    }
}

/// The far side's and near side's materials.
pub(super) fn materials(
    glass: Glass,
    materials: &mut Assets<StandardMaterial>,
) -> (Handle<StandardMaterial>, Handle<StandardMaterial>) {
    let far = materials.add(StandardMaterial {
        base_color: Color::WHITE.mix(&shade(glass), FAR_TINT),
        perceptual_roughness: 0.35,
        reflectance: 0.3,
        // The inside of the far side, lit as seen from here.
        cull_mode: Some(Face::Front),
        double_sided: true,
        ..default()
    });
    let near = materials.add(StandardMaterial {
        // Its colour is in each point's, with how see-through it is there.
        base_color: Color::WHITE,
        perceptual_roughness: 0.1,
        reflectance: 0.8,
        alpha_mode: AlphaMode::Blend,
        ..default()
    });
    (far, near)
}

/// The surface's meshes for the far side, and with colours for the near side:
/// `view` is the way to the camera in the bottle's own space, `neck` the height
/// the neck starts at.
pub(super) fn glass_meshes(surface: &Surface, glass: Glass, view: Vec3, neck: f32) -> (Mesh, Mesh) {
    let (positions, normals, indices) = surface.triangles();
    let tint = shade(glass).to_linear();
    let around = positions.len() / surface.rows.len();
    let colours: Vec<[f32; 4]> = positions
        .iter()
        .zip(&normals)
        .enumerate()
        .map(|(k, (point, normal))| {
            let edge = 1.0 - normal.dot(view).abs();
            let mut alpha = MIDDLE_ALPHA + (EDGE_ALPHA - MIDDLE_ALPHA) * edge * edge;
            if point.y > neck {
                alpha = alpha.max(NECK_ALPHA);
            }
            let flat = Vec2::new(normal.x, normal.z).normalize_or_zero();
            let curved = 1.0 - surface.faceted[k / around];
            let light = ((flat.dot(STREAK) - 0.95) / 0.05).clamp(0.0, 1.0) * curved;
            let colour = tint.mix(&LinearRgba::WHITE, light);
            [
                colour.red,
                colour.green,
                colour.blue,
                alpha.max(light * 0.85),
            ]
        })
        .collect();
    let far = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_indices(Indices::U32(indices));
    let near = far
        .clone()
        .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colours);
    (far, near)
}
