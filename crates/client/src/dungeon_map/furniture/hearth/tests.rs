use bevy::prelude::*;
use shared::crafting_bench::Craft;
use shared::door::Side;
use shared::furniture::{Furniture, Material, Piece, Stone, footprint};

use super::super::crafting_bench::tool_spot;
use super::super::frame::Frame;
use super::{HOOD_TOP, firebox, parts};
use crate::dungeon_map::CELL_SIZE;

const SIDES: [Side; 4] = [Side::North, Side::East, Side::South, Side::West];
const HEARTHS: [Craft; 3] = [Craft::Stove, Craft::Hearth, Craft::Range];

fn hearth(craft: Craft, facing: Side) -> Piece {
    let furniture = Furniture::CraftingBench { craft };
    Piece {
        at: (2, 3),
        size: footprint(furniture, (craft.length(), 1), facing),
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

/// Every hearth, every way round, stays in its cells and rises to its hood.
#[test]
fn a_hearth_stays_in_its_cells() {
    for craft in HEARTHS {
        for facing in SIDES {
            let piece = hearth(craft, facing);
            assert!(piece.is_well_made(), "{craft:?} {facing:?}");
            let (length, width) = Frame::of(&piece).size(&piece);
            let parts = parts(&piece).expect("a hearth has parts");
            let mut highest = 0.0f32;
            for part in &parts {
                for point in [part.low, part.high] {
                    assert!(
                        (0.0..=length).contains(&point.x) && (0.0..=width).contains(&point.z),
                        "{craft:?} {facing:?}: {point}"
                    );
                }
                highest = highest.max(part.high.y);
            }
            assert_eq!(highest, HOOD_TOP, "{craft:?} {facing:?}");
        }
    }
}

/// The firebox is in the hearth's first cell, its fire in front of the hood.
#[test]
fn the_fire_is_in_the_firebox() {
    for craft in HEARTHS {
        for facing in SIDES {
            let piece = hearth(craft, facing);
            let fire = firebox(&piece).expect("a firebox");
            assert!(over(&piece, fire), "{craft:?} {facing:?} at {fire}");
            let cell = Vec3::new(piece.at.0 as f32, 0.0, piece.at.1 as f32) * CELL_SIZE;
            assert!(
                fire.with_y(0.0).distance(cell) < CELL_SIZE * 0.5,
                "{facing:?}"
            );
        }
    }
}

/// Each kitchen tool has its own place on a Hearth or Range, on it; a Stove
/// holds none.
#[test]
fn every_kitchen_tool_has_its_own_place() {
    for craft in [Craft::Hearth, Craft::Range] {
        for facing in SIDES {
            let piece = hearth(craft, facing);
            let mut places: Vec<Vec3> = Vec::new();
            for kind in craft.tools() {
                let (at, room) = tool_spot(&piece, kind).expect("a place");
                assert!(over(&piece, at), "{craft:?} {facing:?} {kind:?} at {at}");
                assert!(room > CELL_SIZE * 0.15, "{kind:?}");
                assert!(
                    places
                        .iter()
                        .all(|other| other.distance(at) > CELL_SIZE * 0.2),
                    "{craft:?} {facing:?} {kind:?}"
                );
                places.push(at);
            }
        }
    }
    let stove = hearth(Craft::Stove, Side::South);
    assert!(tool_spot(&stove, shared::item::ItemKind::Pan).is_none());
}
