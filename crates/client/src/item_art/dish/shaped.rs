//! Food on small plates and in small bowls: noodles, dumplings and egg
//! dishes.

use bevy::prelude::*;

use super::kit::{Kit, round};

/// Noodles (1 × 1): a nest of them in a white bowl, pieces of the second
/// colour on top, chopsticks across the rim.
pub(super) fn noodles(kit: &mut Kit) {
    let crockery = kit.crockery.clone();
    kit.round_dish(&crockery, 0.45, 0.85, 0.38);
    let food = kit.food.clone();
    kit.add(
        Cylinder::new(0.78, 0.02),
        &food,
        Transform::from_xyz(0.0, 0.385, 0.0),
    );
    nest(kit, 0.42);
    let (accent, wood) = (kit.accent.clone(), kit.wood.clone());
    for (x, z) in [(0.2, -0.15), (-0.15, 0.2)] {
        kit.add(
            Sphere::new(0.1),
            &accent,
            Transform::from_xyz(x, 0.48, z).with_scale(Vec3::new(1.2, 0.6, 1.0)),
        );
    }
    for z in [-0.42, -0.3] {
        kit.add(
            Cylinder::new(0.025, 1.4),
            &wood,
            Transform::from_xyz(0.0, 0.46, z)
                .with_rotation(Quat::from_rotation_y(0.25) * Quat::from_rotation_z(1.5)),
        );
    }
}

/// A nest of noodles: thin rings of the food's colour, tipped every way,
/// heaped at `y`.
pub(super) fn nest(kit: &mut Kit, y: f32) {
    let food = kit.food.clone();
    for i in 0..14 {
        let share = i as f32 / 14.0;
        let size = 0.38 + 0.2 * ((i * 5) % 9) as f32 / 9.0;
        kit.add(
            Torus::new(size - 0.04, size),
            &food,
            Transform::from_translation(round(share, 0.18, y + 0.02 * (i % 3) as f32))
                .with_rotation(
                    Quat::from_rotation_y(share * 5.0)
                        * Quat::from_rotation_x(0.25 + 0.1 * (i % 3) as f32),
                ),
        );
    }
}

/// Dumplings (1 × 1): three pleated dumplings on a small white plate, a dip
/// of the second colour.
pub(super) fn dumplings(kit: &mut Kit) {
    let crockery = kit.crockery.clone();
    kit.round_dish(&crockery, 0.7, 0.95, 0.06);
    let (food, accent) = (kit.food.clone(), kit.accent.clone());
    for (x, z, turn) in [(-0.3, -0.25, 0.3), (0.32, -0.2, -0.4), (0.0, 0.32, 0.1)] {
        let at = Transform::from_xyz(x, 0.08, z).with_rotation(Quat::from_rotation_y(turn));
        kit.add(
            Sphere::new(1.0),
            &food,
            at * Transform::from_scale(Vec3::new(0.36, 0.22, 0.24)),
        );
        // The pleats along its top.
        for step in -2..=2 {
            kit.add(
                Sphere::new(0.06),
                &food,
                at * Transform::from_xyz(step as f32 * 0.1, 0.2, 0.0),
            );
        }
    }
    kit.add(
        Sphere::new(0.16),
        &accent,
        Transform::from_xyz(0.45, 0.07, 0.45).with_scale(Vec3::new(1.0, 0.25, 1.0)),
    );
}

/// An egg dish (1 × 1): a folded omelette on a small white plate, browned
/// at its edge, flecked with the second colour.
pub(super) fn egg_dish(kit: &mut Kit) {
    let crockery = kit.crockery.clone();
    kit.round_dish(&crockery, 0.7, 0.95, 0.06);
    let (food, accent, crust) = (kit.food.clone(), kit.accent.clone(), kit.crust.clone());
    kit.add(
        Sphere::new(1.0),
        &crust,
        Transform::from_xyz(0.0, 0.08, 0.0).with_scale(Vec3::new(0.72, 0.14, 0.46)),
    );
    kit.add(
        Sphere::new(1.0),
        &food,
        Transform::from_xyz(0.0, 0.12, -0.02).with_scale(Vec3::new(0.66, 0.14, 0.4)),
    );
    for (x, z) in [(-0.3, 0.05), (0.0, -0.15), (0.25, 0.1), (0.1, 0.18)] {
        kit.add(Sphere::new(0.06), &accent, Transform::from_xyz(x, 0.25, z));
    }
}
