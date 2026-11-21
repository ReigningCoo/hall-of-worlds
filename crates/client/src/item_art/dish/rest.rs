//! Intermediates, one cell each: dough, noodles fresh and dried, roux,
//! stock, butter, fresh cheese, warm milk and yoghurt (the long processes'
//! in `long.rs`).

use bevy::prelude::*;
use shared::food::Rest;

use super::kit::{Kit, round};
use super::pieces::{board, pat};
use super::shaped::nest;

/// An intermediate's pieces.
pub(super) fn rest(kit: &mut Kit, kind: Rest) {
    if super::long::long(kit, kind) {
        return;
    }
    match kind {
        Rest::Dough => dough(kit),
        Rest::FreshNoodles => {
            board(kit);
            nest(kit, 0.18);
            dust(kit, 0.14);
        }
        Rest::DriedNoodles => dried_noodles(kit),
        Rest::Roux => pot_of(kit, 0.3, false),
        Rest::Stock => pot_of(kit, 0.62, true),
        Rest::Butter => pat(kit),
        Rest::FreshCheese => fresh_cheese(kit),
        Rest::WarmMilk => cup(kit),
        Rest::Yoghurt => yoghurt(kit),
        Rest::CookedSoybeans | Rest::Koji | Rest::Moromi | Rest::Miso | Rest::Cheese => {}
    }
}

/// Flour dusted round at `y`.
fn dust(kit: &mut Kit, y: f32) {
    let pale = kit.pale.clone();
    for i in 0..7 {
        let share = i as f32 / 7.0;
        kit.add(
            Cylinder::new(0.05 + 0.02 * (i % 3) as f32, 0.01),
            &pale,
            Transform::from_translation(round(share + 0.05, 0.62, y)),
        );
    }
}

/// Dough: a risen ball of it in an earthenware bowl, floured on top.
fn dough(kit: &mut Kit) {
    let earth = kit.earthenware.clone();
    kit.round_dish(&earth, 0.45, 0.85, 0.42);
    let (food, pale) = (kit.food.clone(), kit.pale.clone());
    kit.add(
        Sphere::new(0.72),
        &food,
        Transform::from_xyz(0.0, 0.38, 0.0).with_scale(Vec3::new(1.0, 0.55, 1.0)),
    );
    for (x, z) in [(0.1, -0.1), (-0.2, 0.15), (0.25, 0.2)] {
        kit.add(
            Sphere::new(0.12),
            &pale,
            Transform::from_xyz(x, 0.76, z).with_scale(Vec3::new(1.0, 0.2, 1.0)),
        );
    }
}

/// Dried noodles: a bundle of straight sticks lying across, tied twice.
fn dried_noodles(kit: &mut Kit) {
    let (food, linen) = (kit.food.clone(), kit.linen.clone());
    let across = Quat::from_rotation_y(0.5) * Quat::from_rotation_z(std::f32::consts::FRAC_PI_2);
    let bundle = Transform::from_xyz(0.0, 0.2, 0.0).with_rotation(across);
    for i in 0..12 {
        let at = round(i as f32 / 12.0, 0.08 + 0.07 * (i % 2) as f32, 0.0);
        kit.add(
            Cylinder::new(0.035, 1.7),
            &food,
            bundle * Transform::from_xyz(at.x, 0.0, at.z),
        );
    }
    for along in [-0.35, 0.35] {
        kit.add(
            Torus::new(0.2, 0.24),
            &linen,
            bundle * Transform::from_xyz(0.0, along, 0.0),
        );
    }
}

/// Roux or stock: an earthenware crock of it, `deep` to its rim, with a
/// wooden spoon in it; stock with bits of the second colour floating.
fn pot_of(kit: &mut Kit, deep: f32, bits: bool) {
    let earth = kit.earthenware.clone();
    kit.round_dish(&earth, 0.62, 0.78, deep);
    let (food, accent, wood) = (kit.food.clone(), kit.accent.clone(), kit.wood.clone());
    kit.add(
        Cylinder::new(0.72, 0.02),
        &food,
        Transform::from_xyz(0.0, deep + 0.005, 0.0),
    );
    if bits {
        for at in [0.1, 0.35, 0.6, 0.85] {
            kit.add(
                Sphere::new(0.08),
                &accent,
                Transform::from_translation(round(at, 0.38, deep + 0.01))
                    .with_scale(Vec3::new(1.0, 0.5, 1.0)),
            );
        }
    }
    // The spoon, its bowl in the food and its handle over the rim.
    kit.add(
        Sphere::new(0.14),
        &wood,
        Transform::from_xyz(-0.2, deep + 0.02, 0.1).with_scale(Vec3::new(1.0, 0.3, 1.4)),
    );
    kit.add(
        Cylinder::new(0.04, 0.85),
        &wood,
        Transform::from_xyz(0.2, deep + 0.08, -0.25)
            .with_rotation(Quat::from_rotation_y(-0.6) * Quat::from_rotation_z(-1.35)),
    );
}

/// Fresh cheese: a soft round of it on its cheesecloth.
fn fresh_cheese(kit: &mut Kit) {
    let (linen, food, pale) = (kit.linen.clone(), kit.food.clone(), kit.pale.clone());
    kit.add(
        Cuboid::new(1.3, 0.03, 1.3),
        &linen,
        Transform::from_xyz(0.0, 0.015, 0.0).with_rotation(Quat::from_rotation_y(0.785)),
    );
    kit.add(
        Cylinder::new(0.6, 0.42),
        &food,
        Transform::from_xyz(0.0, 0.24, 0.0),
    );
    kit.add(
        Torus::new(0.5, 0.62),
        &food,
        Transform::from_xyz(0.0, 0.44, 0.0).with_scale(Vec3::new(1.0, 0.6, 1.0)),
    );
    // The cloth's weave pressed into its top.
    for x in [-0.25, 0.0, 0.25] {
        kit.add(
            Cuboid::new(0.02, 0.02, 0.8),
            &pale,
            Transform::from_xyz(x, 0.46, 0.0),
        );
    }
}

/// Warm milk: an earthenware cup of it with a handle.
fn cup(kit: &mut Kit) {
    let earth = kit.earthenware.clone();
    kit.round_dish(&earth, 0.48, 0.62, 0.75);
    kit.add(
        Torus::new(0.18, 0.28),
        &earth,
        Transform::from_xyz(0.68, 0.4, 0.0)
            .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
    );
    let food = kit.food.clone();
    kit.add(
        Cylinder::new(0.56, 0.02),
        &food,
        Transform::from_xyz(0.0, 0.755, 0.0),
    );
}

/// Yoghurt: a white bowl of it, thick and swirled, a wooden spoon in it.
fn yoghurt(kit: &mut Kit) {
    let crockery = kit.crockery.clone();
    kit.round_dish(&crockery, 0.45, 0.82, 0.4);
    let (food, wood) = (kit.food.clone(), kit.wood.clone());
    kit.add(
        Sphere::new(0.72),
        &food,
        Transform::from_xyz(0.0, 0.38, 0.0).with_scale(Vec3::new(1.0, 0.18, 1.0)),
    );
    kit.add(
        Torus::new(0.2, 0.3),
        &food,
        Transform::from_xyz(0.05, 0.48, 0.0).with_scale(Vec3::new(1.0, 0.5, 1.0)),
    );
    kit.add(
        Cylinder::new(0.04, 0.9),
        &wood,
        Transform::from_xyz(0.3, 0.6, -0.2)
            .with_rotation(Quat::from_rotation_y(-0.5) * Quat::from_rotation_z(-1.2)),
    );
}
