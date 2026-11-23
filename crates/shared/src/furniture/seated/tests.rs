use super::*;
use crate::furniture::{Crates, Sacks, Stone};

#[test]
fn games_tables_have_a_chair_each_side_facing_them() {
    for game in BoardGame::ALL {
        for facing in [Side::North, Side::East, Side::South, Side::West] {
            let [table, light, dark] =
                games_table((4, 7), game, facing, ChairStyle::Stool, Wood::Oak);
            for piece in [table, light, dark] {
                assert!(piece.is_well_made(), "{piece:?}");
            }
            assert_eq!(light.facing, facing);
            assert_eq!(light.facing.beside(light.at), table.at);
            assert_eq!(dark.facing.beside(dark.at), table.at);
            assert!(table.furniture.holds_furniture());
        }
    }
}

#[test]
fn barrels_have_stools_round_them() {
    let sides = [Side::North, Side::East, Side::South, Side::West];
    let pieces = barrel_table((0, 0), Barrel::Topped, &sides, Wood::Pine);
    assert_eq!(pieces.len(), 5);
    for stool in &pieces[1..] {
        assert!(stool.is_well_made());
        assert_eq!(stool.facing.beside(stool.at), (0, 0));
    }
    assert!(pieces[0].is_well_made() && pieces[0].furniture.holds_furniture());
}

#[test]
fn clutter_is_wooden_and_costs_as_tables_and_seats() {
    let piece = |furniture, material| Piece {
        at: (0, 0),
        size: (1, 1),
        facing: Side::West,
        material,
        furniture,
    };
    let oak = Material::Wood(Wood::Oak);
    for crates in Crates::ALL {
        let furniture = Furniture::Crates { crates };
        assert!(piece(furniture, oak).is_well_made());
        assert!(!piece(furniture, Material::Stone(Stone::Granite)).is_well_made());
        assert_eq!(
            furniture.cost_per_mille(),
            crate::furniture::TABLE_COST_PER_MILLE
        );
        assert!(!furniture.holds_furniture());
    }
    for sacks in Sacks::ALL {
        let furniture = Furniture::Sacks { sacks };
        assert!(piece(furniture, oak).is_well_made());
        assert_eq!(
            furniture.cost_per_mille(),
            crate::furniture::SEAT_COST_PER_MILLE
        );
    }
    let wide = Piece {
        size: (2, 1),
        ..piece(
            Furniture::GamesTable {
                game: BoardGame::Chess,
            },
            oak,
        )
    };
    assert!(!wide.is_well_made());
}
