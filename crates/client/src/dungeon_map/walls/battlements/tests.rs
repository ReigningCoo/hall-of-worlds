use bevy::mesh::VertexAttributeValues;
use bevy::prelude::*;
use shared::dungeon::{Cell, Level, Sight};
use std::collections::{BTreeMap, BTreeSet};

use super::super::super::mesh::{LightGrid, Surface, View, build_posed};
use super::super::super::rampart::OVERHANG;
use super::super::super::theme::Shape;
use super::super::super::{CELL_SIZE, WALL_HEIGHT};
use super::super::Above;

/// How high the battlements' merlons stand above the walk: 1.5 metres, a
/// metre being two thirds of a cell (`rampart::parapet`).
const MERLON_TOP: f32 = CELL_SIZE;

/// An upper floor's tower: a room `1..4` walled round `0..5`, with `outside`
/// round it (open air, or a ruin's holes).
fn tower(outside: Cell) -> Level {
    let mut level = Level::new(-2, -2, 9, 9);
    for y in -2..7 {
        for x in -2..7 {
            let wall = (0..5).contains(&x) && (0..5).contains(&y);
            let room = (1..4).contains(&x) && (1..4).contains(&y);
            level.set(
                x,
                y,
                if room {
                    Cell::Floor
                } else if wall {
                    Cell::Wall
                } else {
                    outside
                },
            );
        }
    }
    level
}

/// The highest point of the walls drawn, and how far east they reach, with
/// `above` on the tower `level`, seen from inside.
fn walls_reach(level: &Level, above: Above) -> (f32, f32) {
    let mut sight = Sight::new(level);
    sight.add_view(level, 2, 2, None);
    let views = vec![View {
        cell: (2, 2),
        sight: sight.clone(),
    }];
    let meshes = build_posed(
        level,
        (&views, &sight, &sight, &Sight::new(level)),
        &LightGrid::new(level),
        &[],
        (&[], &BTreeSet::new(), &[], &[], false),
        (
            Vec3::new(0.0, 6000.0, 600.0),
            Shape::Built,
            true,
            false,
            above,
        ),
        None,
        &BTreeMap::new(),
    );
    let walls = &meshes.solid[Surface::Wall as usize];
    let Some(VertexAttributeValues::Float32x3(positions)) =
        walls.attribute(Mesh::ATTRIBUTE_POSITION)
    else {
        panic!("walls are drawn");
    };
    let top = positions.iter().map(|p| p[1]).fold(f32::MIN, f32::max);
    let east = positions.iter().map(|p| p[0]).fold(f32::MIN, f32::max);
    (top, east)
}

fn close(a: f32, b: f32) -> bool {
    (a - b).abs() < 0.01
}

/// The top storey's walls over open air get battlements, standing out past
/// the wall's face.
#[test]
fn the_top_storey_has_battlements() {
    let (top, east) = walls_reach(&tower(Cell::Air), Above::Nothing);
    assert!(close(top, WALL_HEIGHT + MERLON_TOP), "{top}");
    let face = 4.0 * CELL_SIZE + super::super::half_thickness(Shape::Built, WALL_HEIGHT);
    assert!(close(east, face + OVERHANG), "{east}");
}

/// A storey with another on it has none: every upper floor is edged by air.
#[test]
fn a_middle_storey_has_none() {
    let floor_above = tower(Cell::Air);
    let (top, _) = walls_reach(&tower(Cell::Air), Above::Floor(&floor_above));
    assert!(close(top, WALL_HEIGHT), "{top}");
    let (top, _) = walls_reach(&tower(Cell::Air), Above::Unknown);
    assert!(close(top, WALL_HEIGHT), "{top}");
}

/// Where the floor above is open air over the walls (a building narrowing as
/// it goes up), they are the top and get battlements.
#[test]
fn walls_under_open_air_above_have_them() {
    let mut set_back = Level::new(-2, -2, 9, 9);
    for y in -2..7 {
        for x in -2..7 {
            set_back.set(x, y, Cell::Air);
        }
    }
    let (top, _) = walls_reach(&tower(Cell::Air), Above::Floor(&set_back));
    assert!(close(top, WALL_HEIGHT + MERLON_TOP), "{top}");
}

/// A ruin's broken floor (holes) is no open air: no battlements beside it.
#[test]
fn holes_get_none() {
    let (top, _) = walls_reach(&tower(Cell::Hole), Above::Nothing);
    assert!(close(top, WALL_HEIGHT), "{top}");
}
