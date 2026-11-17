//! A coin's parts: built flat, then turned to face the camera and tilted back
//! `TILT`, so its face and the reeded edge below it both show, and centred in
//! its picture at its own size.

use bevy::prelude::*;
use shared::material::Material;

use super::super::bottle::picture_axes;
use super::super::{CAMERA_AT, Part};
use super::mesh::Builder;
use super::{Coin, coo, disc, face};

/// How far the coin's top is tilted back from facing the camera.
const TILT: f32 = 25.0_f32.to_radians();

/// The coin of this metal (`Look::Coin`).
pub(in crate::item_art) fn coin_parts(
    metal: Material,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Vec<Part> {
    let mesh = coin_mesh(Coin::of(metal));
    let transform = placed(&mesh.positions);
    vec![Part {
        mesh: meshes.add(mesh.mesh()),
        material: materials.add(super::super::ring::metal_material(metal, None)),
        transform,
    }]
}

/// The coin's triangles, lying flat with its face up.
pub(super) fn coin_mesh(coin: Coin) -> Builder {
    let mut mesh = Builder::default();
    disc::blank(&coin, &mut mesh);
    face::face(&coin, &coo::coo(), &mut mesh);
    mesh
}

/// Its face turned towards the camera and tilted back, the coo's head up the
/// picture, and its middle in the picture's middle.
pub(super) fn placed(points: &[Vec3]) -> Transform {
    let (right, up) = picture_axes();
    let toward = CAMERA_AT.normalize();
    let facing = toward * TILT.cos() + up * TILT.sin();
    let rotation = Quat::from_mat3(&Mat3::from_cols(right, facing, right.cross(facing)));
    let (mut low, mut high) = (Vec2::splat(f32::MAX), Vec2::splat(f32::MIN));
    for point in points {
        let at = rotation * *point;
        let on = Vec2::new(at.dot(right), at.dot(up));
        low = low.min(on);
        high = high.max(on);
    }
    let middle = (low + high) / 2.0;
    Transform {
        translation: -(right * middle.x + up * middle.y),
        rotation,
        scale: Vec3::ONE,
    }
}
