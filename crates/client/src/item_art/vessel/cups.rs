//! The one-cell drinking vessels: a lidded pewter tankard, an earthenware cup
//! and a stemmed pewter goblet.

use bevy::prelude::*;

use super::{CLAY, Kit, PEWTER, handle, mouth};

/// A pewter tankard, a little narrower at the top, with a hinged lid and a
/// handle on the right.
pub(super) fn tankard(kit: &mut Kit) {
    let pewter = kit.metal(PEWTER);
    let body = ConicalFrustum {
        radius_top: 0.5,
        radius_bottom: 0.58,
        height: 1.3,
    };
    kit.add(body, &pewter, Transform::from_xyz(-0.15, -0.1, 0.0));
    // A foot ring and the lid.
    kit.add(
        Torus::new(0.52, 0.62),
        &pewter,
        Transform::from_xyz(-0.15, -0.74, 0.0),
    );
    kit.add(
        Cylinder::new(0.54, 0.1),
        &pewter,
        Transform::from_xyz(-0.15, 0.6, 0.0),
    );
    kit.add(
        Sphere::new(0.1),
        &pewter,
        Transform::from_xyz(0.32, 0.7, 0.0),
    );
    handle(kit, &pewter, 0.45, 0.0, 0.42);
}

/// An earthenware cup, wider at the top, with a dark inside.
pub(super) fn cup(kit: &mut Kit) {
    let clay = kit.matte(CLAY, 0.85);
    let body = ConicalFrustum {
        radius_top: 0.68,
        radius_bottom: 0.48,
        height: 1.0,
    };
    kit.add(body, &clay, Transform::from_xyz(0.0, -0.2, 0.0));
    kit.add(
        Torus::new(0.62, 0.72),
        &clay,
        Transform::from_xyz(0.0, 0.3, 0.0),
    );
    mouth(kit, 0.63, 0.31);
}

/// A pewter goblet: a bowl on a stem and a round foot.
pub(super) fn goblet(kit: &mut Kit) {
    let pewter = kit.metal(PEWTER);
    let bowl = ConicalFrustum {
        radius_top: 0.6,
        radius_bottom: 0.22,
        height: 0.75,
    };
    kit.add(bowl, &pewter, Transform::from_xyz(0.0, 0.45, 0.0));
    kit.add(
        Sphere::new(0.24),
        &pewter,
        Transform::from_xyz(0.0, 0.08, 0.0),
    );
    kit.add(
        Cylinder::new(0.08, 0.7),
        &pewter,
        Transform::from_xyz(0.0, -0.4, 0.0),
    );
    kit.add(
        Cylinder::new(0.45, 0.08),
        &pewter,
        Transform::from_xyz(0.0, -0.78, 0.0),
    );
    mouth(kit, 0.57, 0.83);
}
