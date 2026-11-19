//! Kindling without Bevy: three sticks split from a log of its wood, each a
//! narrow wedge with a strip of its bark on the outside, split faces of plain
//! grained wood and ringed ends, lying side by side along x, in metres.

use std::f32::consts::PI;

use shared::material::Material;

use super::bark::bark_of;
use super::mesh::{LOG_RADIUS, LogMesh};
use super::pattern::{bark_colour, end_colour, mix, noise};

/// How long a stick is, and the share of the radius its inside is cut off at.
pub const STICK_LENGTH: f32 = 0.45;
const INNER: f32 = 0.5;
/// How wide a wedge is, in radians.
const WEDGE: f32 = 0.5;
/// Each stick: which way its bark faces (radians round x), where it lies
/// across (z, metres), and how far it is turned on the ground (radians).
const STICKS: [(f32, f32, f32); 3] = [(0.3, -0.1, 0.1), (-0.6, 0.0, -0.06), (PI * 0.25, 0.1, 0.04)];
/// Points along, across and out.
const ALONG: usize = 24;
const ACROSS: usize = 10;

/// Three sticks of `wood`'s kindling.
pub fn kindling_mesh(wood: Material) -> LogMesh {
    let look = bark_of(wood);
    let heart = wood.wood().map_or([200, 160, 110], |wood| wood.heartwood);
    let mut mesh = LogMesh::default();
    for (index, &(facing, across, turn)) in STICKS.iter().enumerate() {
        let first = mesh.positions.len();
        let seed = 0x5EED_0000 + index as u32;
        let (from, to) = (facing - WEDGE / 2.0, facing + WEDGE / 2.0);
        let inner = LOG_RADIUS * INNER;
        let middle = (LOG_RADIUS + inner) / 2.0;
        // Moved so its middle lies on the ground at `across`.
        let shift = [0.0, -middle * facing.cos(), across - middle * facing.sin()];
        let place = |x: f32, r: f32, a: f32| [x, r * a.cos() + shift[1], r * a.sin() + shift[2]];
        let inside = |at: [f32; 3]| {
            [
                at[0],
                shift[1] + middle * facing.cos(),
                shift[2] + middle * facing.sin(),
            ]
        };
        let half = STICK_LENGTH / 2.0;
        let x_at = |row: usize| -half + STICK_LENGTH * row as f32 / ALONG as f32;
        let a_at = |column: usize| from + (to - from) * column as f32 / ACROSS as f32;
        let r_at = |column: usize| inner + (LOG_RADIUS - inner) * column as f32 / ACROSS as f32;
        // The bark along the outside.
        mesh.grid((ALONG + 1, ACROSS + 1), false, inside, |row, column| {
            let (x, a) = (x_at(row), a_at(column));
            (
                place(x, LOG_RADIUS, a),
                bark_colour(&look, seed, a, x + half, 0.8),
            )
        });
        // The split faces and the inside, plain wood grained along.
        let grain = |x: f32, r: f32, a: f32| {
            let streak = noise(seed ^ 21, r * 400.0 + a * 30.0, x * 3.0, 4096);
            let out = r / LOG_RADIUS;
            let [red, green, blue, _] = end_colour(&look, heart, seed, a, out);
            let shade = 0.82 + 0.25 * streak;
            [red * shade, green * shade, blue * shade, 1.0]
        };
        for a in [from, to] {
            mesh.grid((ALONG + 1, ACROSS + 1), false, inside, |row, column| {
                let (x, r) = (x_at(row), r_at(column));
                (place(x, r, a), grain(x, r, a))
            });
        }
        mesh.grid((ALONG + 1, ACROSS + 1), false, inside, |row, column| {
            let (x, a) = (x_at(row), a_at(column));
            (place(x, inner, a), grain(x, inner, a))
        });
        // Its ends, ringed as a log's.
        for x in [-half, half] {
            mesh.grid((ACROSS + 1, ACROSS + 1), false, inside, |row, column| {
                let (r, a) = (r_at(row), a_at(column));
                let colour = match row == ACROSS {
                    true => {
                        let [red, green, blue] = mix(look.crack, look.crack, 0.0);
                        [red, green, blue, 1.0]
                    }
                    false => end_colour(&look, heart, seed, a, r / LOG_RADIUS),
                };
                (place(x, r, a), colour)
            });
        }
        turned(&mut mesh, first, turn);
    }
    mesh
}

/// Turns the points from `first` on, and their normals, by `angle` round y.
fn turned(mesh: &mut LogMesh, first: usize, angle: f32) {
    let (sin, cos) = angle.sin_cos();
    let turn = |[x, y, z]: [f32; 3]| [x * cos + z * sin, y, z * cos - x * sin];
    for at in &mut mesh.positions[first..] {
        *at = turn(*at);
    }
    for normal in &mut mesh.normals[first..] {
        *normal = turn(*normal);
    }
}
