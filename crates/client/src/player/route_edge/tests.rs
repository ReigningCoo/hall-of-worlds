use shared::clearance::{Crowding, Other};
use shared::creature::Size;
use shared::spot::Spot;

use super::*;

fn m(x: i64, y: i64) -> Spot {
    Spot::middle_of((x, y))
}

fn giant(at: Spot) -> [Other; 1] {
    [Other {
        at,
        size: Size::Giant,
        crowding: Crowding::Squeeze,
    }]
}

fn stopped(mut spots: Vec<Spot>, others: &[Other]) -> Vec<Spot> {
    stop_at_edge(&mut spots, Size::Medium, others, |_, _| true);
    spots
}

#[test]
fn a_route_into_someone_ends_at_their_edge_on_the_way_in() {
    let them = giant(m(0, 0));
    let spots = stopped(vec![m(-3, 0), m(0, 0)], &them);
    assert_eq!(spots.len(), 2);
    let end = spots[1];
    assert_eq!(end.y, 0);
    // Giant (240) and Medium (80) touch 320 apart: just outside, from the west.
    assert!(end.x < 0 && (320..=324).contains(&end.distance(m(0, 0))));
}

#[test]
fn earlier_points_inside_are_dropped() {
    let them = giant(m(0, 0));
    let spots = stopped(vec![m(-3, -2), m(-3, 0), m(-1, 0), m(0, 0)], &them);
    assert_eq!(spots[..2], [m(-3, -2), m(-3, 0)]);
    assert_eq!(spots.len(), 3);
    assert!(spots[2].x < -320 + 3 && spots[2].x >= -324);
}

#[test]
fn routes_left_alone() {
    // Ending clear of them.
    let beside = giant(m(0, 3));
    let route = vec![m(-3, 0), m(0, 0)];
    assert_eq!(stopped(route.clone(), &beside), route);
    // Starting inside them: stepping out at the end sees to it.
    let near = giant(m(-3, 0));
    let from_inside = vec![m(-3, 0), m(-3, 1)];
    assert_eq!(stopped(from_inside.clone(), &near), from_inside);
    // Players are walked through.
    let player = [Other {
        crowding: Crowding::Pass,
        ..giant(m(0, 0))[0]
    }];
    assert_eq!(stopped(route.clone(), &player), route);
    // A stop the walk can't reach.
    let mut spots = route.clone();
    stop_at_edge(&mut spots, Size::Medium, &giant(m(0, 0)), |_, _| false);
    assert_eq!(spots, route);
}
