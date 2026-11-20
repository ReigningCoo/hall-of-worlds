use std::collections::HashMap;

use bevy::prelude::*;
use shared::dungeon::{Cell, Level};
use shared::fungus::{Fungi, FungusPatch};
use shared::herb::STAGES;

use super::fungi::{fungus_lights, groups};
use super::{FixedLight, glow_grid};

/// A room of floor from -9 to 9 each way, walled round.
fn room() -> Level {
    let mut level = Level::new(-10, -10, 21, 21);
    for y in -9..=9 {
        for x in -9..=9 {
            level.set(x, y, Cell::Floor);
        }
    }
    level
}

/// A patch of `fungus` full grown at minute 0, or one stage short of it.
fn patch(fungus: &str, full: bool) -> FungusPatch {
    let def = Fungi::built_in().get(fungus).expect("a built-in fungus");
    (0..100_000)
        .map(|phase| FungusPatch {
            fungus: fungus.to_owned(),
            phase,
            picked: None,
        })
        .find(|patch| (patch.stage(def, 0) == STAGES) == full && patch.stage(def, 0) > 0)
        .expect("a phase at that stage")
}

#[test]
fn a_one_cell_gap_is_ignored_but_two_part_groups() {
    let cells = [(0, 0), (2, 0), (2, 2), (6, 0), (9, 9)];
    let glowing: HashMap<(i64, i64), ()> = cells.iter().map(|&cell| (cell, ())).collect();
    let found = groups(&glowing);
    assert_eq!(found.len(), 3);
    assert_eq!(found[0], vec![(0, 0), (2, 0), (2, 2)]);
    assert_eq!(found[1], vec![(6, 0)]);
    assert_eq!(found[2], vec![(9, 9)]);
}

#[test]
fn a_group_reaches_the_root_of_its_cells_in_one_blended_colour() {
    let mut level = room();
    for x in 0..9 {
        level.set_fungus(x - 4, 0, patch("glowcap", true));
    }
    level.set_fungus(-4, 1, patch("emberbloom", true));
    let lights = fungus_lights(&level, 0);
    assert_eq!(lights.len(), 10);
    // Ten cells: √10 rounds to 3, the same for every cell, one colour.
    assert!(lights.iter().all(|light| light.radius == 3));
    assert!(lights.iter().all(|light| light.colour == lights[0].colour));
    // Mostly glowcap's green, a little of emberbloom's red.
    let colour = lights[0].colour;
    assert!(colour.y > colour.x && colour.x > 0.0);
}

#[test]
fn only_full_grown_glowing_patches_give_light() {
    let mut level = room();
    level.set_fungus(0, 0, patch("glowcap", false));
    // Grown, but no glow of its own.
    let plain = Fungi::built_in().iter().find(|def| def.glow.is_none());
    level.set_fungus(
        3,
        3,
        patch(&plain.expect("a fungus that doesn't glow").key, true),
    );
    assert!(fungus_lights(&level, 0).is_empty());
    // Picked: back to an earlier stage, dark again.
    level.set_fungus(0, 0, patch("glowcap", true));
    assert_eq!(fungus_lights(&level, 0).len(), 1);
    level.set_fungus_picked(0, 0, Some(0));
    assert!(fungus_lights(&level, 0).is_empty());
}

#[test]
fn many_lights_of_one_colour_are_no_brighter_than_the_nearest() {
    let level = room();
    let light = |cell| FixedLight {
        cell,
        radius: 3,
        colour: Vec3::new(0.0, 0.5, 0.0),
    };
    let one = glow_grid(&level, &[light((0, 0))]);
    let three = glow_grid(&level, &[light((0, 0)), light((1, 0)), light((-1, 0))]);
    assert_eq!(three.at(0, 0), one.at(0, 0));
    assert_eq!(three.at(0, 0), Vec3::new(0.0, 0.5, 0.0));
    // Further out, the nearest of them counts.
    assert!(three.at(3, 0).y > one.at(3, 0).y);
    // Another colour adds to it.
    let red = FixedLight {
        colour: Vec3::new(0.5, 0.0, 0.0),
        ..light((0, 0))
    };
    let mixed = glow_grid(&level, &[light((0, 0)), red]);
    assert_eq!(mixed.at(0, 0), Vec3::new(0.5, 0.5, 0.0));
}

#[test]
fn walls_block_fixed_lights() {
    let mut level = room();
    for y in -9..=9 {
        level.set(2, y, Cell::Wall);
    }
    let glow = glow_grid(
        &level,
        &[FixedLight {
            cell: (0, 0),
            radius: 4,
            colour: Vec3::ONE,
        }],
    );
    assert!(glow.at(1, 0).x > 0.0);
    assert_eq!(glow.at(3, 0), Vec3::ZERO);
}
