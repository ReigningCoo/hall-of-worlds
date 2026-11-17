//! A potion's liquid as parts of its bottle's model: each band of the body (two
//! when still and layered) drawn twice, its far inside solid and its near side
//! see-through, then its features in front. The see-through parts come after
//! the glass's far side and before its near side, kept in that order by their
//! depth bias (the near glass has none).

use bevy::asset::RenderAssetUsages;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;
use bevy::render::render_resource::Face;

use super::super::Part;
use super::front::Front;
use super::liquid::LiquidLook;
use super::liquid_body::{band, levels};
use super::liquid_colour::{alpha, blob_colour, main_colour, second_colour};
use super::liquid_features::{Feature, features};
use super::look::Shape;
use super::spheres::balls_mesh;
use super::surface::Surface;

/// Depth biases: the lower layer first, then the body, then the features.
const LOWER_BIAS: f32 = -30.0;
const BODY_BIAS: f32 = -20.0;
const FEATURE_BIAS: f32 = -10.0;
/// Where a layered liquid's layers meet, of the way up.
const LAYER: f32 = 0.45;
/// How far towards white the far inside of a see-through liquid goes: its
/// near side blends over its own colour, not white glass, which washed it
/// out (owner, 2026-11-17), so being see-through shows as a lighter depth.
const SEE_THROUGH_PALE: f32 = 0.6;
/// How dark the liquid gets towards its edges, where light crosses more of it,
/// and how light its top is.
const EDGE_DARK: f32 = 0.45;
const TOP_LIGHT: f32 = 1.15;

/// The liquid's parts, in the bottle's space: `view` is the way to the camera there.
pub(super) fn liquid_parts(
    shape: Shape,
    look: &LiquidLook,
    view: Vec3,
    transform: Transform,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Vec<Part> {
    let alpha = alpha(look);
    let mut part = |mesh: Mesh, material: StandardMaterial| Part {
        mesh: meshes.add(mesh),
        material: materials.add(material),
        transform,
    };
    let bands = if look.layered {
        vec![
            (band(shape, 0.0, LAYER), second_colour(look), LOWER_BIAS),
            (band(shape, LAYER, 1.0), main_colour(look), BODY_BIAS),
        ]
    } else {
        vec![(band(shape, 0.0, 1.0), main_colour(look), BODY_BIAS)]
    };
    let mut parts = Vec::new();
    for (surface, colour, bias) in &bands {
        let mesh = mesh_of(surface, view);
        let inside = colour.mix(&Color::WHITE, (1.0 - alpha) * SEE_THROUGH_PALE);
        parts.push(part(mesh.clone(), body_material(inside, None)));
        parts.push(part(mesh, body_material(*colour, Some((alpha, *bias)))));
    }
    let body: Vec<Surface> = bands.into_iter().map(|(surface, ..)| surface).collect();
    let (bottom, top) = levels(shape);
    let Some(front) = Front::of(&body, view, bottom, top) else {
        return parts;
    };
    let main = main_colour(look);
    for (feature, balls) in features(look, &front) {
        if balls.is_empty() {
            continue;
        }
        let (colour, glow) = match feature {
            Feature::Fizz => (main.mix(&Color::WHITE, 0.9), false),
            Feature::Bubbles => (main.mix(&Color::WHITE, 0.8), false),
            Feature::Sparkles => (Color::srgb(1.0, 0.97, 0.8), true),
            Feature::Swirl => (second_colour(look), false),
            Feature::Blobs => (blob_colour(look, look.blobs.unwrap_or(look.main)), false),
        };
        parts.push(part(balls_mesh(&balls), feature_material(colour, glow)));
    }
    parts
}

/// The body's material: unlit, its own colour shaded in its mesh, as light
/// coming through it would be (lit, the picture's lights washed it pale). Its
/// far inside is solid (`see_through` none); its near side blends at that alpha
/// with that depth bias.
fn body_material(colour: Color, see_through: Option<(f32, f32)>) -> StandardMaterial {
    let (alpha, alpha_mode, cull, bias) = match see_through {
        None => (1.0, AlphaMode::Opaque, Face::Front, 0.0),
        Some((alpha, bias)) => (alpha, AlphaMode::Blend, Face::Back, bias),
    };
    StandardMaterial {
        base_color: colour.with_alpha(alpha),
        unlit: true,
        alpha_mode,
        cull_mode: Some(cull),
        depth_bias: bias,
        ..default()
    }
}

/// Bubbles, fizz, swirls and blobs, lit and nearly solid; `glow` for sparkles,
/// which light themselves.
fn feature_material(colour: Color, glow: bool) -> StandardMaterial {
    StandardMaterial {
        base_color: colour.with_alpha(if glow { 1.0 } else { 0.85 }),
        emissive: if glow {
            LinearRgba::rgb(2.0, 1.9, 1.4)
        } else {
            LinearRgba::BLACK
        },
        perceptual_roughness: 0.25,
        reflectance: 0.5,
        alpha_mode: AlphaMode::Blend,
        depth_bias: FEATURE_BIAS,
        ..default()
    }
}

/// A surface's mesh, shaded in its colours: darker towards the edges seen from
/// `view` (from either side, for the far inside too), lighter on top.
fn mesh_of(surface: &Surface, view: Vec3) -> Mesh {
    let (positions, normals, indices) = surface.triangles();
    let colours: Vec<[f32; 4]> = normals
        .iter()
        .map(|normal| {
            let shade = if normal.y > 0.7 {
                TOP_LIGHT
            } else {
                1.0 - EDGE_DARK * (1.0 - normal.dot(view).abs())
            };
            [shade, shade, shade, 1.0]
        })
        .collect();
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colours)
    .with_inserted_indices(Indices::U32(indices))
}
