use shared::door::Side;
use shared::furniture::{ChairStyle, Furniture, Material, Piece, TableStyle, Wood};
use shared::protocol::{ClientMessage, DevFurniture};
use shared::site::{DevArea, DevLayout, DevMobs};

use crate::connections::tests::TestClient;

/// A client in an empty 32-cell Dev Area, on its stairs at (1, 16).
fn in_an_area() -> TestClient {
    let mut test = TestClient::new();
    test.send(&[ClientMessage::DevGenerateArea(DevArea {
        size: 32,
        layout: DevLayout::Empty,
        mobs: DevMobs::None,
        seed: 3,
    })]);
    test
}

fn table(at: (i64, i64)) -> Piece {
    Piece {
        at,
        size: (2, 3),
        facing: Side::South,
        material: Material::Wood(Wood::Oak),
        furniture: Furniture::Table {
            style: TableStyle::Joined,
        },
    }
}

/// Places `piece` and shares it, as the tick would.
fn place(test: &mut TestClient, piece: Piece) {
    test.send(&[ClientMessage::DevFurniture(DevFurniture { piece })]);
    super::share_furniture(std::slice::from_mut(&mut test.client), &mut test.world);
}

fn level(test: &TestClient) -> &shared::dungeon::Level {
    &test.client.visit.as_ref().unwrap().level.level
}

#[test]
fn placed_furniture_is_on_the_level_and_replaces_what_it_overlaps() {
    let mut test = in_an_area();
    let before = level(&test).furniture().count();
    place(&mut test, table((6, 10)));
    assert_eq!(level(&test).furniture_at(7, 12), Some(&table((6, 10))));
    assert_eq!(level(&test).furniture_cost_per_mille(7, 12), 3000);
    // The mobs' level has it too.
    let place_of = test.client.visit.as_ref().unwrap().population();
    let mobs_level = test.world.populations.0[&place_of].level();
    assert_eq!(mobs_level.furniture_cost_per_mille(6, 10), 3000);
    // A chair on its corner takes the whole table away.
    let chair = Piece {
        size: (1, 1),
        facing: Side::West,
        furniture: Furniture::Chair {
            style: ChairStyle::LadderBack,
            arms: true,
        },
        ..table((7, 12))
    };
    place(&mut test, chair);
    assert_eq!(level(&test).furniture_at(7, 12), Some(&chair));
    assert_eq!(level(&test).furniture_at(6, 10), None);
    assert_eq!(level(&test).furniture().count(), before + 1);
}

#[test]
fn badly_made_pieces_and_pieces_off_the_floor_are_refused() {
    let mut test = in_an_area();
    let before: Vec<Piece> = level(&test).furniture().copied().collect();
    // Facing across its length; into the wall; on the stairs.
    let across = Piece {
        facing: Side::East,
        ..table((6, 10))
    };
    for piece in [across, table((-1, 10)), table((0, 14))] {
        place(&mut test, piece);
    }
    let after: Vec<Piece> = level(&test).furniture().copied().collect();
    assert_eq!(after, before);
}
