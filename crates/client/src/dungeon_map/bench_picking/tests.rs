use bevy::prelude::*;
use shared::crafting_bench::Craft;
use shared::door::Side;
use shared::dungeon::{Cell, Level, Sight};
use shared::furniture::{Furniture, Material, Piece, Wood};

use super::super::ground_hit::Ground;
use super::{bench_clicked, into_box};
use crate::dungeon_map::CELL_SIZE;

#[test]
fn a_ray_goes_into_a_box_from_its_near_side() {
    let ray = Ray3d::new(Vec3::new(-10.0, 1.0, 0.5), Dir3::X);
    let distance = into_box(ray, Vec3::ZERO, Vec3::ONE).expect("in");
    assert!((distance - 10.0).abs() < 1e-4);
    let past = Ray3d::new(Vec3::new(-10.0, 3.0, 0.5), Dir3::X);
    assert_eq!(into_box(past, Vec3::ZERO, Vec3::ONE), None);
}

/// Looking down at a bench from above and in front finds it, but not once it
/// has never been seen.
#[test]
fn a_seen_bench_is_clicked_from_above() {
    let mut level = Level::new(0, 0, 8, 8);
    for y in 0..8 {
        for x in 0..8 {
            level.set(x, y, Cell::Floor);
        }
    }
    let piece = Piece {
        at: (3, 2),
        size: (2, 1),
        facing: Side::South,
        material: Material::Wood(Wood::Oak),
        furniture: Furniture::CraftingBench {
            craft: Craft::Alchemy,
        },
    };
    level.place_furniture(piece);
    let target = Vec3::new(3.5 * CELL_SIZE, CELL_SIZE * 0.4, 2.0 * CELL_SIZE);
    let eye = target + Vec3::new(0.0, 10.0, 6.0) * CELL_SIZE;
    let ray = Ray3d::new(eye, Dir3::new(target - eye).unwrap());
    let mut seen = Sight::new(&level);
    assert_eq!(bench_clicked(ray, &level, &seen, &[], Ground(None)), None);
    for (x, y) in piece.cells() {
        seen.insert(x, y);
    }
    assert_eq!(
        bench_clicked(ray, &level, &seen, &[], Ground(None)),
        Some(piece)
    );
}
