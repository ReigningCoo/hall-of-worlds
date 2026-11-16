use super::super::{
    BenchStyle, ChairStyle, Furniture, Material, Piece, Stone, TableStyle, Wood, footprint,
};
use crate::door::Side;

fn piece(size: (u8, u8), facing: Side, material: Material, furniture: Furniture) -> Piece {
    Piece {
        at: (0, 0),
        size,
        facing,
        material,
        furniture,
    }
}

const OAK: Material = Material::Wood(Wood::Oak);
const SLATE: Material = Material::Stone(Stone::Slate);

#[test]
fn tables_run_the_way_they_face_and_seats_across_it() {
    let table = Furniture::Table {
        style: TableStyle::Joined,
    };
    assert_eq!(footprint(table, (3, 2), Side::East), (3, 2));
    assert_eq!(footprint(table, (3, 2), Side::South), (2, 3));
    let bench = Furniture::Bench {
        style: BenchStyle::LongStool,
    };
    assert_eq!(footprint(bench, (3, 1), Side::North), (3, 1));
    assert_eq!(footprint(bench, (3, 1), Side::West), (1, 3));
}

#[test]
fn only_pieces_furnishing_could_make_are_well_made() {
    let joined = Furniture::Table {
        style: TableStyle::Joined,
    };
    assert!(piece((3, 2), Side::East, OAK, joined).is_well_made());
    assert!(piece((2, 3), Side::South, OAK, joined).is_well_made());
    // Facing across its length, west, square and south, too big, of stone.
    assert!(!piece((2, 3), Side::East, OAK, joined).is_well_made());
    assert!(!piece((3, 2), Side::West, OAK, joined).is_well_made());
    assert!(!piece((2, 2), Side::South, OAK, joined).is_well_made());
    assert!(!piece((3, 3), Side::East, OAK, joined).is_well_made());
    assert!(!piece((3, 2), Side::East, SLATE, joined).is_well_made());
    let pedestal = Furniture::Table {
        style: TableStyle::Pedestal,
    };
    assert!(piece((2, 1), Side::East, OAK, pedestal).is_well_made());
    assert!(!piece((2, 2), Side::East, OAK, pedestal).is_well_made());
    let stone_chair = Furniture::Chair {
        style: ChairStyle::StoneBlock,
        arms: false,
    };
    assert!(piece((1, 1), Side::West, SLATE, stone_chair).is_well_made());
    let armed = Furniture::Chair {
        style: ChairStyle::StoneBlock,
        arms: true,
    };
    assert!(!piece((1, 1), Side::West, SLATE, armed).is_well_made());
    let bench = Furniture::Bench {
        style: BenchStyle::SplayedPlank,
    };
    assert!(piece((1, 3), Side::East, OAK, bench).is_well_made());
    assert!(!piece((3, 1), Side::East, OAK, bench).is_well_made());
    assert!(!piece((2, 2), Side::North, OAK, bench).is_well_made());
}

#[test]
fn every_style_is_listed_once() {
    assert!(
        TableStyle::WOOD
            .iter()
            .chain(&TableStyle::STONE)
            .eq(TableStyle::ALL.iter())
    );
    let stone = |furniture: Furniture| furniture.is_stone();
    let chairs = ChairStyle::ALL.map(|style| Furniture::Chair { style, arms: false });
    assert_eq!(chairs.into_iter().filter(|&c| stone(c)).count(), 1);
    let benches = BenchStyle::ALL.map(|style| Furniture::Bench { style });
    assert_eq!(benches.into_iter().filter(|&b| stone(b)).count(), 1);
    assert_eq!(Material::ALL.iter().filter(|m| m.is_stone()).count(), 5);
}
