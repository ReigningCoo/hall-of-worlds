use std::collections::BTreeSet;

use super::spots::*;
use super::upstairs::lengths;
use super::*;
use crate::dungeon::{Cell, Level, RoomId};
use crate::site::SiteLevel;

fn floor(index: u32) -> SiteLevel {
    level(index).expect("a floor")
}

/// A number summing up a floor (cells, rooms, doors, locks, stairs), for the
/// golden test.
fn fingerprint(site_level: &SiteLevel) -> u64 {
    let level = &site_level.level;
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    let mut add = |value: u64| hash = (hash ^ value).wrapping_mul(0x0000_0100_0000_01b3);
    for y in 0..DEPTH {
        for x in 0..WIDTH {
            add(level.cell(x, y) as u64);
            add(level.room(x, y).map_or(u64::MAX, |r| r.0 as u64));
            add(u64::from(level.is_shut(x, y)) | u64::from(level.is_locked(x, y)) << 1);
        }
    }
    for stairs in &site_level.stairs {
        add(stairs.x as u64);
        add(stairs.y as u64);
    }
    hash
}

/// The cells that can be walked to from `from`, opening every door that
/// isn't locked on the way.
fn reached(level: &Level, from: (i64, i64)) -> BTreeSet<(i64, i64)> {
    let open = level.with_doors_open();
    let mut seen = BTreeSet::from([from]);
    let mut stack = vec![from];
    while let Some((x, y)) = stack.pop() {
        for (dx, dy) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
            let next = (x + dx, y + dy);
            if open.is_walkable(next.0, next.1) && seen.insert(next) {
                stack.push(next);
            }
        }
    }
    seen
}

fn walkable(level: &Level) -> BTreeSet<(i64, i64)> {
    (0..DEPTH)
        .flat_map(|y| (0..WIDTH).map(move |x| (x, y)))
        .filter(|&(x, y)| level.with_all_doors_open().is_walkable(x, y))
        .collect()
}

fn room_of(index: u32, kind: RoomKind) -> Vec<InnRoom> {
    rooms(index)
        .into_iter()
        .filter(|room| room.kind == kind)
        .collect()
}

/// Changing the inn's shape changes this: only on purpose.
#[test]
fn golden_fingerprints() {
    let prints = [fingerprint(&floor(GROUND)), fingerprint(&floor(UPSTAIRS))];
    assert_eq!(
        prints,
        [3017963899545757124, 5801853540422890675],
        "{prints:?}"
    );
    assert!(level(FLOORS).is_none());
    assert!(rooms(FLOORS).is_empty());
}

#[test]
fn everything_reached_but_the_treasury() {
    let ground = floor(GROUND);
    let from = ground.entrance.expect("an entrance");
    assert_eq!(from, ARRIVAL);
    let got = reached(&ground.level, from);
    let treasury = &room_of(GROUND, RoomKind::Treasury)[0];
    let expected: BTreeSet<_> = (walkable(&ground.level).into_iter())
        .filter(|&(x, y)| !treasury.contains(x, y) && (x, y) != TREASURY_DOOR)
        .collect();
    assert_eq!(got, expected);
    // Upstairs: the corridors and the one own room open to everyone, its
    // door too; the other bedrooms are locked.
    let up = floor(UPSTAIRS);
    let open = &own_rooms()[0];
    assert_eq!(open.tier, OPEN_TIER);
    let corridor = &room_of(UPSTAIRS, RoomKind::Corridor)[0];
    let expected: BTreeSet<_> = (walkable(&up.level).into_iter())
        .filter(|&(x, y)| {
            corridor.contains(x, y) || open.rect.contains(x, y) || (x, y) == open.door
        })
        .collect();
    assert_eq!(reached(&up.level, STAIRS), expected);
}

#[test]
fn stairs_join_the_floors_on_one_cell() {
    let (ground, up) = (floor(GROUND), floor(UPSTAIRS));
    assert_eq!(ground.stairs.len(), 1);
    assert_eq!(up.stairs.len(), 1);
    assert_eq!((ground.stairs[0].x, ground.stairs[0].y), STAIRS);
    assert_eq!((up.stairs[0].x, up.stairs[0].y), STAIRS);
    assert_eq!(ground.stairs[0].to, crate::site::Exit::Level(UPSTAIRS));
    assert_eq!(up.stairs[0].to, crate::site::Exit::Level(GROUND));
    assert!(ground.level.is_walkable(STAIRS.0, STAIRS.1));
    assert!(up.level.is_walkable(STAIRS.0, STAIRS.1));
    assert!(up.entrance.is_none());
}

