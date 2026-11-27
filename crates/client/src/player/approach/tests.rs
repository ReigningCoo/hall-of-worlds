use shared::combat::{Body, reach_between};
use shared::creature::Size;
use shared::dungeon::{Cell, Level};
use shared::id::Id;
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

fn body(x: i64, y: i64) -> Body {
    Body {
        at: Spot::new(x, y),
        height: 0,
        size: Size::Medium,
    }
}

/// Bare hands' reach between two Medium creatures.
fn bare_hands() -> u64 {
    reach_between(Size::Medium, 0, Size::Medium)
}

fn going(routed_to: Option<Spot>, apart: u64) -> Approaching {
    Approaching {
        target: Id::for_test(1),
        reach: bare_hands(),
        routed_to,
        apart,
    }
}

#[test]
fn in_reach_it_stops_whatever_the_route() {
    let level = room(5);
    let me = body(0, 0);
    let them = body(300, 0);
    for route_left in [false, true] {
        let next = next(&level, &going(None, u64::MAX), &me, Some(&them), route_left);
        assert_eq!(next, Next::Reached);
    }
}

#[test]
fn a_wall_between_is_not_in_reach() {
    let mut level = room(5);
    level.set(1, 0, Cell::Wall);
    let routed = going(Some(Spot::new(480, 0)), 480);
    let next = next(&level, &routed, &body(0, 0), Some(&body(480, 0)), true);
    assert_eq!(next, Next::Walk);
}

#[test]
fn the_first_look_finds_a_route_to_them() {
    let level = room(5);
    let them = body(960, 240);
    let next = next(
        &level,
        &going(None, u64::MAX),
        &body(0, 0),
        Some(&them),
        false,
    );
    assert_eq!(next, Next::Route(them.at));
}

#[test]
fn it_routes_again_only_once_they_have_moved_half_a_cell() {
    let level = room(5);
    let routed = going(Some(Spot::new(960, 0)), 960);
    let me = body(0, 0);
    let a_little = body(960 + REROUTE as i64, 0);
    assert_eq!(
        next(&level, &routed, &me, Some(&a_little), true),
        Next::Walk
    );
    let further = body(960 + REROUTE as i64 + 1, 0);
    let next = next(&level, &routed, &me, Some(&further), true);
    assert_eq!(next, Next::Route(further.at));
}

#[test]
fn a_route_ended_out_of_reach_tries_again_only_if_it_came_nearer() {
    let level = room(5);
    let them = body(960, 0);
    let nearer = going(Some(them.at), 1200);
    let me = body(0, 0);
    assert_eq!(
        next(&level, &nearer, &me, Some(&them), false),
        Next::Route(them.at)
    );
    let no_nearer = going(Some(them.at), 960);
    assert_eq!(
        next(&level, &no_nearer, &me, Some(&them), false),
        Next::GiveUp
    );
}

#[test]
fn it_gives_up_once_they_are_gone() {
    let level = room(5);
    let next = next(&level, &going(None, u64::MAX), &body(0, 0), None, true);
    assert_eq!(next, Next::GiveUp);
}
