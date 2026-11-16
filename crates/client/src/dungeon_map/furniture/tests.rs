use bevy::mesh::VertexAttributeValues;
use bevy::prelude::*;
use shared::door::Side;
use shared::dungeon::{Cell, Level, Sight};
use shared::furniture::{Furniture, Material, Piece, Stone, TableStyle, Wood};
use std::collections::{BTreeMap, BTreeSet};

use super::super::CELL_SIZE;
use super::super::mesh::{LightGrid, Surface, View, build_posed};
use super::super::theme::Shape;
use super::frame::Frame;
use super::{TABLE_TOP, tables};

const STYLES: [TableStyle; 6] = [
    TableStyle::Trestle,
    TableStyle::Joined,
    TableStyle::PlankLegs,
    TableStyle::Pedestal,
    TableStyle::StoneSlab,
    TableStyle::StoneBlock,
];

fn table(style: TableStyle, size: (u8, u8), facing: Side) -> Piece {
    let material = if TableStyle::STONE.contains(&style) {
        Material::Stone(Stone::Granite)
    } else {
        Material::Wood(Wood::Oak)
    };
    Piece {
        at: (2, 3),
        size,
        facing,
        material,
        furniture: Furniture::Table { style },
    }
}

/// Every style, every size it fits, stays inside its cells and under its top,
/// and reaches the top.
#[test]
fn tables_stay_inside_their_cells() {
    for style in STYLES {
        for size in [(1, 1), (2, 1), (1, 2), (3, 2), (2, 3), (2, 2)] {
            if !style.fits(size) {
                continue;
            }
            let facing = if size.0 >= size.1 {
                Side::East
            } else {
                Side::South
            };
            let piece = table(style, size, facing);
            let (length, width) = Frame::of(&piece).size(&piece);
            let parts = tables::parts(&piece).expect("a table has parts");
            let mut highest = 0.0f32;
            for part in parts {
                let top = part.high + Vec3::new(part.lean.x, 0.0, part.lean.y);
                for point in [part.low, part.high, top] {
                    assert!(
                        (0.0..=length).contains(&point.x)
                            && (0.0..=width).contains(&point.z)
                            && (0.0..=TABLE_TOP).contains(&point.y),
                        "{style:?} {size:?}: {point}"
                    );
                }
                highest = highest.max(part.high.y);
            }
            assert_eq!(highest, TABLE_TOP, "{style:?} {size:?}");
        }
    }
}

/// A table in a lit room is drawn in the furniture mesh, within its cells,
/// with a black outline; one out of sight is remembered.
#[test]
fn a_table_is_drawn_in_its_cells() {
    let mut level = Level::new(0, 0, 8, 8);
    for y in 0..8 {
        for x in 0..8 {
            level.set(x, y, Cell::Floor);
        }
    }
    let piece = table(TableStyle::Trestle, (3, 2), Side::East);
    level.place_furniture(piece);
    let mut sight = Sight::new(&level);
    sight.add_view(&level, 6, 6, None);
    let views = vec![View {
        cell: (6, 6),
        sight: sight.clone(),
    }];
    let meshes = build_posed(
        &level,
        (&views, &sight, &sight, &Sight::new(&level)),
        &LightGrid::new(&level),
        &[],
        (&[], &BTreeSet::new(), &[], &[], false),
        (Vec3::new(0.0, 600.0, 2000.0), Shape::Built),
        None,
        &BTreeMap::new(),
    );
    assert!(meshes.has_solid[Surface::Furniture as usize]);
    assert!(meshes.outlined.black.is_some(), "an outline");
    let mesh = &meshes.solid[Surface::Furniture as usize];
    let Some(VertexAttributeValues::Float32x3(positions)) =
        mesh.attribute(Mesh::ATTRIBUTE_POSITION)
    else {
        panic!("positions");
    };
    let (west, north) = (1.5 * CELL_SIZE, 2.5 * CELL_SIZE);
    for &[x, y, z] in positions {
        assert!((west..=west + 3.0 * CELL_SIZE).contains(&x), "{x}");
        assert!((north..=north + 2.0 * CELL_SIZE).contains(&z), "{z}");
        assert!((0.0..=TABLE_TOP).contains(&y), "{y}");
    }
}
