use super::{
    CANDLE_LIGHT_CELLS, ChandelierStyle, Decor, DecorSlot, FixedLight, Hanging, Rug, RugColour,
    RugPattern, SCONCE_LIGHT_CELLS, fixed_lights,
};
use crate::door::Side;
use crate::dungeon::{Cell, Level};
use crate::furniture::{Candles, Furniture, Material, Piece, Wood, booth};

fn rug(at: (i64, i64), size: (u8, u8)) -> Decor {
    Decor::Rug {
        at,
        size,
        rug: Rug {
            pattern: RugPattern::Striped,
            colour: RugColour::Madder,
        },
    }
}

fn sconce(wall: (i64, i64), face: Side) -> Decor {
    Decor::Wall {
        wall,
        face,
        hanging: Hanging::Sconce,
    }
}

#[test]
fn hangings_go_only_on_a_walls_face_with_floor_in_front() {
    // The test room: floor from (-7, -3) to (7, 3), walls round it.
    let level = Level::test_room();
    assert!(sconce((0, -4), Side::South).fits(&level));
    // Facing into the wall beyond, or hung on floor: no.
    assert!(!sconce((0, -4), Side::North).fits(&level));
    assert!(!sconce((0, 0), Side::South).fits(&level));
    // The wall in the middle of the room, from either side.
    assert!(sconce((-2, 0), Side::East).fits(&level));
    assert!(sconce((-2, 0), Side::West).fits(&level));
}

#[test]
fn chandeliers_and_rugs_go_only_on_indoor_floor() {
    let mut level = Level::test_room();
    let chandelier = |at| Decor::Chandelier {
        at,
        style: ChandelierStyle::Brass,
    };
    assert!(chandelier((0, 0)).fits(&level));
    assert!(!chandelier((-2, 0)).fits(&level), "a wall");
    level.set(3, 0, Cell::Ground);
    assert!(!chandelier((3, 0)).fits(&level), "no ceiling outside");
    assert!(rug((4, 1), (4, 2)).fits(&level));
    assert!(!rug((0, 1), (5, 1)).fits(&level), "longer than RUG_MOST");
    assert!(!rug((2, 0), (2, 1)).fits(&level), "over ground");
    assert!(!rug((-3, 0), (2, 1)).fits(&level), "over a wall");
}

#[test]
fn decor_replaces_what_was_in_its_place_and_rugs_any_they_overlap() {
    let mut level = Level::test_room();
    level.place_decor(sconce((0, -4), Side::South));
    let shield = Decor::Wall {
        wall: (0, -4),
        face: Side::South,
        hanging: Hanging::Shield,
    };
    level.place_decor(shield);
    assert_eq!(level.decor().collect::<Vec<_>>(), [&shield]);
    level.place_decor(rug((0, 0), (3, 2)));
    let other = rug((2, 1), (2, 2));
    level.place_decor(other);
    assert_eq!(level.decor().count(), 2, "the first rug went");
    assert_eq!(level.decor_at(3, 2), [&other]);
    assert!(level.decor_at(0, 0).is_empty());
    level.remove_decor(DecorSlot::Floor((2, 1)));
    assert!(level.decor_at(3, 2).is_empty());
}

#[test]
fn a_hanging_is_sent_with_its_wall_and_the_cell_in_front() {
    let mut level = Level::test_room();
    let hanging = sconce((0, -4), Side::South);
    level.place_decor(hanging);
    assert_eq!(level.decor_at(0, -4), [&hanging]);
    assert_eq!(level.decor_at(0, -3), [&hanging]);
    assert!(level.decor_at(1, -3).is_empty());
}

#[test]
fn sconces_chandeliers_and_candles_on_tables_are_fixed_lights() {
    let mut level = Level::test_room();
    level.place_decor(sconce((0, -4), Side::South));
    level.place_decor(Decor::Wall {
        wall: (1, -4),
        face: Side::South,
        hanging: Hanging::Tapestry,
    });
    level.place_decor(rug((0, 0), (1, 1)));
    let [_, table, _] = booth((3, -1), 2, 1, true, Wood::Oak);
    level.place_furniture(table);
    level.place_furniture(Piece {
        at: (3, 0),
        size: (1, 1),
        facing: Side::South,
        material: Material::Wood(Wood::Ash),
        furniture: Furniture::Candles {
            candles: Candles::Pair,
        },
    });
    let lights = fixed_lights(&level);
    assert_eq!(
        lights,
        [
            FixedLight {
                cell: (0, -3),
                cells: SCONCE_LIGHT_CELLS,
            },
            FixedLight {
                cell: (3, 0),
                cells: CANDLE_LIGHT_CELLS,
            },
        ]
    );
}

#[test]
fn booths_are_two_settles_facing_across_a_well_made_table() {
    for (length, depth) in [(1, 1), (2, 1), (3, 1), (2, 2), (3, 2), (1, 2)] {
        for east_west in [true, false] {
            let pieces = booth((5, 5), length, depth, east_west, Wood::Pine);
            assert!(pieces.iter().all(Piece::is_well_made), "{pieces:?}");
            let [first, table, last] = pieces;
            assert!(table.furniture.is_table());
            assert_eq!(first.facing, last.facing.opposite());
            // Each settle faces the table, right beside it.
            assert!(table.covers(first.facing.beside(first.at)));
            assert!(table.covers(last.facing.beside(last.at)));
        }
    }
}

#[test]
fn candles_stand_on_furniture_and_are_well_made_one_cell() {
    let candles = Furniture::Candles {
        candles: Candles::Candelabrum,
    };
    assert!(candles.stands_on_furniture());
    let piece = Piece {
        at: (0, 0),
        size: (1, 1),
        facing: Side::East,
        material: Material::Wood(Wood::Oak),
        furniture: candles,
    };
    assert!(piece.is_well_made());
    assert!(
        !Piece {
            size: (2, 1),
            ..piece
        }
        .is_well_made()
    );
}