#[test]
fn rooms_are_marked_on_their_cells() {
    for index in [GROUND, UPSTAIRS] {
        let site_level = floor(index);
        let level = &site_level.level;
        let all = rooms(index);
        for (n, room) in all.iter().enumerate() {
            assert_eq!(room.id, RoomId(n as u16 + 1));
            for y in 0..DEPTH {
                for x in 0..WIDTH {
                    let inside = room.contains(x, y);
                    assert_eq!(level.room(x, y) == Some(room.id), inside, "{x}, {y}");
                    if inside {
                        assert_eq!(level.cell(x, y), Cell::Floor);
                    }
                }
            }
        }
        // The outer wall all round, and no windows out.
        for x in 0..WIDTH {
            for y in [0, DEPTH - 1] {
                let cell = level.cell(x, y);
                assert!(cell == Cell::Wall || (x, y) == ENTRANCE && index == GROUND);
            }
        }
        for y in 0..DEPTH {
            for x in [0, WIDTH - 1] {
                assert_eq!(level.cell(x, y), Cell::Wall);
            }
        }
    }
}

#[test]
fn ground_floor_layout() {
    let ground = floor(GROUND);
    let level = &ground.level;
    let foyer = &room_of(GROUND, RoomKind::Foyer)[0];
    assert_eq!((FOYER.width, FOYER.height), (3, 3));
    assert!(foyer.contains(ARRIVAL.0, ARRIVAL.1) && foyer.contains(ARCHWAY.0, ARCHWAY.1));
    assert_eq!(level.cell(ENTRANCE.0, ENTRANCE.1), Cell::Door);
    assert!(level.is_shut(ENTRANCE.0, ENTRANCE.1));
    assert_eq!(level.cell(ARCHWAY.0, ARCHWAY.1), Cell::Floor);
    // Private rooms (doored) and booths (open), all one size.
    let private = room_of(GROUND, RoomKind::PrivateRoom);
    let booths = room_of(GROUND, RoomKind::Booth);
    assert_eq!((private.len(), booths.len()), (7, 7));
    for room in private.iter().chain(&booths) {
        assert_eq!(room.area(), private[0].area());
    }
    assert!(private.iter().all(|room| room.door.is_some()));
    assert!(booths.iter().all(|room| room.door.is_none()));
    // The bar between the rows: its counter, the gap and the storage in the
    // main room, the stairs and tables' place too.
    let main = &room_of(GROUND, RoomKind::Main)[0];
    for rect in [STORAGE, BAR_GAP, BAR, BACK_FIREPLACE, TABLES] {
        assert!(cells(rect).all(|(x, y)| main.contains(x, y)), "{rect:?}");
    }
    assert!(main.contains(STAIRS.0, STAIRS.1) && TABLES.contains(STAIRS.0, STAIRS.1));
    assert!(BAR.contains(BAR_FLAP.0, BAR_FLAP.1));
    for rect in FRONT_FIREPLACES {
        assert!(cells(rect).all(|(x, y)| main.contains(x, y)));
        assert!(cells(rect).all(|(x, y)| level.cell(x, y + 1) == Cell::Wall));
    }
    assert!(cells(STORAGE).all(|(x, y)| level.cell(x, y - 1) == Cell::Wall));
    // Doors from behind the bar into the kitchen and the brewing room.
    let kitchen = &room_of(GROUND, RoomKind::Kitchen)[0];
    let brewing = &room_of(GROUND, RoomKind::Brewing)[0];
    let ageing = &room_of(GROUND, RoomKind::Ageing)[0];
    let (kx, ky) = KITCHEN_DOOR;
    assert!(kitchen.contains(kx - 1, ky) && BAR_GAP.contains(kx + 1, ky));
    let (bx, by) = BREWING_DOOR;
    assert!(brewing.contains(bx + 1, by) && BAR_GAP.contains(bx - 1, by));
    let (ax, ay) = AGEING_DOOR;
    assert!(ageing.contains(ax - 1, ay) && brewing.contains(ax + 1, ay));
    assert!(cells(STAIR_DOWN_CORNER).all(|(x, y)| kitchen.contains(x, y)));
    // The treasury: 5 × 5, locked, a window either side of its door into
    // the kitchen.
    let treasury = &room_of(GROUND, RoomKind::Treasury)[0];
    assert_eq!(treasury.rects, [TREASURY]);
    assert_eq!((TREASURY.width, TREASURY.height), (5, 5));
    assert_eq!(treasury.door, Some(TREASURY_DOOR));
    assert!(level.is_locked(TREASURY_DOOR.0, TREASURY_DOOR.1));
    let (tx, ty) = TREASURY_DOOR;
    assert!(kitchen.contains(tx, ty + 1) && treasury.contains(tx, ty - 1));
    for (x, y) in TREASURY_WINDOWS {
        assert_eq!(level.cell(x, y), Cell::Window);
        assert!(kitchen.contains(x, y + 1) && treasury.contains(x, y - 1));
    }
}

