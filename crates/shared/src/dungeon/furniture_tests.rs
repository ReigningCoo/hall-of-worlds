use super::{Cell, Level};
use crate::bottle_rack::{BottleRack, BottleRackKind};
use crate::door::Side;
use crate::furniture::{Furniture, Material, Piece, TableStyle, Wood};

fn level() -> Level {
    let mut level = Level::new(0, 0, 8, 8);
    for y in 0..8 {
        for x in 0..8 {
            level.set(x, y, Cell::Floor);
        }
    }
    level
}

fn table(at: (i64, i64), size: (u8, u8)) -> Piece {
    Piece {
        at,
        size,
        facing: Side::East,
        material: Material::Wood(Wood::Oak),
        furniture: Furniture::Table {
            style: TableStyle::Trestle,
        },
    }
}

fn spirit_rack(at: (i64, i64), length: u8) -> Piece {
    let rack = BottleRack {
        kind: BottleRackKind::Spirit,
        length,
    };
    Piece {
        at,
        size: (length, 1),
        facing: Side::South,
        material: Material::Wood(Wood::Oak),
        furniture: Furniture::BottleRack { rack },
    }
}

#[test]
fn a_spirit_rack_stands_only_on_one_table_and_on_top_of_it() {
    let mut level = level();
    level.place_furniture(spirit_rack((1, 1), 2));
    assert!(
        level.furniture_on_top_at(1, 1).is_none(),
        "nothing to stand on"
    );
    level.place_furniture(table((1, 1), (3, 2)));
    level.place_furniture(spirit_rack((1, 1), 2));
    assert!(level.furniture_at(1, 1).unwrap().furniture.is_table());
    assert_eq!(level.furniture_on_top_at(2, 1).unwrap().at, (1, 1));
    assert_eq!(level.furniture_on_top().count(), 1);
    // Moves still cost what the table's do.
    assert_eq!(level.furniture_cost_per_mille(1, 1), 3000);
    // Not hanging over its edge.
    assert!(!level.can_stand_on_furniture(&spirit_rack((3, 1), 2)));
}

#[test]
fn a_rack_goes_with_the_table_it_stands_on() {
    let mut level = level();
    level.place_furniture(table((1, 1), (3, 1)));
    level.place_furniture(spirit_rack((1, 1), 1));
    level.place_furniture(spirit_rack((2, 1), 2));
    // The second's first cell overlapped nothing; both stand.
    assert_eq!(level.furniture_on_top().count(), 2);
    level.place_furniture(spirit_rack((1, 1), 2));
    assert_eq!(level.furniture_on_top().count(), 1, "replaced both");
    level.place_furniture(table((3, 0), (1, 1)).moved((0, 1)));
    assert!(level.furniture_on_top_at(1, 1).is_none(), "its table went");
}
