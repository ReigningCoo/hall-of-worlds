//! Logs lying on the ground (`Load::Logs`, woodcutting plan PR 4): one, two
//! side by side, or three or more as two with one on top, of `item_art`'s
//! log (fewer points round, as they are small on screen), drawn to the same
//! scale as casks; with outlines grown out from each log's own middle, so
//! they have no cracks at the ends.

use shared::material::Material;

use super::bag::Model;
use crate::item_art::{GROUND_DETAIL, LOG_LENGTH, LOG_RADIUS, LogMesh, log_mesh};
use crate::view3d::MODEL_SCALE;

/// World units a metre of log is drawn: as casks are.
const UNITS_PER_METRE: f32 = MODEL_SCALE * 1000.0 * 1.3;

/// How far apart the logs' middles are, in radii: a little more than
/// touching, so one's outline never hides another.
const GAP: f32 = 1.12;

/// Where each log of a pile of `count` lies (x and z across the ground, y up
/// to its middle, in metres) and how far it is turned (radians round y).
fn placed(count: u8) -> Vec<([f32; 3], f32)> {
    let r = LOG_RADIUS;
    match count {
        0 | 1 => vec![([0.0, r, 0.0], 0.0)],
        2 => vec![([0.04, r, -r * GAP], 0.05), ([-0.05, r, r * GAP], -0.04)],
        _ => vec![
            ([0.04, r, -r * GAP], 0.05),
            ([-0.05, r, r * GAP], -0.04),
            ([0.0, r + r * GAP * 3.0_f32.sqrt(), 0.0], 0.03),
        ],
    }
}

/// How tall a pile of `count` logs is, and how far it reaches from its
/// middle, in world units.
pub(super) fn pile_units(count: u8) -> (f32, f32) {
    let high = match count {
        0 | 1 => 2.0 * LOG_RADIUS,
        2 => 2.0 * LOG_RADIUS,
        _ => LOG_RADIUS * (2.0 + GAP * 3.0_f32.sqrt()),
    };
    (high * UNITS_PER_METRE, LOG_LENGTH / 2.0 * UNITS_PER_METRE)
}

/// The pile to draw, and the same with normals pointing out from each log's
/// middle, to grow its outlines from.
pub(super) fn log_models(wood: Material, count: u8) -> (Model, Model) {
    let mut lit = Model::default();
    let mut for_outline = Model::default();
    for (index, (at, turn)) in placed(count).into_iter().enumerate() {
        let log = log_mesh(wood, GROUND_DETAIL, index as u32 + 1);
        let first = lit.positions.len() as u32;
        let (sin, cos) = turn.sin_cos();
        let turned = |[x, y, z]: [f32; 3]| [x * cos + z * sin, y, z * cos - x * sin];
        let LogMesh {
            positions,
            normals,
            colours,
            indices,
        } = log;
        let half = LOG_LENGTH / 2.0;
        for (point, normal) in positions.iter().zip(&normals) {
            let [x, y, z] = turned(*point);
            let place = [x + at[0], y + at[1], z + at[2]];
            lit.positions.push(place.map(|axis| axis * UNITS_PER_METRE));
            lit.normals.push(turned(*normal));
            let outward = [
                point[0] / half,
                point[1] / LOG_RADIUS,
                point[2] / LOG_RADIUS,
            ];
            for_outline.normals.push(turned(normalised(outward)));
        }
        lit.colours.extend(colours);
        lit.indices
            .extend(indices.iter().map(|index| index + first));
    }
    lit.uvs = vec![[0.0, 0.0]; lit.positions.len()];
    for_outline.positions = lit.positions.clone();
    for_outline.uvs = lit.uvs.clone();
    for_outline.colours = lit.colours.clone();
    for_outline.indices = lit.indices.clone();
    (lit, for_outline)
}

fn normalised([x, y, z]: [f32; 3]) -> [f32; 3] {
    let length = (x * x + y * y + z * z).sqrt().max(1e-6);
    [x / length, y / length, z / length]
}
