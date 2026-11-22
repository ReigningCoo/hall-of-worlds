//! Fireplaces: their sizes, log baskets, fires and being faced.

use super::*;
use crate::bar::{BarCounter, BarKind, room_storage};
use crate::campfire::{LIGHT_SECONDS, MAX_KINDLING, burn_seconds, can_make_fire};
use crate::door::Side;
use crate::dungeon::{Cell, Level};
use crate::facing::Heading;
use crate::furniture::{Furniture, Held, Holder, Material, Piece, Stone, Wood, footprint};
use crate::id::Id;
use crate::item::{Item, ItemKind};
use crate::material::Material as WoodMaterial;
use crate::spot::Spot;
use crate::woodcutting::new_kindling;

fn oak() -> WoodMaterial {
    WoodMaterial::Oak
}

fn kindling(n: u128, count: u32) -> Item {
    new_kindling(oak(), count, Id::for_test(n), None)
}

fn piece(length: u8, at: (i64, i64)) -> Piece {
    let furniture = Furniture::Fireplace {
        fireplace: Fireplace { length },
    };
    Piece {
        at,
        size: footprint(furniture, (length, 1), Side::South),
        facing: Side::South,
        material: Material::Stone(Stone::Granite),
        furniture,
    }
}

#[test]
fn fireplaces_are_three_sizes_of_stone() {
    assert_eq!(Fireplace::all().len(), 3);
    for length in 1..=3 {
        let fireplace = piece(length, (0, 0));
        assert!(fireplace.is_well_made(), "{length}");
        let wooden = Piece {
            material: Material::Wood(Wood::Oak),
            ..fireplace
        };
        assert!(!wooden.is_well_made());
        let deep = Piece {
            size: (length, 2),
            ..fireplace
        };
        assert!(!deep.is_well_made());
    }
    assert!(!Fireplace { length: 4 }.is_well_made());
    assert_eq!(Fireplace { length: 3 }.name(), "Inglenook");
    assert!(Fireplace { length: 3 }.is_inglenook());
    assert_eq!(Fireplace { length: 3 }.fire_cell(), 1);
    assert_eq!(Fireplace { length: 2 }.fire_cell(), 0);
}

#[test]
fn a_fireplace_holds_kindling_and_no_drink() {
    let fireplace = Fireplace { length: 1 };
    let holder = Holder::of(piece(1, (0, 0)).furniture);
    assert_eq!(holder, Some(Holder::Fire(fireplace)));
    assert!(!Holder::Fire(fireplace).is_drink_storage());
    assert_eq!(Holder::Fire(fireplace).name(), "Fireplace");
    let mut held = Held::new(Holder::Fire(fireplace));
    let pan = Item::new(Id::for_test(1), ItemKind::Pan);
    assert!(held.put(pan).is_err());
    assert!(
        held.put(kindling(2, KINDLING_PER_PLACE + 1)).is_err(),
        "too many"
    );
    assert_eq!(held.put(kindling(3, 4)), Ok(None));
    assert_eq!(held.put(kindling(4, 1)), Ok(None));
    assert!(held.put(kindling(5, 1)).is_err(), "two places");
    assert!(held.take(Id::for_test(3)).is_some());
    assert_eq!(held.put(kindling(5, 1)), Ok(None));
    let Held::Fire(fireside) = &held else {
        panic!("not a fireside");
    };
    let first = fireside.places()[0].as_ref().map(|item| item.id);
    assert_eq!(first, Some(Id::for_test(5)));
    assert_eq!(held.items().count(), 2);
    assert!(!held.replace(kindling(5, 1)));
}

#[test]
fn a_fireplace_burns_as_a_hearth() {
    let mut fireside = Fireside::new(Fireplace { length: 2 });
    let held = Held::Fire(fireside.clone());
    assert!(!fireside.is_lit(0) && !held.is_lit(0));
    assert_eq!(fireside.burn(10, oak()), Some(1));
    assert!(fireside.is_lit(10) && Held::Fire(fireside.clone()).is_lit(10));
    for left in 2..=MAX_KINDLING {
        assert_eq!(fireside.burn(11, oak()), Some(left));
    }
    assert_eq!(fireside.burn(12, oak()), None, "full");
    let out = 10 + u64::from(burn_seconds(oak())) * MAX_KINDLING as u64 + LIGHT_SECONDS;
    assert!(!fireside.is_lit(out));
    assert_eq!(fireside.burn(out, oak()), Some(1), "lit again");
    fireside.always_lit = true;
    assert!(fireside.burn(out, oak()).is_none());
    assert!(fireside.is_lit(u64::MAX));
}

#[test]
fn a_fireplace_faced_can_be_lit() {
    let mut level = Level::new(0, 0, 8, 8);
    for x in 0..8 {
        for y in 0..8 {
            level.set(x, y, if y == 0 { Cell::Wall } else { Cell::Floor });
        }
    }
    level.place_furniture(piece(3, (2, 1)));
    let north = Heading::NORTH;
    let faced = fireplace_faced(&level, Spot::middle_of((3, 2)), north);
    assert_eq!(faced.map(|piece| piece.at), Some((2, 1)));
    assert!(can_make_fire(&level, Spot::middle_of((3, 2)), north));
    assert!(fireplace_faced(&level, Spot::middle_of((6, 4)), north).is_none());
    // Never drink storage for a bar in its room.
    let bar = Piece {
        at: (2, 4),
        size: (2, 1),
        facing: Side::South,
        material: Material::Wood(Wood::Oak),
        furniture: Furniture::BarCounter {
            counter: BarCounter {
                kind: BarKind::Single,
                length: 2,
            },
        },
    };
    level.place_furniture(bar);
    let storage = room_storage(&level, &bar);
    assert!(storage.iter().all(|piece| piece.at != (2, 1)));
}
