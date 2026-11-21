//! Baked and griddled food: loaves, flatbreads, pies and cakes.

use bevy::prelude::*;

use super::kit::{Kit, round};

/// A loaf (2 × 1): a long, domed loaf, browned, with pale slashes across.
pub(super) fn loaf(kit: &mut Kit) {
    let (crust, pale) = (kit.crust.clone(), kit.pale.clone());
    kit.add(
        Sphere::new(1.0),
        &crust,
        Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(0.98, 0.32, 0.42)),
    );
    for x in [-0.45, 0.0, 0.45] {
        kit.add(
            Capsule3d::new(0.035, 0.42),
            &pale,
            Transform::from_xyz(x, 0.3, 0.0).with_rotation(
                Quat::from_rotation_y(0.6) * Quat::from_rotation_x(std::f32::consts::FRAC_PI_2),
            ),
        );
    }
}

/// Flatbreads (2 × 1): two thin rounds overlapping, griddled with spots of
/// crust and a smear of the second colour.
pub(super) fn flatbread(kit: &mut Kit) {
    let (food, crust, accent) = (kit.food.clone(), kit.crust.clone(), kit.accent.clone());
    for (x, y) in [(-0.4, 0.0), (0.4, 0.06)] {
        kit.add(
            Cylinder::new(0.52, 0.05),
            &food,
            Transform::from_xyz(x, y, 0.0).with_scale(Vec3::new(1.15, 1.0, 0.85)),
        );
    }
    for (x, z) in [
        (0.25, 0.1),
        (0.55, -0.15),
        (0.45, 0.25),
        (0.7, 0.05),
        (0.2, -0.2),
    ] {
        kit.add(
            Cylinder::new(0.06, 0.01),
            &crust,
            Transform::from_xyz(x, 0.09, z),
        );
    }
    kit.add(
        Sphere::new(0.22),
        &accent,
        Transform::from_xyz(-0.5, 0.04, 0.0).with_scale(Vec3::new(1.3, 0.2, 1.0)),
    );
}

/// A pie (2 × 2): a deep earthenware dish, a domed browned lid with a
/// crimped edge, slits showing the filling's colour.
pub(super) fn pie(kit: &mut Kit) {
    let earth = kit.earthenware.clone();
    kit.round_dish(&earth, 0.8, 0.98, 0.3);
    let (crust, accent) = (kit.crust.clone(), kit.accent.clone());
    kit.add(
        Sphere::new(0.88),
        &crust,
        Transform::from_xyz(0.0, 0.3, 0.0).with_scale(Vec3::new(1.0, 0.25, 1.0)),
    );
    // The crimped edge, a ring of little lumps.
    for i in 0..18 {
        kit.add(
            Sphere::new(0.1),
            &crust,
            Transform::from_translation(round(i as f32 / 18.0, 0.86, 0.33))
                .with_scale(Vec3::new(1.0, 0.7, 1.0)),
        );
    }
    // Slits through the lid, the filling showing.
    for turn in [0.0, 2.1, 4.2] {
        let along = Quat::from_rotation_y(turn);
        kit.add(
            Capsule3d::new(0.04, 0.22),
            &accent,
            Transform::from_translation(along * Vec3::new(0.32, 0.53, 0.0))
                .with_rotation(along * Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
        );
    }
}

/// A cake (2 × 2): a round cake on a white plate, its top and a ring of
/// piping in the second colour, a pale topping in the middle.
pub(super) fn cake(kit: &mut Kit) {
    let crockery = kit.crockery.clone();
    kit.round_dish(&crockery, 0.8, 1.0, 0.05);
    let (food, accent, pale) = (kit.food.clone(), kit.accent.clone(), kit.pale.clone());
    kit.add(
        Cylinder::new(0.8, 0.42),
        &food,
        Transform::from_xyz(0.0, 0.26, 0.0),
    );
    kit.add(
        Cylinder::new(0.8, 0.05),
        &accent,
        Transform::from_xyz(0.0, 0.48, 0.0),
    );
    for i in 0..12 {
        kit.add(
            Sphere::new(0.09),
            &accent,
            Transform::from_translation(round(i as f32 / 12.0, 0.68, 0.52)),
        );
    }
    kit.add(
        Sphere::new(0.17),
        &pale,
        Transform::from_xyz(0.0, 0.56, 0.0).with_scale(Vec3::new(1.0, 0.7, 1.0)),
    );
}
