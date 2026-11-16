use bevy::prelude::*;
use shared::dungeon::{Cell, Level, Sight};
use shared::site::{Gate, Rect, WallLook, WallMaterial};
use std::collections::{BTreeMap, BTreeSet};

use super::super::mesh::{LightGrid, Surface, View, build_posed};
use super::super::theme::Shape;
use super::parapet::{CRENEL_HEIGHT, MERLON_HEIGHT, METRE, OVERHANG};
use super::stone::stone_boxes;
use super::{Build, LOG_TILE, without_ramparts};
use crate::dungeon_map::town::atlas_uv;
use crate::dungeon_map::town::cell_centre;
use crate::dungeon_map::walls::half_thickness;

/// A walled square of ground, its gate in the south side.
fn walled() -> (Level, WallLook) {
    let mut level = Level::new(-2, -2, 12, 12);
    for y in -2..10 {
        for x in -2..10 {
            level.set(x, y, Cell::Ground);
        }
    }
    let wall = WallLook {
        rect: Rect {
            x: 0,
            y: 0,
            width: 8,
            height: 8,
        },
        material: WallMaterial::Log,
        gates: vec![Gate {
            x: 3,
            y: 7,
            runs_east_west: true,
        }],
        towers: Vec::new(),
        gaps: Vec::new(),
    };
    for y in 0..8 {
        for x in 0..8 {
            if wall.is_gate(x, y) {
                level.set(x, y, Cell::Door);
            } else if wall.has(x, y) {
                level.set(x, y, Cell::Wall);
            }
        }
    }
    (level, wall)
}

/// A wall round a site is drawn by its material in the town's picture, never
/// as the place's own walls.
#[test]
fn a_rampart_is_drawn_in_its_material() {
    let (level, wall) = walled();
    let mut sight = Sight::new(&level);
    sight.add_view(&level, 4, 9, None);
    let views = vec![View {
        cell: (4, 9),
        sight: sight.clone(),
    }];
    let meshes = build_posed(
        &level,
        (&views, &sight, &sight, &Sight::new(&level)),
        &LightGrid::new(&level),
        &[],
        (
            &[],
            &BTreeSet::new(),
            &[],
            std::slice::from_ref(&wall),
            false,
        ),
        (Vec3::new(0.0, 600.0, 2000.0), Shape::Built, false),
        None,
        &BTreeMap::new(),
    );
    assert!(!meshes.has_solid[Surface::Wall as usize], "no plain walls");
    let town = &meshes.solid[Surface::Town as usize];
    let Some(bevy::mesh::VertexAttributeValues::Float32x2(uvs)) =
        town.attribute(Mesh::ATTRIBUTE_UV_0)
    else {
        panic!("the town mesh has picture coordinates");
    };
    let [u0, v0] = atlas_uv(LOG_TILE, 0.0, 0.0);
    let [u1, v1] = atlas_uv(LOG_TILE, 1.0, 1.0);
    assert!(!uvs.is_empty());
    for &[u, v] in uvs {
        assert!((u0..=u1).contains(&u) && (v0..=v1).contains(&v), "{u}, {v}");
    }
}

/// A stone wall round `0..8`, a tower on its north side over `3..6`, and the
/// level with both: the tower's outline walls, the wall's own cells.
fn towered() -> (Level, WallLook) {
    let mut level = Level::new(-2, -2, 12, 12);
    for y in -2..10 {
        for x in -2..10 {
            level.set(x, y, Cell::Ground);
        }
    }
    let tower = Rect {
        x: 3,
        y: -1,
        width: 3,
        height: 3,
    };
    let wall = WallLook {
        rect: Rect {
            x: 0,
            y: 0,
            width: 8,
            height: 8,
        },
        material: WallMaterial::Stone,
        gates: Vec::new(),
        towers: vec![tower],
        gaps: Vec::new(),
    };
    for y in -1..2 {
        for x in 3..6 {
            let edge = x == 3 || x == 5 || y == -1 || y == 1;
            level.set(x, y, if edge { Cell::Wall } else { Cell::Floor });
        }
    }
    for y in 0..8 {
        for x in 0..8 {
            if wall.has(x, y) {
                level.set(x, y, Cell::Wall);
            }
        }
    }
    (level, wall)
}

/// A north-side cell's parapet stands on the wall's outer half, out past its
/// face, crenels a metre and merlons a metre and a half above the walk.
#[test]
fn a_parapet_is_on_the_outer_edge() {
    let (level, wall) = towered();
    let build = Build::of(WallMaterial::Stone);
    let boxes = stone_boxes(
        &wall,
        &level,
        &build,
        half_thickness(Shape::Built, 0.0),
        (1, 0),
    );
    let centre = cell_centre(1, 0);
    let parapet: Vec<_> = (boxes.iter())
        .filter(|(_, high)| high.y > build.height + 0.01)
        .collect();
    assert!(!parapet.is_empty());
    let near = |a: f32, b: f32| (a - b).abs() < 0.001;
    for (low, high) in &parapet {
        assert!(high.z < centre.z, "on the outer (north) half: {low} {high}");
        assert!(
            near(low.z, centre.z - build.half - OVERHANG),
            "out past the face"
        );
        let up = high.y - build.height;
        assert!(near(up, CRENEL_HEIGHT) || near(up, MERLON_HEIGHT), "{up}");
    }
    assert!(near(CRENEL_HEIGHT, METRE) && near(MERLON_HEIGHT, 1.5 * METRE));
    let merlons = (parapet.iter())
        .filter(|(_, high)| near(high.y - build.height, MERLON_HEIGHT))
        .count();
    assert_eq!(merlons, 2, "two merlons to a cell");
}

/// Where the wall meets a tower, the tower's wall doesn't reach out along the
/// wall's middle, and the wall and its parapet come on to the tower's face.
#[test]
fn a_wall_meets_a_tower_at_its_face() {
    let (level, wall) = towered();
    let walls = std::slice::from_ref(&wall);
    let shape = level.wall_shape(3, 0).expect("the tower's wall");
    assert!(shape.west, "the level joins the tower to the wall");
    let shape = without_ramparts(walls, &level, (3, 0), shape);
    assert!(!shape.west, "no arm towards the wall");
    assert!(shape.north && shape.south, "the tower's own wall stays");
    let building_half = half_thickness(Shape::Built, 0.0);
    let build = Build::of(WallMaterial::Stone);
    let boxes = stone_boxes(&wall, &level, &build, building_half, (2, 0));
    let face = cell_centre(3, 0).x - building_half;
    let near = |a: f32, b: f32| (a - b).abs() < 0.001;
    let reaching = |above: f32| {
        (boxes.iter()).any(|(_, high)| near(high.x, face) && near(high.y, build.height + above))
    };
    assert!(reaching(0.0), "the run reaches the tower's face");
    assert!(reaching(CRENEL_HEIGHT), "and so does its parapet");
    assert!(boxes.iter().all(|(_, high)| high.x <= face + 0.001));
}
