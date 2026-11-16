use shared::dungeon::{Cell, Level, Sight};
use shared::poi::PoiKind;
use shared::protocol::{LevelInfo, LevelKind};

use super::*;

fn info(index: u32, building: bool, basement: bool) -> LevelInfo {
    LevelInfo {
        kind: LevelKind::Poi(PoiKind::StoneTower),
        ruined: false,
        building,
        index,
        levels: 4,
        basement,
        bounds: (0, 0, 9, 9),
        arrival: shared::spot::Spot::middle_of((1, 1)),
        entering_site: false,
        place_name: None,
        ground: Default::default(),
    }
}

#[test]
fn floors_below_are_only_a_buildings_and_nearest_first() {
    assert_eq!(floors_below(&info(2, true, false)), vec![(1, 1), (0, 2)]);
    assert!(floors_below(&info(0, true, false)).is_empty());
    // A cave's depths are not floors.
    assert!(floors_below(&info(2, false, false)).is_empty());
    // The basement is the last level, under the ground.
    assert!(floors_below(&info(3, true, true)).is_empty());
}

#[test]
fn floors_above_are_only_a_buildings_and_nearest_first() {
    assert_eq!(floors_above(&info(1, true, false)), vec![(2, 1), (3, 2)]);
    assert_eq!(floors_above(&info(0, true, true)), vec![(1, 1), (2, 2)]);
    assert!(floors_above(&info(3, true, false)).is_empty());
    assert!(floors_above(&info(0, false, false)).is_empty());
    // Nothing is above a basement but the ground.
    assert!(floors_above(&info(3, true, true)).is_empty());
}

/// A 9 × 9 floor: a room of 3 × 3 in the middle walled round, ground outside.
fn ground_floor() -> Level {
    let mut level = Level::new(0, 0, 9, 9);
    for y in 0..9 {
        for x in 0..9 {
            let cell = match (x, y) {
                (3..=5, 3..=5) => Cell::Floor,
                (2..=6, 2..=6) => Cell::Wall,
                _ => Cell::Ground,
            };
            level.set(x, y, cell);
        }
    }
    level
}

fn all(level: &Level) -> Sight {
    let mut sight = Sight::new(level);
    for y in 0..9 {
        for x in 0..9 {
            sight.insert(x, y);
        }
    }
    sight
}

#[test]
fn remembered_rooms_are_not_drawn_but_slices_are() {
    let level = ground_floor();
    let known = all(&level);
    let nothing = Sight::new(&level);
    let drawn = drawn(&level, &known, &nothing, &[], ((4, 4), None), true);
    assert!(drawn.is_seen(0, 0), "outside");
    assert!(drawn.is_seen(2, 4), "a wall facing outside");
    assert!(!drawn.is_seen(4, 4), "the room inside");
}

#[test]
fn remembered_slices_can_be_turned_off_but_not_what_is_seen_now() {
    let level = ground_floor();
    let known = all(&level);
    let mut now = Sight::new(&level);
    now.insert(1, 1);
    let drawn = drawn(&level, &known, &now, &[], ((4, 4), None), false);
    assert!(drawn.is_seen(1, 1));
    assert!(!drawn.is_seen(0, 0));
}

#[test]
fn a_room_under_holes_all_the_way_up_is_drawn() {
    let level = ground_floor();
    let known = all(&level);
    let nothing = Sight::new(&level);
    let mut above = Level::new(0, 0, 9, 9);
    above.set(4, 4, Cell::Hole);
    let mut top = Level::new(0, 0, 9, 9);
    top.set(4, 4, Cell::Air);
    let both = [&above, &top];
    let drawn_now = drawn(&level, &known, &nothing, &both, ((4, 4), None), true);
    assert!(drawn_now.is_seen(4, 4));
    top.set(4, 4, Cell::Floor);
    let both = [&above, &top];
    let covered = drawn(&level, &known, &nothing, &both, ((4, 4), None), true);
    assert!(!covered.is_seen(4, 4));
}

#[test]
fn the_ground_floor_is_drawn_only_within_range() {
    let level = ground_floor();
    let known = all(&level);
    let drawn = drawn(&level, &known, &known, &[], ((0, 0), Some(3.0)), true);
    assert!(drawn.is_seen(3, 0));
    assert!(!drawn.is_seen(3, 1));
}

#[test]
fn darker_at_night_than_by_day() {
    assert!((kept_per_storey(0.1, 0.5, 1.0) - 0.9).abs() < 1e-6);
    assert!((kept_per_storey(0.1, 0.5, 0.0) - 0.5).abs() < 1e-6);
    assert!(kept_per_storey(0.1, 0.5, 0.5) < 0.9);
}
