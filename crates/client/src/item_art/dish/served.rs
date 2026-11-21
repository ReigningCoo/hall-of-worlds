//! Food served in or on crockery: bowls, plates, sauces, puddings, jars and
//! pastes.

use bevy::prelude::*;

use super::kit::{Kit, round};

/// A bowl of soup or stew (2 × 2): a deep white bowl, the food's surface
/// near its rim and pieces of the second colour in it.
pub(super) fn bowl(kit: &mut Kit) {
    let crockery = kit.crockery.clone();
    kit.round_dish(&crockery, 0.5, 0.98, 0.5);
    let (food, accent) = (kit.food.clone(), kit.accent.clone());
    kit.add(
        Cylinder::new(0.9, 0.02),
        &food,
        Transform::from_xyz(0.0, 0.505, 0.0),
    );
    for (at, size) in [
        (0.05, 0.16),
        (0.3, 0.13),
        (0.52, 0.17),
        (0.74, 0.12),
        (0.9, 0.14),
    ] {
        kit.add(
            Sphere::new(size),
            &accent,
            Transform::from_translation(round(at, 0.5, 0.52)).with_scale(Vec3::new(1.0, 0.55, 1.0)),
        );
    }
    kit.add(
        Sphere::new(0.15),
        &accent,
        Transform::from_xyz(0.0, 0.52, 0.0).with_scale(Vec3::new(1.0, 0.5, 1.0)),
    );
}

/// A plate (2 × 2): a wide white plate, the food heaped in the middle and
/// pieces of the second colour beside it.
pub(super) fn plate(kit: &mut Kit) {
    let crockery = kit.crockery.clone();
    kit.round_dish(&crockery, 0.7, 1.0, 0.08);
    let (food, accent, crust) = (kit.food.clone(), kit.accent.clone(), kit.crust.clone());
    kit.add(
        Sphere::new(0.5),
        &food,
        Transform::from_xyz(-0.15, 0.1, -0.1).with_scale(Vec3::new(1.0, 0.42, 0.85)),
    );
    // Pieces lying beside the heap.
    for (x, z, turn) in [(0.5, 0.3, 0.4), (0.35, 0.55, -0.3), (0.6, -0.15, 1.2)] {
        kit.add(
            Capsule3d::new(0.1, 0.26),
            &accent,
            Transform::from_xyz(x, 0.16, z)
                .with_rotation(Quat::from_rotation_y(turn) * Quat::from_rotation_z(1.5)),
        );
    }
    // Browned bits on top of the heap.
    for (x, z) in [(-0.3, -0.2), (0.0, 0.05), (-0.1, -0.35)] {
        kit.add(Sphere::new(0.08), &crust, Transform::from_xyz(x, 0.3, z));
    }
}

/// A sauce (1 × 1): a small white sauce boat of it with a drip down the
/// side and a swirl of the second colour.
pub(super) fn sauce(kit: &mut Kit) {
    let crockery = kit.crockery.clone();
    kit.round_dish(&crockery, 0.45, 0.78, 0.42);
    // The lip it is poured from.
    kit.add(
        Cone::new(0.22, 0.4),
        &crockery,
        Transform::from_xyz(0.78, 0.38, 0.0).with_rotation(Quat::from_rotation_z(-1.3)),
    );
    let (food, accent) = (kit.food.clone(), kit.accent.clone());
    kit.add(
        Cylinder::new(0.72, 0.02),
        &food,
        Transform::from_xyz(0.0, 0.425, 0.0),
    );
    kit.add(
        Torus::new(0.22, 0.3),
        &accent,
        Transform::from_xyz(-0.05, 0.43, 0.05).with_scale(Vec3::new(1.0, 0.3, 1.0)),
    );
    // A drip down the front.
    kit.add(
        Capsule3d::new(0.05, 0.2),
        &food,
        Transform::from_xyz(0.1, 0.28, 0.68).with_rotation(Quat::from_rotation_x(0.45)),
    );
}

/// A pudding (1 × 1): a dome turned out on a small plate, a cap of the
/// second colour running down it.
pub(super) fn pudding(kit: &mut Kit) {
    let crockery = kit.crockery.clone();
    kit.round_dish(&crockery, 0.7, 0.95, 0.06);
    let (food, accent, pale) = (kit.food.clone(), kit.accent.clone(), kit.pale.clone());
    kit.add(
        Sphere::new(0.62),
        &food,
        Transform::from_xyz(0.0, 0.08, 0.0).with_scale(Vec3::new(1.0, 0.85, 1.0)),
    );
    kit.add(
        Sphere::new(0.48),
        &accent,
        Transform::from_xyz(0.0, 0.42, 0.0).with_scale(Vec3::new(1.0, 0.6, 1.0)),
    );
    kit.add(Sphere::new(0.1), &pale, Transform::from_xyz(0.0, 0.72, 0.0));
}

/// A jar (1 × 1): an open earthenware crock filled to the neck, a linen
/// cloth tied round it, flecks of the second colour on top.
pub(super) fn jar(kit: &mut Kit) {
    let (earth, linen) = (kit.earthenware.clone(), kit.linen.clone());
    kit.add(
        Cylinder::new(0.5, 0.75),
        &earth,
        Transform::from_xyz(0.0, 0.375, 0.0),
    );
    kit.add(
        Sphere::new(0.5),
        &earth,
        Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(1.0, 0.3, 1.0)),
    );
    kit.add(
        Torus::new(0.4, 0.5),
        &earth,
        Transform::from_xyz(0.0, 0.8, 0.0),
    );
    // The cloth folded back round the neck, and its string.
    kit.add(
        Torus::new(0.46, 0.56),
        &linen,
        Transform::from_xyz(0.0, 0.66, 0.0).with_scale(Vec3::new(1.0, 1.6, 1.0)),
    );
    let (food, accent) = (kit.food.clone(), kit.accent.clone());
    kit.add(
        Cylinder::new(0.42, 0.02),
        &food,
        Transform::from_xyz(0.0, 0.78, 0.0),
    );
    for at in [0.1, 0.45, 0.8] {
        kit.add(
            Sphere::new(0.07),
            &accent,
            Transform::from_translation(round(at, 0.2, 0.8)),
        );
    }
}

/// A paste (1 × 1): a heap of it in a shallow earthenware dish, flecked
/// with the second colour.
pub(super) fn paste(kit: &mut Kit) {
    let earth = kit.earthenware.clone();
    kit.round_dish(&earth, 0.5, 0.82, 0.22);
    let (food, accent) = (kit.food.clone(), kit.accent.clone());
    kit.add(
        Sphere::new(0.62),
        &food,
        Transform::from_xyz(0.0, 0.14, 0.0).with_scale(Vec3::new(1.0, 0.42, 1.0)),
    );
    for (x, z) in [(0.2, 0.1), (-0.22, 0.05), (0.02, -0.22), (-0.05, 0.25)] {
        kit.add(Sphere::new(0.07), &accent, Transform::from_xyz(x, 0.38, z));
    }
}
