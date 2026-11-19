//! A log's shape without Bevy, so the ground's log piles can draw it too: a
//! metre long along x, 30 cm across, its bark's relief and colour on the side
//! (`pattern.rs`) and its rings on the flat ends, in metres, round the x axis.

use std::f32::consts::TAU;

use shared::material::Material;

use super::bark::bark_of;
use super::pattern::{bark_colour, depth, end_colour, relief};

/// A log's length and radius, in metres (30 × 30 × 100 cm).
pub const LOG_LENGTH: f32 = 1.0;
pub const LOG_RADIUS: f32 = 0.15;

/// How finely a log is built: points round, rings along, and circles out on
/// each end.
#[derive(Clone, Copy, Debug)]
pub struct Detail {
    pub around: usize,
    pub along: usize,
    pub out: usize,
}

/// For pictures, and for logs on the ground, which are small on screen.
pub const PICTURE_DETAIL: Detail = Detail {
    around: 96,
    along: 64,
    out: 48,
};
pub const GROUND_DETAIL: Detail = Detail {
    around: 28,
    along: 10,
    out: 8,
};

/// Points and triangles, ready for a mesh.
#[derive(Clone, Debug, Default)]
pub struct LogMesh {
    pub positions: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub colours: Vec<[f32; 4]>,
    pub indices: Vec<u32>,
}

impl LogMesh {
    /// A grid of `rows` × `columns` points (`point(row, column)`: where and
    /// what colour), joined into triangles facing away from `inside`, with
    /// smooth normals; `wrap` joins the last column to the first.
    pub(super) fn grid(
        &mut self,
        (rows, columns): (usize, usize),
        wrap: bool,
        inside: impl Fn([f32; 3]) -> [f32; 3],
        point: impl Fn(usize, usize) -> ([f32; 3], [f32; 4]),
    ) {
        let first = self.positions.len();
        for row in 0..rows {
            for column in 0..columns {
                let (at, colour) = point(row, column);
                self.positions.push(at);
                self.colours.push(colour);
                self.normals.push([0.0; 3]);
            }
        }
        let index = |row: usize, column: usize| (first + row * columns + column % columns) as u32;
        let spans = if wrap { columns } else { columns - 1 };
        for row in 0..rows - 1 {
            for column in 0..spans {
                let corners = [
                    index(row, column),
                    index(row + 1, column),
                    index(row + 1, column + 1),
                    index(row, column + 1),
                ];
                self.triangle([corners[0], corners[1], corners[2]], &inside);
                self.triangle([corners[0], corners[2], corners[3]], &inside);
            }
        }
        for normal in &mut self.normals[first..] {
            *normal = unit(*normal);
        }
    }

    /// One triangle, turned to face away from `inside` of its middle, its
    /// normal added to its corners'.
    fn triangle(&mut self, mut corners: [u32; 3], inside: impl Fn([f32; 3]) -> [f32; 3]) {
        let [a, b, c] = corners.map(|i| self.positions[i as usize]);
        let mut normal = cross(sub(b, a), sub(c, a));
        let middle = [0, 1, 2].map(|i| (a[i] + b[i] + c[i]) / 3.0);
        if dot(normal, sub(middle, inside(middle))) < 0.0 {
            corners.swap(1, 2);
            normal = normal.map(|n| -n);
        }
        for i in corners {
            let n = &mut self.normals[i as usize];
            *n = [n[0] + normal[0], n[1] + normal[1], n[2] + normal[2]];
        }
        self.indices.extend(corners);
    }
}

/// A log of `wood`, built to `detail`; `seed` varies its bark between logs.
pub fn log_mesh(wood: Material, detail: Detail, seed: u32) -> LogMesh {
    let look = bark_of(wood);
    let heart = wood.wood().map_or([200, 160, 110], |wood| wood.heartwood);
    let mut mesh = LogMesh::default();
    let half = LOG_LENGTH / 2.0;
    let angle = |column: usize| column as f32 / detail.around as f32 * TAU;
    let radius = |column: usize, x: f32| {
        let raised = relief(&look, seed, angle(column), x + half);
        (
            LOG_RADIUS * (1.0 - depth(look.style) * (1.0 - raised)),
            raised,
        )
    };
    let axis = |at: [f32; 3]| [at[0], 0.0, 0.0];
    mesh.grid(
        (detail.along + 1, detail.around),
        true,
        axis,
        |row, column| {
            let x = -half + LOG_LENGTH * row as f32 / detail.along as f32;
            let (r, raised) = radius(column, x);
            let a = angle(column);
            let colour = bark_colour(&look, seed, a, x + half, raised);
            ([x, r * a.cos(), r * a.sin()], colour)
        },
    );
    // The bark's own thickness at the ends, darker than the wood.
    let wood_share = 0.9;
    for side in [-1.0_f32, 1.0] {
        let x = side * half;
        let behind = |_: [f32; 3]| [x - side, 0.0, 0.0];
        mesh.grid(
            (detail.out + 2, detail.around),
            true,
            behind,
            |row, column| {
                let a = angle(column);
                let (rim, _) = radius(column, x);
                let (r, colour) = match row {
                    0 => (0.0, end_colour(&look, heart, seed, a, 0.0)),
                    _ if row <= detail.out => {
                        let out = row as f32 / detail.out as f32;
                        let r = out * LOG_RADIUS * wood_share;
                        (r, end_colour(&look, heart, seed, a, out))
                    }
                    _ => {
                        let [r, g, b] = super::pattern::mix(look.crack, look.crack, 0.0);
                        (rim, [r, g, b, 1.0])
                    }
                };
                ([x, r * a.cos(), r * a.sin()], colour)
            },
        );
    }
    mesh
}

fn sub(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn cross(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn dot(a: [f32; 3], b: [f32; 3]) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

pub(super) fn unit(v: [f32; 3]) -> [f32; 3] {
    let length = dot(v, v).sqrt().max(1e-9);
    v.map(|c| c / length)
}
