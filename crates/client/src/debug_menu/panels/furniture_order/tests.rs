use shared::door::Side;
use shared::furniture::{ChairStyle, Furniture, Material, Stone, TableStyle, Wood};

use super::{FurnitureOrder, Kind};

#[test]
fn clicking_the_same_cell_cycles_through_every_variation_and_wraps() {
    let mut order = FurnitureOrder {
        material: Some(Material::Wood(Wood::Ash)),
        ..FurnitureOrder::default()
    };
    // A 2 × 3 table of wood: every wooden style but the pedestal.
    let styles: Vec<Furniture> = (0..4).map(|_| order.click((4, 5), 0).furniture).collect();
    let expected = [
        TableStyle::Trestle,
        TableStyle::Joined,
        TableStyle::PlankLegs,
    ];
    let expected = expected.map(|style| Furniture::Table { style });
    assert_eq!(styles, [&expected[..], &expected[..1]].concat());
    // A click elsewhere keeps the variation; a change of choices too.
    assert_eq!(order.click((9, 9), 0).furniture, expected[0]);
    assert_eq!(order.click((9, 9), 0).furniture, expected[1]);
    order.facing = Side::South;
    let piece = order.click((9, 9), 0);
    assert_eq!(piece.furniture, expected[1]);
    assert_eq!((piece.size, piece.facing), ((2, 3), Side::South));
    assert!(piece.is_well_made());
}

#[test]
fn chairs_come_without_then_with_arms_and_stone_ones_without() {
    let mut order = FurnitureOrder {
        kind: Kind::Chair,
        size: 0,
        facing: Side::West,
        material: None,
        ..FurnitureOrder::default()
    };
    let pieces: Vec<_> = (0..11).map(|n| order.click((0, 0), n)).collect();
    assert!(pieces.iter().all(|piece| piece.is_well_made()));
    let stool = |arms| Furniture::Chair {
        style: ChairStyle::Stool,
        arms,
    };
    assert_eq!(pieces[0].furniture, stool(false));
    assert_eq!(pieces[1].furniture, stool(true));
    let stone = Furniture::Chair {
        style: ChairStyle::StoneBlock,
        arms: false,
    };
    assert_eq!(pieces[10].furniture, stone);
    assert!(pieces[10].material.is_stone());
    // Wrapped round.
    assert_eq!(order.click((0, 0), 0).furniture, stool(false));
}

#[test]
fn every_kind_size_facing_and_material_makes_well_made_pieces() {
    let materials = [
        None,
        Some(Material::Wood(Wood::Oak)),
        Some(Material::Stone(Stone::Marble)),
    ];
    for kind in Kind::ALL {
        for size in 0..kind.sizes().len() {
            for facing in [Side::North, Side::East, Side::South, Side::West] {
                for material in materials {
                    let mut order = FurnitureOrder {
                        kind,
                        size,
                        facing,
                        material,
                        ..FurnitureOrder::default()
                    };
                    for roll in 0..12 {
                        let piece = order.click((1, 1), roll);
                        assert!(piece.is_well_made(), "{piece:?}");
                    }
                }
            }
        }
    }
}

#[test]
fn cask_racks_cycle_through_the_sizes_made_at_their_length_and_are_wooden() {
    let mut order = FurnitureOrder {
        kind: Kind::CaskRack,
        size: 1,
        facing: Side::South,
        material: Some(Material::Stone(Stone::Granite)),
        ..FurnitureOrder::default()
    };
    let pieces: Vec<_> = (0..9).map(|n| order.click((3, 3), n)).collect();
    let racks: Vec<_> = (pieces.iter())
        .map(|piece| match piece.furniture {
            Furniture::CaskRack { rack } => rack.size,
            other => panic!("not a rack: {other:?}"),
        })
        .collect();
    // Two cells long: every size, the Tun's stand too, then round again.
    assert_eq!(racks[..8], shared::cask::CaskSize::ALL);
    assert_eq!(racks[8], racks[0]);
    for piece in &pieces {
        assert!(piece.is_well_made(), "{piece:?}");
        assert_eq!(piece.size, (2, 1));
    }
}
