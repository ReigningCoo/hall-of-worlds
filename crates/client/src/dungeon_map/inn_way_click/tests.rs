use shared::hall::{GATE_SPACING, INN_WAY_OFFSET, WALKWAY_ROWS};
use shared::protocol::LevelInfo;

use super::*;
use crate::dungeon_map::CELL_SIZE;

fn hall(origin: i64) -> CurrentDungeon {
    CurrentDungeon {
        info: Some(LevelInfo {
            kind: LevelKind::Hall { origin },
            ruined: false,
            building: false,
            index: 0,
            levels: 1,
            basement: false,
            bounds: (0, -1, 64, WALKWAY_ROWS + 2),
            arrival: shared::spot::Spot::middle_of((1, 1)),
            entering_site: false,
            place_name: None,
            ground: Default::default(),
        }),
        ..Default::default()
    }
}

/// Straight down onto the middle of a cell.
fn down_onto((x, y): (i64, i64)) -> Ray3d {
    let at = Vec3::new(x as f32 * CELL_SIZE, 10.0, y as f32 * CELL_SIZE);
    Ray3d::new(at, Dir3::NEG_Y)
}

#[test]
fn a_click_on_a_way_in_goes_to_the_tile_in_front() {
    let origin = 2 * GATE_SPACING;
    let dungeon = hall(origin);
    let x = INN_WAY_OFFSET;
    assert_eq!(inn_way_front(down_onto((x, -1)), &dungeon), Some((x, 0)));
    let south = (x, WALKWAY_ROWS);
    let front = Some((x, WALKWAY_ROWS - 1));
    assert_eq!(inn_way_front(down_onto(south), &dungeon), front);
}

#[test]
fn anywhere_else_is_not_one() {
    let dungeon = hall(0);
    let x = INN_WAY_OFFSET;
    // The void beside it, and the walkway in front.
    assert_eq!(inn_way_front(down_onto((x + 1, -1)), &dungeon), None);
    assert_eq!(inn_way_front(down_onto((x, 0)), &dungeon), None);
    // Not in the Hall.
    let site = CurrentDungeon::default();
    assert_eq!(inn_way_front(down_onto((x, -1)), &site), None);
}
