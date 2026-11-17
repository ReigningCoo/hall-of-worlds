//! The coin's face inside the rim: a fine round grid over the field, lifted by
//! the coo's relief and shaded darker where it is low.

use bevy::prelude::*;

use super::Coin;
use super::disc::{AROUND, FIELD, RIM_HIGH, angle};
use super::mesh::Builder;
use super::outline::{Outline, relief};

/// Rings of points out from the middle to the rim.
const RINGS: usize = 60;
/// How high the highest of the emboss stands above the field, against the
/// thickness: a little under the rim, as a struck coin's is.
pub(super) const RELIEF: f32 = 0.32;
/// The field's shade, and how much lighter the highest of the emboss is.
const FIELD_SHADE: f32 = 0.6;
const RAISED_SHADE: f32 = 0.4;

/// The face, from these outlines.
pub(super) fn face(coin: &Coin, outlines: &[Outline], mesh: &mut Builder) {
    let field = coin.thickness / 2.0 - RIM_HIGH * coin.thickness;
    let reach = FIELD * coin.radius;
    // The face's units are radius 1, y up; it lies with up along -z, the way
    // the picture's camera has it.
    let point = |at: Vec2| {
        let height = relief(outlines, at);
        let place = Vec3::new(
            at.x * reach,
            field + height * RELIEF * coin.thickness,
            -at.y * reach,
        );
        (place, FIELD_SHADE + RAISED_SHADE * height)
    };
    let mut points = vec![point(Vec2::ZERO)];
    for ring in 1..=RINGS {
        let r = ring as f32 / RINGS as f32;
        points.extend((0..AROUND).map(|i| point(Vec2::from_angle(-angle(i)) * r)));
    }
    let at = |ring: usize, i: usize| (1 + (ring - 1) * AROUND + i % AROUND) as u32;
    let middle = (0..AROUND).map(move |i| [0, at(1, i), at(1, i + 1)]);
    let rows = (1..RINGS).flat_map(move |ring| {
        (0..AROUND).flat_map(move |i| {
            let (a, b) = (at(ring, i), at(ring, i + 1));
            let (c, d) = (at(ring + 1, i + 1), at(ring + 1, i));
            [[a, d, c], [a, c, b]]
        })
    });
    mesh.smooth(points, middle.chain(rows), Vec3::Y);
}
