//! The two-by-two vessels: an earthenware jug with a handle and a sewn
//! leather waterskin with a tied neck.

use bevy::prelude::*;

use super::{CLAY, CORK, Kit, LEATHER, handle, mouth};

/// A jug: a round belly, a narrower neck with a rim, and a handle.
pub(super) fn jug(kit: &mut Kit) {
    let clay = kit.matte(CLAY, 0.85);
    let belly = Transform::from_xyz(-0.1, -0.3, 0.0).with_scale(Vec3::new(1.0, 0.9, 1.0));
    kit.add(Sphere::new(0.78), &clay, belly);
    let neck = ConicalFrustum {
        radius_top: 0.4,
        radius_bottom: 0.5,
        height: 0.6,
    };
    kit.add(neck, &clay, Transform::from_xyz(-0.1, 0.55, 0.0));
    kit.add(
        Torus::new(0.36, 0.46),
        &clay,
        Transform::from_xyz(-0.1, 0.85, 0.0),
    );
    mouth(kit, 0.37, 0.86);
    handle(kit, &clay, 0.55, 0.3, 0.42);
}

/// A waterskin: a soft leather bag, flatter front to back, its neck tied
/// with a cord and stoppered.
pub(super) fn waterskin(kit: &mut Kit) {
    let leather = kit.matte(LEATHER, 0.75);
    let cord = kit.matte([0.78, 0.7, 0.52], 0.9);
    let cork = kit.matte(CORK, 0.9);
    let bag = Transform::from_xyz(0.0, -0.25, 0.0).with_scale(Vec3::new(0.95, 0.72, 0.45));
    kit.add(Sphere::new(1.0), &leather, bag);
    let neck = ConicalFrustum {
        radius_top: 0.16,
        radius_bottom: 0.32,
        height: 0.55,
    };
    kit.add(neck, &leather, Transform::from_xyz(0.0, 0.62, 0.0));
    kit.add(
        Torus::new(0.17, 0.25),
        &cord,
        Transform::from_xyz(0.0, 0.7, 0.0),
    );
    kit.add(
        Cylinder::new(0.15, 0.25),
        &cork,
        Transform::from_xyz(0.0, 0.95, 0.0),
    );
}
