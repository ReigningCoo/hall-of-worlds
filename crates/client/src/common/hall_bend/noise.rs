//! The smooth noise the Hall's bend and twist come from: layers of knots,
//! eased between with smoothstep, by each column's place in the whole Hall.

use shared::hall::HALL_LENGTH;
use shared::rng::Seed;

/// The sideways swing's layers: columns between their knots, and the most
/// each moves the walkway either way, in cells. Each layer turns no faster
/// than `3 × most ÷ columns` a column (smoothstep's steepest is 1.5 × the
/// difference between two knots), so together under 1/32 of a cell, a pixel
/// of the marble's picture.
pub(super) const SIDEWAYS: [(i64, f32); 2] = [(256, 1.1), (64, 0.35)];
/// The same for the height: under 1/64 of a cell a column.
pub(super) const UP: [(i64, f32); 2] = [(256, 0.3), (64, 0.1)];
/// The same for the twist round the middle line, in degrees: at most 7.5°
/// either way, so two places, or the player and a place, differ by at most
/// 15°, turning under 0.8° a column (the owner's limit is about 1°).
pub(super) const TWIST: [(i64, f32); 3] = [(64, 5.0), (16, 2.0), (8, 0.5)];
/// Where the knots' values come from (any fixed number: the same Hall for all).
const SEED: Seed = Seed::new(0x4841_4c4c_4245_4e44);

/// The layers' swings added up at a column (`part` of the way to the next).
/// `which` keeps each swing's knots apart.
pub(super) fn swing(layers: &[(i64, f32)], which: i64, column: i64, part: f32) -> f32 {
    (layers.iter().zip(0..))
        .map(|(&(every, most), layer)| {
            let knots = HALL_LENGTH / every;
            let knot = column / every;
            let along = ((column % every) as f32 + part) / every as f32;
            let eased = along * along * (3.0 - 2.0 * along);
            let from = knot_value(which, layer, knot);
            let to = knot_value(which, layer, (knot + 1) % knots);
            most * (from + (to - from) * eased)
        })
        .sum()
}

/// A knot's value, from -1 to 1.
fn knot_value(which: i64, layer: i64, knot: i64) -> f32 {
    let hash = SEED.at(&[which, layer, knot]);
    (hash >> 40) as f32 / (1u64 << 23) as f32 - 1.0
}
