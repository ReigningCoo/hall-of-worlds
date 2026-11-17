//! The Keyring (owner, 2026-11-17): an iron ring with three keys hanging from it,
//! fanned out to the right, lying flat along the picture's width (x), two cells
//! wide and one long. It looks the same however many keys it holds. Its keys come
//! from the keys' own builder (`key/`), rolled from fixed seeds (owner), so they
//! look like loose keys.

use bevy::prelude::*;
use shared::rng::Seed;

use super::Part;
use super::key::{self, KeyLook};

/// Where the ring is, and how big.
const RING_AT: Vec3 = Vec3::new(-0.7, 0.0, 0.0);
const RING: f32 = 0.38;
/// How far each key reaches from the ring's edge to its tip.
pub(super) const KEY_LONG: f32 = 1.55;
/// Each key's seed and how far it is turned from straight right.
const KEYS: [(u64, f32); 3] = [(23, -0.24), (49, 0.0), (45, 0.24)];

/// The Keyring's parts (`Look::Keyring`), against the picture's `VIEW_WIDTH` of 2.5.
pub(super) fn keyring_parts(
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Vec<Part> {
    let iron = materials.add(StandardMaterial {
        base_color: Color::srgb(0.42, 0.42, 0.44),
        metallic: 0.8,
        perceptual_roughness: 0.45,
        ..default()
    });
    // The ring itself, lying flat (a torus lies in the x-z plane).
    let mut parts = vec![Part {
        mesh: meshes.add(Torus::new(RING - 0.08, RING)),
        material: iron,
        transform: Transform::from_translation(RING_AT),
    }];
    for (look, at) in keys() {
        parts.extend(key::hung(&look, at, KEY_LONG, meshes, materials));
    }
    parts
}

/// Turned over, bit inwards, for a key fanned towards +z (the picture's bottom),
/// so its bit stays inside the picture.
fn flip(turn: f32) -> f32 {
    if turn < 0.0 {
        std::f32::consts::PI
    } else {
        0.0
    }
}

/// The three keys and where each hangs: its bow's middle on the ring's edge,
/// each a little higher than the last so they lie over one another.
pub(super) fn keys() -> Vec<(KeyLook, Transform)> {
    let hang = |index: usize, turn: f32| {
        let lift = Vec3::Y * (0.05 + 0.04 * index as f32);
        Transform::from_translation(RING_AT + lift).with_rotation(Quat::from_rotation_y(turn))
            * Transform::from_xyz(RING - 0.04, 0.0, 0.0)
            * Transform::from_rotation(Quat::from_rotation_x(flip(turn)))
    };
    let key = |(index, (seed, turn)): (usize, (u64, f32))| {
        (KeyLook::roll(Seed::new(seed)), hang(index, turn))
    };
    KEYS.into_iter().enumerate().map(key).collect()
}
