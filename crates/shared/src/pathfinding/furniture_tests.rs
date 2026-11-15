//! Routes go round furniture: it never blocks, but crossing it is dearer.

use super::*;
use crate::clearance::RoomMap;
use crate::creature::Size;
use crate::door::Side;
use crate::dungeon::{Cell, Level};
use crate::furniture::{Furniture, Material, Piece, TableStyle, Wood};
use crate::movement::{Gait, LevelWay};
use crate::spot::Spot;

/// A 9 × 9 room with a 2 × 3 table (2 east–west, 3 north–south) at its
/// middle, if `table`.
fn room_with_table(table: bool) -> Level {
    let mut level = Level::new(-5, -5, 11, 11);
    for y in -4..=4 {
        for x in -4..=4 {
            level.set(x, y, Cell::Floor);
        }
    }
    if table {
        level.place_furniture(Piece {
            at: (-1, -1),
            size: (2, 3),
            facing: Side::South,
            material: Material::Wood(Wood::Oak),
            furniture: Furniture::Table {
                style: TableStyle::Joined,
            },
        });
    }
    level
}

fn route(level: &Level, from: (i64, i64), to: (i64, i64)) -> SizedPath {
    let way = LevelWay {
        level,
        relief: None,
        terrain: None,
        gait: Gait::UNSKILLED,
    };
    let rooms = RoomMap::new(level, Size::Medium);
    let (from, to) = (Spot::middle_of(from), Spot::middle_of(to));
    find_sized_path(&way, &rooms, from, to, None, |_, _| true)
}

/// Whether a route's straight moves pass over any furnished cell.
fn crosses_furniture(level: &Level, path: &SizedPath) -> bool {
    path.spots.windows(2).any(|pair| {
        (0..=16).any(|i| {
            let (x, y) = pair[0].towards(pair[1], i, 16).cell();
            level.furniture_at(x, y).is_some()
        })
    })
}

#[test]
fn a_route_goes_round_a_table() {
    let open = route(&room_with_table(false), (-3, 0), (3, 0));
    let level = room_with_table(true);
    let round = route(&level, (-3, 0), (3, 0));
    assert!(round.reached);
    assert!(!crosses_furniture(&level, &round), "{:?}", round.spots);
    assert!(round.cost > open.cost);
}

#[test]
fn a_table_never_blocks() {
    // Walled in on every side but over the table, the way is still open.
    let mut level = room_with_table(true);
    for y in -4..=4 {
        if !(-1..=1).contains(&y) {
            level.set(0, y, Cell::Wall);
        }
    }
    let path = route(&level, (-3, 0), (3, 0));
    assert!(path.reached);
}

#[test]
fn mobs_route_round_a_table() {
    let level = room_with_table(true);
    let map = DistanceMap::to(&level, (3, 0));
    let plain = DistanceMap::to(&room_with_table(false), (3, 0));
    // Going round costs more than straight over open floor, but less than
    // over the table.
    let cost = map.cost((-3, 0)).expect("reached");
    assert!(cost > plain.cost((-3, 0)).unwrap());
    let mut at = (-3, 0);
    while at != (3, 0) {
        let step = map.towards(&level, at, |_| true).expect("a way on");
        at = step.from(at);
        assert!(level.furniture_at(at.0, at.1).is_none(), "{at:?}");
    }
}
