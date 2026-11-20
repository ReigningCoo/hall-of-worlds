use bevy::prelude::Vec3;
use shared::dungeon::{Cell, Level};

use super::glow_grid;
use super::lava::lava_lights;

/// A room of floor from -12 to 12 each way, walled round.
fn room() -> Level {
    let mut level = Level::new(-13, -13, 27, 27);
    for y in -12..=12 {
        for x in -12..=12 {
            level.set(x, y, Cell::Floor);
        }
    }
    level
}

/// Lava from `(x0, y0)` to `(x1, y1)`, both included.
fn pool(level: &mut Level, (x0, y0): (i64, i64), (x1, y1): (i64, i64)) {
    for y in y0..=y1 {
        for x in x0..=x1 {
            level.set(x, y, Cell::Lava);
        }
    }
}

#[test]
fn lava_lights_four_cells_from_its_rim_and_no_further() {
    let mut level = room();
    pool(&mut level, (0, -12), (0, 12));
    let glow = glow_grid(&level, &lava_lights(&level));
    assert!(glow.at(4, 0).x > 0.0);
    assert_eq!(glow.at(5, 0), Vec3::ZERO);
    // Orange: more red than green, more green than blue.
    let near = glow.at(1, 0);
    assert!(near.x > near.y && near.y > near.z);
}

#[test]
fn walls_block_lava_light() {
    let mut level = room();
    pool(&mut level, (0, -12), (0, 12));
    for y in -12..=12 {
        level.set(2, y, Cell::Wall);
    }
    let glow = glow_grid(&level, &lava_lights(&level));
    assert!(glow.at(1, 0).x > 0.0);
    assert_eq!(glow.at(3, 0), Vec3::ZERO);
}

#[test]
fn inner_lava_gives_no_light() {
    let mut level = room();
    pool(&mut level, (-2, -2), (2, 2));
    let lights = lava_lights(&level);
    // The 5 × 5 pool's rim: 16 cells, none of the inner 3 × 3.
    assert_eq!(lights.len(), 16);
    assert!(lights.iter().all(|light| {
        let (x, y) = light.cell;
        x.abs() == 2 || y.abs() == 2
    }));
}

#[test]
fn a_big_pool_is_no_brighter_than_a_strip() {
    let mut strip = room();
    pool(&mut strip, (-12, 0), (12, 0));
    let mut big = room();
    pool(&mut big, (-12, -8), (12, 0));
    let strip_glow = glow_grid(&strip, &lava_lights(&strip));
    let big_glow = glow_grid(&big, &lava_lights(&big));
    for y in 1..=5 {
        assert_eq!(big_glow.at(0, y), strip_glow.at(0, y));
    }
}
