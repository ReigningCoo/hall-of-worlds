//! The stoppered vessels: a tall, dark green wine bottle with a cork (two
//! cells, unlike a potion's small vial) and a flat, leather-covered flask.

use bevy::prelude::*;

use super::{CORK, Kit, LEATHER, PEWTER};

/// Dark green bottle glass.
const BOTTLE_GLASS: [f32; 3] = [0.08, 0.22, 0.12];

/// A wine bottle: a tall body, round shoulders, a long neck and its cork.
pub(super) fn wine_bottle(kit: &mut Kit) {
    let glass = kit.materials.add(StandardMaterial {
        base_color: Color::srgb(BOTTLE_GLASS[0], BOTTLE_GLASS[1], BOTTLE_GLASS[2]),
        perceptual_roughness: 0.15,
        reflectance: 0.6,
        ..default()
    });
    let cork = kit.matte(CORK, 0.9);
    kit.add(
        Cylinder::new(0.5, 2.3),
        &glass,
        Transform::from_xyz(0.0, -0.95, 0.0),
    );
    let shoulders = Transform::from_xyz(0.0, 0.2, 0.0).with_scale(Vec3::new(1.0, 0.6, 1.0));
    kit.add(Sphere::new(0.5), &glass, shoulders);
    kit.add(
        Cylinder::new(0.17, 1.2),
        &glass,
        Transform::from_xyz(0.0, 0.95, 0.0),
    );
    kit.add(
        Torus::new(0.15, 0.23),
        &glass,
        Transform::from_xyz(0.0, 1.5, 0.0),
    );
    kit.add(
        Cylinder::new(0.15, 0.35),
        &cork,
        Transform::from_xyz(0.0, 1.65, 0.0),
    );
}

/// A flask: flat front to back, leather over its lower part, a pewter neck
/// and stopper.
pub(super) fn flask(kit: &mut Kit) {
    let leather = kit.matte(LEATHER, 0.8);
    let pewter = kit.metal(PEWTER);
    let flat = Vec3::new(0.62, 0.7, 0.3);
    kit.add(
        Sphere::new(1.0),
        &pewter,
        Transform::from_xyz(0.0, -0.1, 0.0).with_scale(flat),
    );
    let cover = Transform::from_xyz(0.0, -0.22, 0.0).with_scale(flat * Vec3::new(1.03, 0.82, 1.06));
    kit.add(Sphere::new(1.0), &leather, cover);
    kit.add(
        Cylinder::new(0.14, 0.35),
        &pewter,
        Transform::from_xyz(0.0, 0.7, 0.0),
    );
    kit.add(
        Cylinder::new(0.18, 0.16),
        &pewter,
        Transform::from_xyz(0.0, 0.92, 0.0),
    );
}
