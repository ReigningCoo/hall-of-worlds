use bevy::prelude::*;
use shared::door::Side;
use shared::fireplace::Fireplace;
use shared::furniture::{Furniture, Material, Piece, Stone, footprint};

use super::super::frame::Frame;
use super::{BREAST_TOP, basket_spot, fire_spot, parts};
use crate::dungeon_map::CELL_SIZE;

const SIDES: [Side; 4] = [Side::North, Side::East, Side::South, Side::West];

fn fireplace(length: u8, facing: Side) -> Piece {
    let furniture = Furniture::Fireplace {
        fireplace: Fireplace { length },
    };
    Piece {
        at: (2, 3),
        size: footprint(furniture, (length, 1), facing),
        facing,
        material: Material::Stone(Stone::Granite),
        furniture,
    }
}

/// Whether a point in the world (on flat floor) lies over one of its cells.
fn over(piece: &Piece, at: Vec3) -> bool {
    piece.cells().any(|(x, y)| {
        (at.x / CELL_SIZE - x as f32).abs() <= 0.5 && (at.z / CELL_SIZE - y as f32).abs() <= 0.5
    })
}

/// Every fireplace, every way round, stays in its cells, its breast rising
/// to the top of the wall and no higher.
#[test]
fn a_fireplace_stays_in_its_cells() {
    for length in 1..=3 {
        for facing in SIDES {
            let piece = fireplace(length, facing);
            assert!(piece.is_well_made(), "{length} {facing:?}");
            let (long, wide) = Frame::of(&piece).size(&piece);
            let parts = parts(&piece).expect("a fireplace has parts");
            let mut highest = 0.0f32;
            for part in &parts {
                for point in [part.low, part.high] {
                    assert!(
                        (0.0..=long).contains(&point.x) && (0.0..=wide).contains(&point.z),
                        "{length} {facing:?}: {point}"
                    );
                }
                highest = highest.max(part.high.y);
            }
            assert_eq!(highest, BREAST_TOP, "{length} {facing:?}");
        }
    }
}

/// The fire burns in its cell (an inglenook's middle one), and each basket
/// place lies over the piece, apart from the others.
#[test]
fn the_fire_and_the_logs_have_their_places() {
    for length in 1..=3 {
        for facing in SIDES {
            let piece = fireplace(length, facing);
            let Furniture::Fireplace { fireplace } = piece.furniture else {
                unreachable!();
            };
            let fire = fire_spot(&piece, fireplace);
            let cell = piece.cells().nth(usize::from(fireplace.fire_cell()));
            let (x, y) = cell.expect("its fire cell");
            let middle = Vec3::new(x as f32, 0.0, y as f32) * CELL_SIZE;
            assert!(fire.with_y(0.0).distance(middle) < CELL_SIZE * 0.5);
            let spots: Vec<Vec3> = (0..fireplace.basket_places())
                .map(|i| basket_spot(&piece, i).expect("a place").0)
                .collect();
            assert!(spots.iter().all(|&spot| over(&piece, spot)));
            assert!(
                spots
                    .windows(2)
                    .all(|w| w[0].distance(w[1]) > CELL_SIZE * 0.1)
            );
            assert!(basket_spot(&piece, fireplace.basket_places()).is_none());
        }
    }
}
