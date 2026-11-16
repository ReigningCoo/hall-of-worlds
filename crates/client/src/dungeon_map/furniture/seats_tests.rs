use bevy::prelude::*;
use shared::door::Side;
use shared::dungeon::{Cell, Level};
use shared::furniture::{
    BenchStyle, ChairStyle, Furniture, Material, Piece, Stone, TableStyle, Wood, footprint,
};

use super::super::CELL_SIZE;
use super::frame::Frame;
use super::seats::{self, OVERHANG};

const SIDES: [Side; 4] = [Side::North, Side::East, Side::South, Side::West];

fn seat(furniture: Furniture, long: u8, facing: Side) -> Piece {
    let material = if furniture.is_stone() {
        Material::Stone(Stone::Slate)
    } else {
        Material::Wood(Wood::Ash)
    };
    Piece {
        at: (2, 3),
        size: footprint(furniture, (long, 1), facing),
        facing,
        material,
        furniture,
    }
}

fn every_seat() -> Vec<(Furniture, u8)> {
    let mut all = Vec::new();
    for style in ChairStyle::ALL {
        for arms in [false, true] {
            if !(arms && style == ChairStyle::StoneBlock) {
                all.push((Furniture::Chair { style, arms }, 1));
            }
        }
    }
    for style in BenchStyle::ALL {
        for long in 1..=3 {
            all.push((Furniture::Bench { style }, long));
        }
    }
    all
}

/// Every seat, every way round, stays inside its cells (reaching under a
/// table only forwards, by `OVERHANG`) and below a table's top.
#[test]
fn seats_stay_inside_their_cells() {
    for (furniture, long) in every_seat() {
        for facing in SIDES {
            for under_table in [false, true] {
                let piece = seat(furniture, long, facing);
                let (depth, length) = Frame::of(&piece).size(&piece);
                let forwards = matches!(facing, Side::East | Side::South);
                let (low, high) = match (under_table, forwards) {
                    (false, _) => (0.0, depth),
                    (true, true) => (0.0, depth + OVERHANG),
                    (true, false) => (-OVERHANG, depth),
                };
                let parts = seats::parts(&piece, under_table).expect("a seat has parts");
                assert!(!parts.is_empty());
                for part in parts {
                    let top = part.high + Vec3::new(part.lean.x, 0.0, part.lean.y);
                    for point in [part.low, part.high, top] {
                        assert!(
                            (low - 0.001..=high + 0.001).contains(&point.x)
                                && (0.0..=length).contains(&point.z)
                                && (0.0..=CELL_SIZE * 0.6).contains(&point.y),
                            "{furniture:?} {long} {facing:?} {under_table}: {point}"
                        );
                    }
                }
            }
        }
    }
}

/// A chair's tall back stands behind its sitter: on the side away from the
/// way it faces.
#[test]
fn backs_are_behind() {
    let chair = Furniture::Chair {
        style: ChairStyle::LadderBack,
        arms: false,
    };
    for facing in SIDES {
        let piece = seat(chair, 1, facing);
        let parts = seats::parts(&piece, false).unwrap();
        let tallest = parts
            .iter()
            .max_by(|a, b| a.high.y.total_cmp(&b.high.y))
            .unwrap();
        let middle = (tallest.low.x + tallest.high.x) / 2.0;
        let behind_low = matches!(facing, Side::East | Side::South);
        assert_eq!(middle < CELL_SIZE / 2.0, behind_low, "{facing:?}");
    }
}

/// A seat faces a table when one is in front of it, not beside or behind.
#[test]
fn faces_a_table_in_front() {
    let mut level = Level::new(0, 0, 8, 8);
    for y in 0..8 {
        for x in 0..8 {
            level.set(x, y, Cell::Floor);
        }
    }
    level.place_furniture(Piece {
        at: (3, 3),
        size: (2, 1),
        facing: Side::East,
        material: Material::Wood(Wood::Oak),
        furniture: Furniture::Table {
            style: TableStyle::Joined,
        },
    });
    let bench = Furniture::Bench {
        style: BenchStyle::LongStool,
    };
    let at = |at: (i64, i64), facing| Piece {
        at,
        ..seat(bench, 2, facing)
    };
    assert!(seats::faces_table(&level, &at((3, 2), Side::South)));
    assert!(seats::faces_table(&level, &at((3, 4), Side::North)));
    assert!(!seats::faces_table(&level, &at((3, 2), Side::North)));
    assert!(!seats::faces_table(&level, &at((3, 5), Side::North)));
}
