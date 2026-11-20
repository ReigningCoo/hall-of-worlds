//! `Shell`: a fungus patch's model being put together from smooth pieces
//! (`item_art`'s fungus meshes, wood and earth), each moved into place, then
//! made into the maps' three meshes: shaded once in its own colours (the
//! maps' surfaces are unlit), its black outline (every point pushed out along
//! its normal, an inverted hull), and grey for where it is remembered.

use std::collections::HashMap;

use bevy::asset::RenderAssetUsages;
use bevy::mesh::{Indices, PrimitiveTopology, VertexAttributeValues};
use bevy::prelude::*;

use super::super::pieces::OUTLINE_WIDTH;
use super::super::triangles::{LIGHT, LIT, NEVER_RAISED, SHADOW};

/// Smooth pieces put together: points with normals and colours (linear), and
/// triangles as point numbers.
#[derive(Clone, Debug, Default)]
pub struct Shell {
    pub positions: Vec<Vec3>,
    pub normals: Vec<Vec3>,
    pub colours: Vec<Vec3>,
    pub indices: Vec<u32>,
}

impl Shell {
    /// Adds `mesh` moved by `place`; points without a colour are `fallback`
    /// (linear).
    pub fn add(&mut self, mesh: &Mesh, place: Transform, fallback: Vec3) {
        let Some(VertexAttributeValues::Float32x3(points)) =
            mesh.attribute(Mesh::ATTRIBUTE_POSITION)
        else {
            return;
        };
        let normals = match mesh.attribute(Mesh::ATTRIBUTE_NORMAL) {
            Some(VertexAttributeValues::Float32x3(normals)) => Some(normals),
            _ => None,
        };
        let colours = match mesh.attribute(Mesh::ATTRIBUTE_COLOR) {
            Some(VertexAttributeValues::Float32x4(colours)) => Some(colours),
            _ => None,
        };
        let matrix = place.to_matrix();
        // Normals follow a squashed piece by the inverse transpose.
        let turn = Mat3::from_mat4(matrix).inverse().transpose();
        let first = self.positions.len() as u32;
        for (i, point) in points.iter().enumerate() {
            self.positions
                .push(matrix.transform_point3(Vec3::from_array(*point)));
            let normal = normals.map_or(Vec3::Y, |n| Vec3::from_array(n[i]));
            self.normals.push((turn * normal).normalize_or(Vec3::Y));
            let colour = colours.map_or(fallback, |c| Vec3::new(c[i][0], c[i][1], c[i][2]));
            self.colours.push(colour);
        }
        match mesh.indices() {
            Some(indices) => (self.indices).extend(indices.iter().map(|i| first + i as u32)),
            None => (self.indices).extend(first..first + points.len() as u32),
        }
    }

    /// Shrinks the whole about the foot of its upright, if need be, so nothing
    /// reaches further than `reach` from it across the ground.
    pub fn fit(&mut self, reach: f32) {
        let most = (self.positions.iter())
            .map(|p| p.x.hypot(p.z))
            .fold(0.0f32, f32::max);
        if most > reach {
            let scale = reach / most;
            self.positions.iter_mut().for_each(|p| *p *= scale);
        }
    }

