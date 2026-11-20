use bevy::prelude::*;
use shared::crafting_bench::Craft;
use shared::door::Side;
use shared::furniture::{Furniture, Material, Piece, Wood};
use shared::item::ItemKind;

use super::super::frame::Frame;
use super::{SHELF_TOP, TABLE_TOP, parts, tool_spot};
use crate::dungeon_map::CELL_SIZE;

fn bench(facing: Side) -> Piece {
    let size = match facing {
        Side::North | Side::South => (2, 1),
        Side::East | Side::West => (1, 2),
    };
    Piece {
        at: (2, 3),
        size,
        facing,
        material: Material::Wood(Wood::Oak),
        furniture: Furniture::CraftingBench {
            craft: Craft::Alchemy,
        },
    }
}

const SIDES: [Side; 4] = [Side::North, Side::East, Side::South, Side::West];

/// Every way round, it stays in its cells, reaches the shelf's height, and the
/// shelf is on the side away from its front.
#[test]
fn a_bench_stays_in_its_cells_with_its_shelf_at_the_back() {
    for facing in SIDES {
        let piece = bench(facing);
        let frame = Frame::of(&piece);
        let (length, width) = frame.size(&piece);
        let parts = parts(&piece).expect("a bench has parts");
        let mut highest = 0.0f32;
        for part in &parts {
            for point in [part.low, part.high] {
                assert!(
                    (0.0..=length).contains(&point.x) && (0.0..=width).contains(&point.z),
                    "{facing:?}: {point}"
                );
            }
            highest = highest.max(part.high.y);
        }
        assert_eq!(highest, SHELF_TOP, "{facing:?}");
        // The shelf's middle, in the world, is behind the bench's middle.
        let shelf = parts.iter().find(|p| p.high.y == SHELF_TOP).unwrap();
        let shelf = frame.place((shelf.low + shelf.high) / 2.0);
        let middle = frame.place(Vec3::new(length, 0.0, width) / 2.0);
        let (dx, dy) = facing.beside((0, 0));
        let ahead = Vec3::new(dx as f32, 0.0, dy as f32);
        assert!((shelf - middle).dot(ahead) < -CELL_SIZE * 0.2, "{facing:?}");
    }
}

/// Every apparatus has its own place on it: tall ones on the shelf, the rest
/// on the top in front, all within its cells.
#[test]
fn every_tool_has_its_own_place() {
    for facing in SIDES {
        let piece = bench(facing);
        let mut places: Vec<Vec3> = Vec::new();
        for kind in Craft::Alchemy.tools() {
            let (at, room) = tool_spot(&piece, kind).expect("a place");
            let tall = matches!(
                kind,
                ItemKind::Retort | ItemKind::Alembic | ItemKind::Pelican
            );
            assert_eq!(at.y, if tall { SHELF_TOP } else { TABLE_TOP }, "{kind:?}");
            assert!(room > CELL_SIZE * 0.2, "{kind:?}");
            let covers = piece.cells().any(|(x, y)| {
                (at.x / CELL_SIZE - x as f32).abs() <= 0.5
                    && (at.z / CELL_SIZE - y as f32).abs() <= 0.5
            });
            assert!(covers, "{facing:?} {kind:?} at {at}");
            assert!(
                places.iter().all(|other| other.distance(at) > room * 0.9),
                "{kind:?}"
            );
            places.push(at);
        }
        assert!(tool_spot(&piece, ItemKind::Cauldron).is_none());
    }
}