#[test]
fn upstairs_bedrooms_grow_away_from_the_stairs() {
    let up = floor(UPSTAIRS);
    let corridor = &room_of(UPSTAIRS, RoomKind::Corridor)[0];
    assert!(corridor.contains(STAIRS.0, STAIRS.1));
    let bedrooms: Vec<_> = rooms(UPSTAIRS)
        .into_iter()
        .filter(|room| {
            matches!(
                room.kind,
                RoomKind::Bedroom { .. } | RoomKind::OwnRoom { .. }
            )
        })
        .collect();
    assert!(bedrooms.len() >= 24, "{}", bedrooms.len());
    let tier = |room: &InnRoom| match room.kind {
        RoomKind::Bedroom { tier } | RoomKind::OwnRoom { tier } => tier,
        _ => unreachable!(),
    };
    let size = |t: u8| (bedrooms.iter().filter(move |r| tier(r) == t)).map(InnRoom::area);
    for t in 1..CORNER_TIER {
        let biggest = size(t).max().expect("rooms of each tier");
        let next_least = size(t + 1).min().expect("rooms of each tier");
        assert!(biggest < next_least, "tier {t}");
    }
    for room in &bedrooms {
        let (x, y) = room.door.expect("a door");
        assert_eq!(up.level.cell(x, y), Cell::Door);
        let beside = [(0, -1), (1, 0), (0, 1), (-1, 0)].map(|(dx, dy)| (x + dx, y + dy));
        assert!(beside.iter().any(|&(x, y)| corridor.contains(x, y)));
        assert!(beside.iter().any(|&(x, y)| room.contains(x, y)));
    }
    // The H's corridors are 2 wide.
    for rect in &corridor.rects {
        assert_eq!(rect.width.min(rect.height), 2);
    }
    assert_eq!(lengths(26), [3, 4, 5, 11]);
    assert_eq!(lengths(18), [3, 4, 9]);
}

fn cells(rect: crate::site::Rect) -> impl Iterator<Item = (i64, i64)> {
    (rect.y..rect.y + rect.height)
        .flat_map(move |y| (rect.x..rect.x + rect.width).map(move |x| (x, y)))
}

/// Prints both floors (run with `cargo test -p shared print_inn --
/// --ignored --nocapture`): `#` wall, `+` door (`L` locked), `=` window,
/// `<` the stairs, `.` floor, rooms by a letter each.
#[test]
#[ignore]
fn print_inn() {
    for index in [GROUND, UPSTAIRS] {
        let site_level = floor(index);
        let level = &site_level.level;
        println!("Floor {index}:");
        for y in 0..DEPTH {
            let row: String = (0..WIDTH)
                .map(|x| match level.cell(x, y) {
                    _ if (x, y) == STAIRS => '<',
                    Cell::Wall => '#',
                    Cell::Door if level.is_locked(x, y) => 'L',
                    Cell::Door => '+',
                    Cell::Window => '=',
                    _ => level.room(x, y).map_or('.', |r| {
                        let letters = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKMNOPQRSTUVWXYZ";
                        letters[(r.0 as usize - 1) % letters.len()] as char
                    }),
                })
                .collect();
            println!("{row}");
        }
    }
}
