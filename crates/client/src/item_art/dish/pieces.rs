//! Food served without crockery: balls, blocks, pats, sausages, strips and
//! anything else.

use bevy::prelude::*;

use super::kit::Kit;

/// Balls (1 × 1): three of them heaped on a wooden board, browned spots
/// and a sprinkle of the second colour.
pub(super) fn balls(kit: &mut Kit) {
    board(kit);
    let (food, accent, crust) = (kit.food.clone(), kit.accent.clone(), kit.crust.clone());
    for (x, y, z) in [(-0.33, 0.33, 0.2), (0.33, 0.33, 0.2), (0.0, 0.38, -0.3)] {
        kit.add(Sphere::new(0.32), &food, Transform::from_xyz(x, y, z));
        kit.add(
            Sphere::new(0.09),
            &crust,
            Transform::from_xyz(x + 0.1, y + 0.25, z - 0.08),
        );
        kit.add(
            Sphere::new(0.05),
            &accent,
            Transform::from_xyz(x - 0.12, y + 0.26, z + 0.05),
        );
    }
}

/// A block (1 × 1): a block of it on a wooden board with a slice cut off
/// and leaning, its cut face paler.
pub(super) fn block(kit: &mut Kit) {
    board(kit);
    let (food, pale, accent) = (kit.food.clone(), kit.pale.clone(), kit.accent.clone());
    kit.add(
        Cuboid::new(0.9, 0.5, 0.9),
        &food,
        Transform::from_xyz(-0.15, 0.37, 0.0),
    );
    kit.add(
        Cuboid::new(0.02, 0.48, 0.86),
        &pale,
        Transform::from_xyz(0.31, 0.37, 0.0),
    );
    kit.add(
        Cuboid::new(0.14, 0.5, 0.88),
        &food,
        Transform::from_xyz(0.55, 0.33, 0.0).with_rotation(Quat::from_rotation_z(-0.5)),
    );
    for (x, z) in [(-0.4, -0.2), (0.0, 0.25), (-0.25, 0.3)] {
        kit.add(Sphere::new(0.05), &accent, Transform::from_xyz(x, 0.62, z));
    }
}

/// A pat (1 × 1): a rounded pat of it on a square of linen, scored with a
/// knife, a leaf of the second colour beside it.
pub(super) fn pat(kit: &mut Kit) {
    let (linen, food, crust, accent) = (
        kit.linen.clone(),
        kit.food.clone(),
        kit.pale.clone(),
        kit.accent.clone(),
    );
    kit.add(
        Cuboid::new(1.4, 0.03, 1.4),
        &linen,
        Transform::from_xyz(0.0, 0.015, 0.0).with_rotation(Quat::from_rotation_y(0.3)),
    );
    kit.add(
        Capsule3d::new(0.22, 0.35),
        &food,
        Transform::from_xyz(0.0, 0.2, 0.0)
            .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2))
            .with_scale(Vec3::new(0.85, 1.0, 1.4)),
    );
    for x in [-0.12, 0.0, 0.12] {
        kit.add(
            Cuboid::new(0.02, 0.02, 0.4),
            &crust,
            Transform::from_xyz(x, 0.39, 0.0),
        );
    }
    kit.add(
        Sphere::new(0.2),
        &accent,
        Transform::from_xyz(0.45, 0.05, 0.42).with_scale(Vec3::new(1.4, 0.15, 0.7)),
    );
}

/// Sausages (2 × 1): two lying side by side, grill marks across them.
pub(super) fn sausages(kit: &mut Kit) {
    let (food, crust) = (kit.food.clone(), kit.crust.clone());
    for (z, turn) in [(-0.2, 0.06), (0.2, -0.05)] {
        let at = Transform::from_xyz(0.0, 0.18, z).with_rotation(
            Quat::from_rotation_y(turn) * Quat::from_rotation_z(std::f32::consts::FRAC_PI_2),
        );
        kit.add(Capsule3d::new(0.18, 1.5), &food, at);
        for step in -2..=2 {
            kit.add(
                Cylinder::new(0.185, 0.04),
                &crust,
                at * Transform::from_xyz(0.0, step as f32 * 0.32, 0.0)
                    .with_rotation(Quat::from_rotation_x(0.4)),
            );
        }
    }
}

/// Strips (2 × 1): four wavy strips lying across, each with a band of the
/// second colour along it.
pub(super) fn strips(kit: &mut Kit) {
    let (food, accent) = (kit.food.clone(), kit.accent.clone());
    for (row, z) in [-0.33f32, -0.11, 0.11, 0.33].into_iter().enumerate() {
        for wave in 0..4 {
            let x = -0.75 + wave as f32 * 0.5;
            let tilt = if (wave + row) % 2 == 0 { 0.18 } else { -0.18 };
            let at = Transform::from_xyz(x, 0.1 + 0.04 * (row % 2) as f32, z)
                .with_rotation(Quat::from_rotation_z(tilt));
            kit.add(Cuboid::new(0.52, 0.04, 0.12), &food, at);
            kit.add(
                Cuboid::new(0.52, 0.045, 0.04),
                &accent,
                at * Transform::from_xyz(0.0, 0.0, 0.04),
            );
        }
    }
}

/// Anything else (1 × 1): three golden lumps on a cloth, browned, with a
/// smear of the second colour.
pub(super) fn other(kit: &mut Kit) {
    let (linen, food, crust, accent) = (
        kit.linen.clone(),
        kit.food.clone(),
        kit.crust.clone(),
        kit.accent.clone(),
    );
    kit.add(
        Cylinder::new(0.95, 0.03),
        &linen,
        Transform::from_xyz(0.0, 0.015, 0.0),
    );
    for (x, z, size) in [(-0.3, 0.2, 0.34), (0.32, 0.15, 0.3), (0.0, -0.3, 0.32)] {
        kit.add(
            Sphere::new(size),
            &food,
            Transform::from_xyz(x, size * 0.6, z).with_scale(Vec3::new(1.15, 0.8, 1.0)),
        );
        kit.add(
            Sphere::new(size * 0.5),
            &crust,
            Transform::from_xyz(x + size * 0.3, size * 1.05, z - size * 0.2)
                .with_scale(Vec3::new(1.0, 0.45, 1.0)),
        );
    }
    kit.add(
        Sphere::new(0.15),
        &accent,
        Transform::from_xyz(0.45, 0.04, -0.4).with_scale(Vec3::new(1.0, 0.25, 1.0)),
    );
}

/// A wooden board, for food served without crockery.
pub(super) fn board(kit: &mut Kit) {
    let wood = kit.wood.clone();
    kit.add(
        Cuboid::new(1.6, 0.12, 1.5),
        &wood,
        Transform::from_xyz(0.0, 0.06, 0.0),
    );
}
