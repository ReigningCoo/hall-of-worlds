use std::collections::{BTreeMap, BTreeSet};

use super::super::spots::*;
use super::super::{DEPTH, GROUND, InnRoom, RoomKind, UPSTAIRS, WIDTH, level, rooms};
use super::pieces;
use crate::bar::BarKind;
use crate::dungeon::{Cell, Level};
use crate::furniture::{Furniture, Piece};

fn floor(index: u32) -> Level {
    level(index).expect("a floor").level
}

fn neighbours((x, y): (i64, i64)) -> [(i64, i64); 4] {
    [(x, y - 1), (x + 1, y), (x, y + 1), (x - 1, y)]
}

/// Whether a piece lets anyone through its cells for these paths: only the
/// bar's flap.
fn passable(piece: &Piece) -> bool {
    matches!(piece.furniture, Furniture::BarCounter { counter } if counter.kind == BarKind::Flap)
}

/// The cells reached from `from` on foot, around every piece but the flap,
/// through every door that isn't locked.
fn reached(level: &Level, from: (i64, i64)) -> BTreeSet<(i64, i64)> {
    let open = level.with_doors_open();
    let free = |(x, y): (i64, i64)| {
        open.is_walkable(x, y) && level.furniture_at(x, y).is_none_or(passable)
    };
    let mut seen = BTreeSet::from([from]);
    let mut stack = vec![from];
    while let Some(cell) = stack.pop() {
        for next in neighbours(cell) {
            if free(next) && seen.insert(next) {
                stack.push(next);
            }
        }
    }
    seen
}

fn room_kind(rooms: &[InnRoom], (x, y): (i64, i64)) -> Option<RoomKind> {
    (rooms.iter())
        .find(|room| room.contains(x, y))
        .map(|room| room.kind)
}

#[test]
fn nothing_overlaps_and_everything_is_placed() {
    for index in [GROUND, UPSTAIRS] {
        let rooms = rooms(index);
        let pieces = pieces(index, &rooms);
        let level = floor(index);
        let mut floor_cells = BTreeMap::new();
        for piece in pieces.iter().filter(|p| !p.furniture.stands_on_furniture()) {
            assert!(piece.is_well_made(), "{piece:?}");
            for cell in piece.cells() {
                assert_eq!(level.cell(cell.0, cell.1), Cell::Floor, "{piece:?}");
                assert!(room_kind(&rooms, cell).is_some(), "{piece:?} outside");
                assert!(![STAIRS, ARRIVAL, START].contains(&cell), "{piece:?}");
                let other = floor_cells.insert(cell, piece.at);
                assert!(other.is_none(), "{piece:?} over {other:?}");
            }
        }
        let placed = level.furniture().count() + level.furniture_on_top().count();
        assert_eq!(placed, pieces.len(), "every piece stays where it was put");
    }
}

#[test]
fn decor_fits_once_a_place() {
    for index in [GROUND, UPSTAIRS] {
        let level = floor(index);
        let mut slots = BTreeSet::new();
        for decor in level.decor() {
            assert!(decor.fits(&level), "{decor:?}");
            assert!(slots.insert(format!("{:?}", decor.slot())), "{decor:?}");
            assert!(
                !decor.cells().contains(&STAIRS)
                    || matches!(decor, crate::decor::Decor::Wall { .. })
            );
        }
        assert!(
            slots.len() > 100,
            "decorations on most walls: {}",
            slots.len()
        );
    }
}

#[test]
fn about_thirty_table_groups_scattered() {
    let level = floor(GROUND);
    let tables = (level.furniture())
        .filter(|p| p.cells().all(|(x, y)| TABLES.contains(x, y)))
        .filter(|p| {
            matches!(
                p.furniture,
                Furniture::Table { .. } | Furniture::GamesTable { .. } | Furniture::Barrel { .. }
            )
        })
        .count();
    assert!((25..=30).contains(&tables), "{tables} groups");
}

