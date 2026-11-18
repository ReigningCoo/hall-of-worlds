use shared::site::FieldLook;
use shared::site::Rect;

use super::super::town::PLOUGHED_TILE;
use super::*;

fn field(livestock: bool, variety: u32) -> FieldLook {
    FieldLook {
        rect: Rect {
            x: 0,
            y: 0,
            width: 8,
            height: 8,
        },
        livestock,
        variety,
        orchard: false,
    }
}

/// A field of crops is ploughed, its plants standing on it as they grow;
/// pasture stays short.
#[test]
fn crops_grow_on_ploughed_fields_and_pasture_stays_short() {
    for variety in 0..10 {
        assert_eq!(Stage::of(&field(false, variety)), Stage::Ploughed);
    }
    for variety in 0..20 {
        let stage = Stage::of(&field(true, variety));
        assert!(matches!(stage, Stage::Grass | Stage::Short));
    }
}

/// An orchard is grass under its fruit plants, whatever its variety.
#[test]
fn orchards_are_grass() {
    for variety in 0..10 {
        let orchard = FieldLook {
            orchard: true,
            ..field(false, variety)
        };
        assert_eq!(Stage::of(&orchard), Stage::Grass);
    }
}

/// About 20% land, 20% ploughed and 60% grass.
#[test]
fn ruined_fields_are_a_mix() {
    let mut counts = [0; 3];
    for y in 0..200 {
        for x in 0..200 {
            counts[match ruined_tile(x, y) {
                None => 0,
                Some(PLOUGHED_TILE) => 1,
                Some(_) => 2,
            }] += 1;
        }
    }
    assert!((7000..9000).contains(&counts[0]), "{counts:?}");
    assert!((7000..9000).contains(&counts[1]), "{counts:?}");
    assert!((23000..25000).contains(&counts[2]), "{counts:?}");
}