    /// Fewer points and triangles: every point within the same `grid`-sized
    /// box becomes one (their middle, normals and colours averaged), and
    /// triangles left with no area are dropped. The item pictures' meshes are
    /// made for close views; on the maps their fine detail is too small to see.
    pub fn simplify(&mut self, grid: f32) {
        let mut boxes: HashMap<[i32; 4], u32> = HashMap::new();
        let mut sums: Vec<(Vec3, Vec3, Vec3, f32)> = Vec::new();
        let mut moved = Vec::with_capacity(self.positions.len());
        for (i, point) in self.positions.iter().enumerate() {
            // Points facing apart (a thin plate's two sides) stay apart.
            let n = self.normals[i];
            let facing =
                i32::from(n.x > 0.0) | i32::from(n.y > 0.0) << 1 | i32::from(n.z > 0.0) << 2;
            let [x, y, z] = (*point / grid).round().as_ivec3().to_array();
            let at = *boxes.entry([x, y, z, facing]).or_insert_with(|| {
                sums.push((Vec3::ZERO, Vec3::ZERO, Vec3::ZERO, 0.0));
                sums.len() as u32 - 1
            });
            let sum = &mut sums[at as usize];
            sum.0 += *point;
            sum.1 += self.normals[i];
            sum.2 += self.colours[i];
            sum.3 += 1.0;
            moved.push(at);
        }
        let mut indices = Vec::with_capacity(self.indices.len());
        let mut kept = std::collections::HashSet::new();
        for triangle in self.indices.chunks(3) {
            let [a, b, c] = [0, 1, 2].map(|k| moved[triangle[k] as usize]);
            let mut key = [a, b, c];
            key.sort_unstable();
            if a != b && b != c && a != c && kept.insert(key) {
                indices.extend([a, b, c]);
            }
        }
        self.positions = sums.iter().map(|s| s.0 / s.3).collect();
        self.normals = (sums.iter()).map(|s| s.1.normalize_or(Vec3::Y)).collect();
        self.colours = sums.iter().map(|s| s.2 / s.3).collect();
        self.indices = indices;
    }

    /// Whether nothing has been added.
    pub fn is_empty(&self) -> bool {
        self.indices.is_empty()
    }

    /// How many triangles there are.
    #[cfg(test)]
    pub fn len(&self) -> usize {
        self.indices.len() / 3
    }

    /// Shaded in its own colours.
    pub fn fill(&self) -> Mesh {
        let colours = (self.normals.iter().zip(&self.colours))
            .map(|(normal, colour)| shaded(*colour, *normal))
            .collect();
        self.mesh(self.positions.clone(), colours)
    }

    /// Its black outline: grown by `OUTLINE_WIDTH` (`thin` of it for fine
    /// things like lichens), drawn inside out.
    pub fn outline(&self, thin: f32) -> Mesh {
        let grown = (self.positions.iter().zip(&self.normals))
            .map(|(point, normal)| *point + *normal * OUTLINE_WIDTH * thin)
            .collect();
        self.mesh(grown, vec![[0.0, 0.0, 0.0, 1.0]; self.positions.len()])
    }

    /// Shaded in greys by each point's lightness, for memory (as trees').
    pub fn memory(&self) -> Mesh {
        let colours = (self.normals.iter().zip(&self.colours))
            .map(|(normal, colour)| {
                let [r, g, b, _] = shaded(*colour, *normal);
                let light = 0.3 * r + 0.59 * g + 0.11 * b;
                let shade = (0.2 + light * 0.6).min(0.45);
                [shade, shade, shade, 1.0]
            })
            .collect();
        self.mesh(self.positions.clone(), colours)
    }

    /// A mesh of these points and colours, for the dungeon's surface material
    /// (as `Triangles::into_mesh`).
    fn mesh(&self, positions: Vec<Vec3>, colours: Vec<[f32; 4]>) -> Mesh {
        let count = positions.len();
        let positions: Vec<[f32; 3]> = positions.iter().map(|p| p.to_array()).collect();
        let normals: Vec<[f32; 3]> = self.normals.iter().map(|n| n.to_array()).collect();
        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colours)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0.0f32; 2]; count])
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_1, vec![[NEVER_RAISED, 0.0]; count])
        .with_inserted_indices(Indices::U32(self.indices.clone()))
    }
}

/// `colour` lit by the plants' fixed light from the way `normal` faces.
fn shaded(colour: Vec3, normal: Vec3) -> [f32; 4] {
    let lit = colour * (SHADOW + LIT * normal.dot(LIGHT.normalize()).max(0.0));
    [lit.x, lit.y, lit.z, 1.0]
}
