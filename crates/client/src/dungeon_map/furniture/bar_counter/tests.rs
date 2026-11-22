use shared::bar::{BarCounter, BarKind};
use shared::door::Side;
use shared::dungeon::{Cell, Level};
use shared::furniture::{Furniture, Material, Piece, Wood};

use super::*;

fn piece(at: (i64, i64), counter: BarCounter, facing: Side) -> Piece {
    let furniture = Furniture::BarCounter { counter };
    let size = shared::furniture::footprint(furniture, (counter.length, 1), facing);
    Piece {
        at,
        size,
        facing,
        material: Material::Wood(Wood::Oak),
        furniture,
    }
}

fn level() -> Level {
    let mut level = Level::new(0, 0, 12, 12);
    for y in 0..12 {
        for x in 0..12 {
            level.set(x, y, Cell::Floor);
        }
    }
    level
}

#[test]
fn every_vessel_stands_on_its_counter_apart_from_the_others() {
    for counter in BarCounter::all() {
        for facing in [Side::North, Side::East, Side::South, Side::West] {
            let piece = piece((4, 6), counter, facing);
            assert!(piece.is_well_made(), "{counter:?}");
            let spots: Vec<Vec3> = (0..counter.capacity())
                .map(|index| bar_vessel_spot(&piece, index).unwrap())
                .collect();
            let top = bar_height(&piece).unwrap();
            let (x0, z0) = ((3.5) * CELL_SIZE, 5.5 * CELL_SIZE);
            let (w, h) = (f32::from(piece.size.0), f32::from(piece.size.1));
            for (i, a) in spots.iter().enumerate() {
                assert!(a.y < top, "{counter:?} {facing:?}");
                assert!(
                    (x0..x0 + w * CELL_SIZE).contains(&a.x),
                    "{counter:?} {facing:?}"
                );
                assert!(
                    (z0..z0 + h * CELL_SIZE).contains(&a.z),
                    "{counter:?} {facing:?}"
                );
                for b in &spots[i + 1..] {
                    assert!(a.distance(*b) > CELL_SIZE * 0.15, "{counter:?} {facing:?}");
                }
            }
            assert!(!parts(&level(), &piece).unwrap().is_empty());
        }
    }
}

#[test]
fn counters_in_a_line_join_into_one_run() {
    let single = BarCounter {
        kind: BarKind::Single,
        length: 2,
    };
    let flap = BarCounter {
        kind: BarKind::Flap,
        length: 1,
    };
    let mut level = level();
    let first = piece((2, 4), single, Side::South);
    let second = piece((4, 4), single, Side::South);
    let gap = piece((6, 4), flap, Side::South);
    let turned = piece((0, 4), single, Side::North);
    for p in [first, second, gap, turned] {
        level.place_furniture(p);
    }
    assert!(joins(&level, &first, true) && joins(&level, &second, false));
    // The flap and a counter facing the other way end the run.
    assert!(!joins(&level, &second, true) && !joins(&level, &first, false));
    // A joined end has no end panel: fewer parts.
    let alone = parts(&self::level(), &first).unwrap().len();
    assert!(parts(&level, &second).unwrap().len() < alone);
}

#[test]
fn a_double_counters_vessels_stand_towards_its_drinkers() {
    let double = BarCounter {
        kind: BarKind::Double,
        length: 1,
    };
    let south = piece((4, 6), double, Side::South);
    let spot = bar_vessel_spot(&south, 0).unwrap();
    // Cell (4, 6)'s middle is at z = 6 cells; the drinkers are south (+z).
    assert!(spot.z > 6.0 * CELL_SIZE, "{spot:?}");
}
