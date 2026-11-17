//! Many small balls in one mesh: bubbles, fizz, sparkles, blobs and swirls.

use bevy::asset::RenderAssetUsages;
use bevy::mesh::{Indices, PrimitiveTopology, VertexAttributeValues};
use bevy::prelude::*;

/// One ball: where, turned how, and stretched how far along each of its axes.
#[derive(Clone, Copy, Debug)]
pub(super) struct Ball {
    pub(super) at: Vec3,
    pub(super) turn: Quat,
    pub(super) size: Vec3,
}

impl Ball {
    /// A round ball.
    pub(super) fn round(at: Vec3, radius: f32) -> Self {
        Self {
            at,
            turn: Quat::IDENTITY,
            size: Vec3::splat(radius),
        }
    }
}

/// The balls as one mesh.
pub(super) fn balls_mesh(balls: &[Ball]) -> Mesh {
    let unit = Sphere::new(1.0).mesh().ico(1).expect("a small icosphere");
    let read = |attribute| match unit.attribute(attribute) {
        Some(VertexAttributeValues::Float32x3(values)) => values.clone(),
        _ => Vec::new(),
    };
    let (unit_positions, unit_normals) =
        (read(Mesh::ATTRIBUTE_POSITION), read(Mesh::ATTRIBUTE_NORMAL));
    let unit_indices: Vec<u32> = unit
        .indices()
        .into_iter()
        .flat_map(|i| i.iter())
        .map(|i| i as u32)
        .collect();
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut indices = Vec::new();
    for ball in balls {
        let start = positions.len() as u32;
        for (p, n) in unit_positions.iter().zip(&unit_normals) {
            let p = ball.at + ball.turn * (Vec3::from(*p) * ball.size);
            let n = ball.turn * (Vec3::from(*n) / ball.size).normalize_or_zero();
            positions.push(p);
            normals.push(n);
        }
        indices.extend(unit_indices.iter().map(|i| start + i));
    }
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_indices(Indices::U32(indices))
}
