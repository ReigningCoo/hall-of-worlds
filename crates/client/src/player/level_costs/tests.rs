use bevy::math::DVec2;
use shared::clearance::{Crowding, Other};
use shared::creature::Size;
use shared::dungeon::{Cell, Level};
use shared::movement::Gait;
use shared::spot::Spot;

use super::*;
use crate::common::WildCellHeights;

fn floor() -> Level {
    let mut level = Level::new(-6, -2, 13, 5);
    for x in -5..=5 {
        for y in -1..=1 {
            level.set(x, y, Cell::Floor);
        }
    }
    level
}

#[test]
fn walking_is_paced_by_the_squeeze_it_is_charged() {
    let level = floor();
    let heights = WildCellHeights::default();
    let giant = [Other {
        at: Spot::middle_of((0, 0)),
        size: Size::Giant,
        crowding: Crowding::Squeeze,
    }];
    let costs = LevelCosts {
        level: &level,
        heights: &heights,
        relief: None,
        gait: Gait::default(),
        sneaking: false,
        speed: shared::energy::NORMAL_SPEED,
        size: Size::Medium,
        others: &giant,
    };
    let east = DVec2::new(1.0, 0.0);
    let west = DVec2::new(-1.0, 0.0);
    // Far from them: as on flat ground.
    let far = Spot::middle_of((-4, 0));
    assert!((costs.pace(far, far, east) - 1.0).abs() < 0.01);
    // Pushing into them: as slow as the move will be charged.
    let inside = Spot::middle_of((0, 0)).offset(-150, 0);
    let pace = costs.pace(inside, inside, east);
    let charged = costs.move_time(inside, inside.offset(40, 0)) as f64
        / costs.move_time(far, far.offset(40, 0)) as f64;
    assert!(
        pace > 1.5 && (pace - charged).abs() < 0.2,
        "{pace} {charged}"
    );
    // Walking out of them: not slowed.
    assert!((costs.pace(inside, inside, west) - 1.0).abs() < 0.01);
    // A stretch begun outside them is charged as squeezed to its end.
    assert!(costs.pace(inside.offset(-200, 0), inside, east) > 1.5);
}