#[test]
fn kept_places_stay_clear() {
    let level = floor(GROUND);
    let clear = |(x, y): (i64, i64)| level.furniture_at(x, y).is_none();
    for x in STAIR_DOWN_CORNER.x..STAIR_DOWN_CORNER.x + STAIR_DOWN_CORNER.width {
        for y in STAIR_DOWN_CORNER.y..STAIR_DOWN_CORNER.y + STAIR_DOWN_CORNER.height {
            assert!(clear((x, y)), "the corner kept for a stair down");
        }
    }
    for (x, y) in TREASURY_WINDOWS.into_iter().chain([TREASURY_DOOR]) {
        assert!(clear((x, y + 1)), "before the treasury's window");
    }
    for x in BAR_GAP.x..BAR_GAP.x + BAR_GAP.width {
        for y in BAR_GAP.y..BAR_GAP.y + BAR_GAP.height {
            assert!(clear((x, y)), "behind the bar");
        }
    }
    for fire in FRONT_FIREPLACES.into_iter().chain([BACK_FIREPLACE]) {
        for x in fire.x..fire.x + fire.width {
            let front = if fire.y == STORAGE.y {
                fire.y + 1
            } else {
                fire.y - 1
            };
            assert!(clear((x, front)), "before a fireplace");
        }
    }
}

/// From inside the entrance: the stairs, every door but the treasury's from
/// both sides, behind the bar through its flap, and something beside every
/// piece but a table among its seats (or the treasury's) can be walked to.
#[test]
fn paths_to_everything_downstairs() {
    let level = floor(GROUND);
    let rooms = rooms(GROUND);
    let reached = reached(&level, ARRIVAL);
    assert!(reached.contains(&STAIRS));
    assert!(reached.contains(&START), "a new character's start");
    assert!(reached.contains(&BAR_FLAP));
    assert!(reached.contains(&(BAR_FLAP.0, BAR_GAP.y)), "behind the bar");
    for room in &rooms {
        if room.kind == RoomKind::Treasury {
            continue;
        }
        let door = room.door.unwrap_or(ARCHWAY);
        assert!(reached.contains(&door), "{:?}'s door", room.kind);
        let inside = neighbours(door)
            .into_iter()
            .any(|cell| room.contains(cell.0, cell.1));
        assert!(inside || room.door.is_none());
    }
    for door in [KITCHEN_DOOR, BREWING_DOOR, AGEING_DOOR] {
        assert!(reached.contains(&door));
    }
    let seats = |piece: &Piece| {
        matches!(
            piece.furniture,
            Furniture::Chair { .. } | Furniture::Bench { .. }
        )
    };
    let beside =
        |piece: &Piece, ok: &dyn Fn((i64, i64)) -> bool| piece.cells().flat_map(neighbours).any(ok);
    for piece in level.furniture() {
        if room_kind(&rooms, piece.at) == Some(RoomKind::Treasury) {
            continue;
        }
        let walked = |cell| reached.contains(&cell);
        let seated = |(x, y)| level.furniture_at(x, y).is_some_and(seats);
        let ok = beside(piece, &walked)
            || (piece.furniture.is_table()
                || matches!(
                    piece.furniture,
                    Furniture::GamesTable { .. } | Furniture::Barrel { .. }
                ))
                && beside(piece, &seated);
        assert!(ok, "nothing beside {piece:?} is reached");
    }
}

/// Upstairs: every corridor cell and the open own room's door, and beside
/// every piece in the open own room.
#[test]
fn paths_to_everything_upstairs() {
    let level = floor(UPSTAIRS);
    let rooms = rooms(UPSTAIRS);
    let reached = reached(&level, STAIRS);
    for room in &rooms {
        if room.kind == RoomKind::Corridor {
            for rect in &room.rects {
                for x in rect.x..rect.x + rect.width {
                    for y in rect.y..rect.y + rect.height {
                        assert!(reached.contains(&(x, y)), "corridor at {:?}", (x, y));
                    }
                }
            }
        }
        if let Some(door) = room.door {
            let front = neighbours(door)
                .into_iter()
                .filter(|c| room.contains(c.0, c.1));
            for (x, y) in front {
                assert!(level.furniture_at(x, y).is_none(), "in front of {door:?}");
            }
        }
    }
    let open = (rooms.iter()).find(|room| room.kind == RoomKind::OwnRoom { tier: 1 });
    let open = open.expect("the open tier's room");
    for piece in level.furniture().filter(|p| open.contains(p.at.0, p.at.1)) {
        let ok = piece
            .cells()
            .flat_map(neighbours)
            .any(|c| reached.contains(&c));
        assert!(ok, "{piece:?}");
    }
    assert!(
        reached
            .iter()
            .all(|&(x, y)| (0..WIDTH).contains(&x) && (0..DEPTH).contains(&y))
    );
}

#[test]
fn the_bar_pours_from_its_storage() {
    let level = floor(GROUND);
    let bar = level.furniture_at(BAR.x, BAR.y).expect("the bar");
    let storage = crate::bar::room_storage(&level, bar);
    assert!(storage.len() >= 8, "{} pieces of storage", storage.len());
}
