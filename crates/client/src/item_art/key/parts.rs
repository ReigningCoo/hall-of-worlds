//! A key put together from its bow, shank and bit, and laid across its picture.

use std::f32::consts::FRAC_PI_4;

use bevy::prelude::*;

use super::super::Part;
use super::look::{KeyLook, KeyMetal};
use super::piece::{Finish, Piece};
use super::{bit, bow, shank};

/// How much of the picture's `VIEW_WIDTH` of 2.5 a key may fill, each way.
pub(super) const FIT: f32 = 2.2;

/// Every piece of the key, in its own space.
pub(super) fn pieces(look: &KeyLook) -> Vec<Piece> {
    let (mut pieces, reach) = bow::bow(look);
    let (shank_pieces, shank) = shank::shank(look, reach, bit::bit_width(look));
    pieces.extend(shank_pieces);
    pieces.extend(bit::bit(look, shank));
    pieces
}

/// Where a key goes in its picture: diagonally across the square, bow at the
/// bottom left, so a long shank fits, scaled to fill it.
pub(super) fn fitted(pieces: &[Piece]) -> Transform {
    let turned = Transform::from_rotation(Quat::from_rotation_y(FRAC_PI_4));
    let (mut low, mut high) = (Vec3::MAX, Vec3::MIN);
    for corner in pieces.iter().flat_map(|piece| piece.corners(turned)) {
        low = low.min(corner);
        high = high.max(corner);
    }
    let size = high - low;
    let scale = FIT / size.x.max(size.z);
    turned
        .with_scale(Vec3::splat(scale))
        .with_translation(-(low + high) / 2.0 * scale)
}

/// A key's picture (`Look::Key`).
pub(in super::super) fn key_parts(
    look: &KeyLook,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Vec<Part> {
    let pieces = pieces(look);
    placed(look, &pieces, fitted(&pieces), meshes, materials)
}

/// A key hung by its bow's middle at `at`, its shank along `at`'s x, scaled to
/// reach `long` from there to its tip (the Keyring's keys).
pub(in super::super) fn hung(
    look: &KeyLook,
    at: Transform,
    long: f32,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Vec<Part> {
    let pieces = pieces(look);
    placed(look, &pieces, hung_at(&pieces, at, long), meshes, materials)
}

/// Where `hung` places a key's pieces.
pub(super) fn hung_at(pieces: &[Piece], at: Transform, long: f32) -> Transform {
    let tip = pieces
        .iter()
        .flat_map(|piece| piece.corners(Transform::IDENTITY))
        .map(|corner| corner.x)
        .fold(0.0, f32::max);
    at * Transform::from_scale(Vec3::splat(long / tip))
}

/// A key's parts placed by `whole`.
fn placed(
    look: &KeyLook,
    pieces: &[Piece],
    whole: Transform,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Vec<Part> {
    let metal = materials.add(metal(look));
    let hole = materials.add(StandardMaterial {
        base_color: Color::srgb(0.04, 0.04, 0.04),
        perceptual_roughness: 1.0,
        ..default()
    });
    let part = |piece: &Piece| Part {
        mesh: meshes.add(piece.mesh()),
        material: match piece.finish {
            Finish::Metal => metal.clone(),
            Finish::Hole => hole.clone(),
        },
        transform: whole * piece.at,
    };
    pieces.iter().map(part).collect()
}

/// The key's metal, darker, duller and a little greener the more tarnished.
fn metal(look: &KeyLook) -> StandardMaterial {
    let [red, green, blue] = match look.metal {
        KeyMetal::Iron => [0.42, 0.42, 0.44],
        KeyMetal::DarkIron => [0.24, 0.23, 0.24],
        KeyMetal::Brass => [0.78, 0.6, 0.28],
        KeyMetal::Bronze => [0.62, 0.42, 0.24],
    };
    let tarnish = look.tarnish as f32 * 0.08;
    let dull = 1.0 - tarnish;
    StandardMaterial {
        base_color: Color::srgb(red * dull, green * dull + tarnish * 0.12, blue * dull),
        metallic: 0.8 - tarnish,
        perceptual_roughness: 0.42 + tarnish,
        ..default()
    }
}
