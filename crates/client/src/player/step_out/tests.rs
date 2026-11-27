use shared::clearance::{Crowding, Footing, Other};
use shared::creature::Size;
use shared::dungeon::{Cell, Level};
use shared::spot::Spot;

use super::*;

/// A floor room from `(-r, -r)` to `(r, r)`, walled round.
fn room(r: i64) -> Level {
    let mut level = Level::new(-r - 1, -r - 1, 2 * r + 3, 2 * r + 3);
    for y in -r..=r {
        for x in -r..=r {
            level.set(x, y, Cell::Floor);
        }
    }
    level
}

fn giant(at: Spot, crowding: Crowding) -> [Other; 1] {
    [Other {
        at,
        size: Size::Giant,
        crowding,
    }]
}

fn out(level: &Level, others: &[Other], at: Spot, entered: Option<Spot>) -> Option<Spot> {
    let walker = Walker {
        level,
        size: Size::Medium,
        footing: Footing::Walks,
        others,
        squeezes: false,
    };
    step_out(&walker, at, entered)
}

/// Giant (240) and Medium (80) circles touch 320 units apart.
const TOUCHING: u64 = 320;

#[test]
fn steps_out_to_the_closest_edge() {
    let level = room(4);
    let middle = Spot::middle_of((0, 0));
    let them = giant(middle, Crowding::Squeeze);
    let at = middle.offset(-100, 30);
    let to = out(&level, &them, at, Some(middle.offset(0, 600))).unwrap();
    assert!(to.distance(middle) >= TOUCHING && to.distance(middle) <= TOUCHING + 3);
    // The same way from them as the player was, not back where they came in.
    assert!(to.x < middle.x && to.y > middle.y && (to.y - middle.y) < (middle.x - to.x));
}

#[test]
fn at_the_exact_centre_steps_back_the_way_they_came() {
    let level = room(4);
    let middle = Spot::middle_of((0, 0));
    let them = giant(middle, Crowding::Squeeze);
    let to = out(&level, &them, middle, Some(middle.offset(600, 0))).unwrap();
    assert_eq!(to.y, middle.y);
    assert!(to.x > middle.x && to.distance(middle) >= TOUCHING);
    // Nowhere known to go back to: stays.
    assert_eq!(out(&level, &them, middle, None), None);
}

#[test]
fn with_no_room_at_the_closest_edge_steps_back_the_way_they_came() {
    let level = room(1);
    // Against the east wall: the closest edge is beyond it.
    let middle = Spot::middle_of((1, 0));
    let them = giant(middle, Crowding::Squeeze);
    let at = middle.offset(60, 0);
    let to = out(&level, &them, at, Some(Spot::middle_of((-1, 0)))).unwrap();
    assert!(to.x < middle.x && to.distance(middle) >= TOUCHING);
}

#[test]
fn only_someone_squeezed_past_is_stepped_out_of() {
    let level = room(4);
    let middle = Spot::middle_of((0, 0));
    let at = middle.offset(-100, 0);
    let clear = middle.offset(-600, 0);
    assert_eq!(out(&level, &[], at, Some(clear)), None);
    let players = giant(middle, Crowding::Pass);
    assert_eq!(out(&level, &players, at, Some(clear)), None);
    let enemy = giant(middle, Crowding::Block);
    assert_eq!(out(&level, &enemy, at, Some(clear)), None);
}
