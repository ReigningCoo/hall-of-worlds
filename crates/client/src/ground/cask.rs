//! A cask standing on the ground (`Load::Cask`): `item_art`'s cask, drawn to
//! the same scale as creatures' models (a little larger, as bags are), with
//! outlines grown out from its middle so they have no cracks at the staves.

use shared::cask::CaskLook;

use super::bag::Model;
use crate::item_art::{CaskMesh, cask_height, cask_mesh};
use crate::view3d::MODEL_SCALE;

/// World units a metre of cask is drawn: creatures' scale, a little larger.
const UNITS_PER_METRE: f32 = MODEL_SCALE * 1000.0 * 1.3;

/// How tall a cask of this look stands on the ground, in world units.
pub(super) fn cask_units(look: CaskLook) -> f32 {
    cask_height(look.size) * UNITS_PER_METRE
}

/// The cask to draw, and the same with normals pointing out from its middle,
/// to grow its outlines from.
pub(super) fn cask_models(look: CaskLook) -> (Model, Model) {
    let CaskMesh {
        positions,
        normals,
        colours,
        indices,
    } = cask_mesh(look);
    let positions: Vec<[f32; 3]> = (positions.iter())
        .map(|point| point.map(|axis| axis * UNITS_PER_METRE))
        .collect();
    let height = cask_units(look);
    let radius = (positions.iter())
        .map(|[x, _, z]| (x * x + z * z).sqrt())
        .fold(0.0_f32, f32::max);
    let outward = (positions.iter())
        .map(|&[x, y, z]| normalised([x / radius, (y - height / 2.0) / (height / 2.0), z / radius]))
        .collect();
    let uvs = vec![[0.0, 0.0]; positions.len()];
    let lit = Model {
        positions: positions.clone(),
        normals,
        uvs: uvs.clone(),
        colours: colours.clone(),
        indices: indices.clone(),
    };
    let for_outline = Model {
        positions,
        normals: outward,
        uvs,
        colours,
        indices,
    };
    (lit, for_outline)
}

fn normalised([x, y, z]: [f32; 3]) -> [f32; 3] {
    let length = (x * x + y * y + z * z).sqrt().max(1e-6);
    [x / length, y / length, z / length]
}
