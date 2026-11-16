use shared::dungeon::{Cell, Level, Sight};

use super::*;

/// A 5 × 1 floor below: wall, floor, floor, hole, ground; the floor above is
/// air over all but the first floor cell.
fn floors() -> (Level, Level) {
    let mut below = Level::new(0, 0, 5, 1);
    for (x, cell) in [
        Cell::Wall,
        Cell::Floor,
        Cell::Floor,
        Cell::Hole,
        Cell::Ground,
    ]
    .into_iter()
    .enumerate()
    {
        below.set(x as i64, 0, cell);
    }
    let mut over = Level::new(0, 0, 5, 1);
    for x in 0..5 {
        over.set(x, 0, if x == 1 { Cell::Floor } else { Cell::Air });
    }
    (below, over)
}

fn everything(level: &Level) -> Sight {
    let mut known = Sight::new(level);
    for x in 0..5 {
        known.insert(x, 0);
    }
    known
}

#[test]
fn only_rooms_and_unseen_walls_under_air_are_roofed() {
    let (below, over) = floors();
    let drawn = Sight::new(&below);
    // The wall isn't drawn, so the roof covers it; the floor under the floor
    // above, the hole and the ground get none.
    let known = everything(&below);
    assert_eq!(
        roofed((&below, &known), &over, &drawn),
        vec![(0, 0), (2, 0)]
    );
}

#[test]
fn nothing_unknown_is_roofed() {
    let (below, over) = floors();
    let mut known = Sight::new(&below);
    known.insert(2, 0);
    let drawn = Sight::new(&below);
    assert_eq!(roofed((&below, &known), &over, &drawn), vec![(2, 0)]);
    assert_eq!(roofing(&over, (&below, &known)), vec![(2, 0)]);
}

#[test]
fn a_drawn_wall_keeps_its_own_top() {
    let (below, over) = floors();
    let mut drawn = Sight::new(&below);
    drawn.insert(0, 0);
    let known = everything(&below);
    assert_eq!(roofed((&below, &known), &over, &drawn), vec![(2, 0)]);
}

#[test]
fn a_floor_above_roofs_what_is_under_its_air() {
    let (below, over) = floors();
    // Over the wall and the floor beside it; not over the hole or the ground,
    // nor where the floor above has floor of its own.
    let known = everything(&below);
    assert_eq!(roofing(&over, (&below, &known)), vec![(0, 0), (2, 0)]);
}
