//! `Builder`: a coin's triangles as they are made, flat quads for its rim and
//! edge and smooth rows for its face, each point with a shade that darkens the
//! metal (the face's low field, the edge) so the emboss reads when small.

use bevy::asset::RenderAssetUsages;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;

#[derive(Default)]
pub(super) struct Builder {
    pub(super) positions: Vec<Vec3>,
    pub(super) normals: Vec<Vec3>,
    pub(super) shades: Vec<f32>,
    pub(super) indices: Vec<u32>,
}

impl Builder {
    /// A flat four-sided face, turned to face `outward`.
    pub(super) fn quad(&mut self, corners: [Vec3; 4], outward: Vec3, shade: f32) {
        let [a, b, c, d] = corners;
        let mut normal = (b - a).cross(d - a) + (d - c).cross(b - c);
        let flip = normal.dot(outward) < 0.0;
        if flip {
            normal = -normal;
        }
        let first = self.positions.len() as u32;
        self.positions.extend(corners);
        self.normals.extend([normal.normalize_or(outward); 4]);
        self.shades.extend([shade; 4]);
        let order = if flip {
            [0, 2, 1, 0, 3, 2]
        } else {
            [0, 1, 2, 0, 2, 3]
        };
        self.indices.extend(order.map(|k| first + k));
    }

    /// Points with their own normals and shades, and triangles between them by
    /// their places in `points`, turned to face `outward`.
    pub(super) fn smooth(
        &mut self,
        points: Vec<(Vec3, f32)>,
        triangles: impl Iterator<Item = [u32; 3]>,
        outward: Vec3,
    ) {
        let first = self.positions.len() as u32;
        let mut normals = vec![Vec3::ZERO; points.len()];
        let mut indices = Vec::new();
        for mut t in triangles {
            let [a, b, c] = t.map(|k| points[k as usize].0);
            let mut face = (b - a).cross(c - a);
            if face.dot(outward) < 0.0 {
                t.swap(1, 2);
                face = -face;
            }
            t.iter().for_each(|&k| normals[k as usize] += face);
            indices.extend(t.map(|k| first + k));
        }
        self.normals
            .extend(normals.iter().map(|n| n.normalize_or(outward)));
        for (point, shade) in points {
            self.positions.push(point);
            self.shades.push(shade);
        }
        self.indices.extend(indices);
    }

    pub(super) fn mesh(self) -> Mesh {
        let colours: Vec<[f32; 4]> = self.shades.iter().map(|&s| [s, s, s, 1.0]).collect();
        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, self.positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, self.normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colours)
        .with_inserted_indices(Indices::U32(self.indices))
    }
}
