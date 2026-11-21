//! The long processes' intermediates (cooking plan PR 12), one cell each:
//! cooked soybeans, soybean koji, moromi, miso and cheese.

use bevy::prelude::*;
use shared::food::Rest;

use super::kit::{Kit, round};

/// A long process's pieces; `false` for any other kind.
pub(super) fn long(kit: &mut Kit, kind: Rest) -> bool {
    match kind {
        Rest::CookedSoybeans => beans(kit, false),
        Rest::Koji => beans(kit, true),
        Rest::Moromi => tub(kit),
        Rest::Miso => crock(kit),
        Rest::Cheese => wheel(kit),
        _ => return false,
    }
    true
}

/// Soybeans heaped in an earthenware bowl; as koji, in a shallow wooden
/// tray instead, furred pale with the mould.
fn beans(kit: &mut Kit, koji: bool) {
    let food = kit.food.clone();
    let base = if koji {
        let wood = kit.wood.clone();
        kit.add(
            Cuboid::new(1.5, 0.12, 1.5),
            &wood,
            Transform::from_xyz(0.0, 0.06, 0.0),
        );
        0.14
    } else {
        let earth = kit.earthenware.clone();
        kit.round_dish(&earth, 0.45, 0.8, 0.4);
        0.3
    };
    for ring in 0..3 {
        let radius = 0.55 - 0.2 * ring as f32;
        for i in 0..(9 - 3 * ring) {
            let at = round(
                i as f32 / (9 - 3 * ring) as f32,
                radius,
                base + 0.1 * ring as f32,
            );
            kit.add(Sphere::new(0.12), &food, Transform::from_translation(at));
        }
    }
    if koji {
        let pale = kit.pale.clone();
        for i in 0..8 {
            let at = round(
                i as f32 / 8.0 + 0.06,
                0.35 + 0.1 * (i % 2) as f32,
                base + 0.2,
            );
            kit.add(
                Sphere::new(0.1),
                &pale,
                Transform::from_translation(at).with_scale(Vec3::new(1.0, 0.4, 1.0)),
            );
        }
    }
}

/// Moromi: a small wooden tub of the dark mash, hooped twice.
fn tub(kit: &mut Kit) {
    let (wood, accent, food) = (kit.wood.clone(), kit.accent.clone(), kit.food.clone());
    kit.round_dish(&wood, 0.6, 0.7, 0.85);
    for y in [0.2, 0.65] {
        kit.add(
            Torus::new(0.6 + 0.08 * y, 0.68 + 0.08 * y),
            &accent,
            Transform::from_xyz(0.0, y, 0.0),
        );
    }
    kit.add(
        Cylinder::new(0.64, 0.02),
        &food,
        Transform::from_xyz(0.0, 0.8, 0.0),
    );
}

/// Miso: an earthenware crock packed with the paste, a wooden paddle in it.
fn crock(kit: &mut Kit) {
    let (earth, food, wood) = (kit.earthenware.clone(), kit.food.clone(), kit.wood.clone());
    kit.round_dish(&earth, 0.55, 0.72, 0.7);
    kit.add(
        Sphere::new(0.66),
        &food,
        Transform::from_xyz(0.0, 0.66, 0.0).with_scale(Vec3::new(1.0, 0.22, 1.0)),
    );
    kit.add(
        Cuboid::new(0.22, 0.03, 0.9),
        &wood,
        Transform::from_xyz(0.2, 0.9, -0.1)
            .with_rotation(Quat::from_rotation_y(0.6) * Quat::from_rotation_x(0.9)),
    );
}

/// Cheese: a tall wheel of it with a darker rind, a wedge's worth of its
/// pale inside showing on top.
fn wheel(kit: &mut Kit) {
    let (food, accent, pale, wood) = (
        kit.food.clone(),
        kit.accent.clone(),
        kit.pale.clone(),
        kit.wood.clone(),
    );
    kit.add(
        Cylinder::new(0.85, 0.06),
        &wood,
        Transform::from_xyz(0.0, 0.03, 0.0),
    );
    kit.add(
        Cylinder::new(0.68, 0.55),
        &food,
        Transform::from_xyz(0.0, 0.34, 0.0),
    );
    for y in [0.08, 0.6] {
        kit.add(
            Torus::new(0.6, 0.72),
            &accent,
            Transform::from_xyz(0.0, y, 0.0).with_scale(Vec3::new(1.0, 0.5, 1.0)),
        );
    }
    kit.add(
        Cylinder::new(0.5, 0.02),
        &pale,
        Transform::from_xyz(0.0, 0.62, 0.0),
    );
}
