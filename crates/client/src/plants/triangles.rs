//! `Triangles`: a model as loose triangles, each with its own flat normal and
//! colour, shaded once as it is built (the maps' surfaces are unlit).

use bevy::asset::RenderAssetUsages;
use bevy::mesh::PrimitiveTopology;
use bevy::prelude::*;

/// Where the light seems to come from on plants: above, a little to the
/// south-east, so every side shows a different shade.
pub(super) const LIGHT: Vec3 = Vec3::new(0.35, 1.0, 0.55);

/// How dark the side away from the light is, and how much the light adds.
pub(super) const SHADOW: f32 = 0.55;
pub(super) const LIT: f32 = 0.45;

/// The sight mask's ground height for every vertex (`sight_mask.wesl`): far
/// above anything, so no face counts as a raised top, which the mask never
/// cuts.
pub(super) const NEVER_RAISED: f32 = 1.0e9;

/// Loose triangles with their colours.
#[derive(Clone, Debug, Default)]
pub struct Triangles {
    pub positions: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub colours: Vec<[f32; 4]>,
}

impl Triangles {
    /// A triangle facing away from `inside`, shaded by which way it faces
    /// when `shade`.
    pub fn add(&mut self, corners: [Vec3; 3], inside: Vec3, colour: Vec3, shade: bool) {
        let [a, mut b, mut c] = corners;
        let mut normal = (b - a).cross(c - a).normalize_or_zero();
        if normal.dot((a + b + c) / 3.0 - inside) < 0.0 {
            std::mem::swap(&mut b, &mut c);
            normal = -normal;
        }
        let lit = if shade {
            colour * (SHADOW + LIT * normal.dot(LIGHT.normalize()).max(0.0))
        } else {
            colour
        };
        for corner in [a, b, c] {
            self.positions.push(corner.to_array());
            self.normals.push(normal.to_array());
            self.colours.push([lit.x, lit.y, lit.z, 1.0]);
        }
    }

    /// Adds `other`'s triangles turned `turn` about the upright and moved by
    /// `offset`, keeping their shading.
    pub fn append_moved(&mut self, other: &Self, offset: Vec3, turn: f32) {
        let rotation = Quat::from_rotation_y(turn);
        let moved = |p: &[f32; 3]| (rotation * Vec3::from_array(*p) + offset).to_array();
        let turned = |n: &[f32; 3]| (rotation * Vec3::from_array(*n)).to_array();
        self.positions.extend(other.positions.iter().map(moved));
        self.normals.extend(other.normals.iter().map(turned));
        self.colours.extend_from_slice(&other.colours);
    }

    /// How many triangles there are.
    #[cfg(test)]
    pub fn len(&self) -> usize {
        self.positions.len() / 3
    }

    #[cfg(test)]
    pub fn is_empty(&self) -> bool {
        self.positions.is_empty()
    }

    /// As a mesh for the dungeon's surface material: picture coordinates
    /// unused, the second ones keeping the mask from taking anything for a
    /// raised top.
    pub fn into_mesh(self) -> Mesh {
        let count = self.positions.len();
        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, self.positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, self.normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, self.colours)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0.0f32; 2]; count])
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_1, vec![[NEVER_RAISED, 0.0]; count])
    }
}
