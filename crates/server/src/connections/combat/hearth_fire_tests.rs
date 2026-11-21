//! Hearths' fires (cooking plan PR 8): a hearth that isn't always lit is lit
//! and fed with kindling by Make Campfire, and players are told it is lit.

use shared::ability::Ability;
use shared::campfire::feed_ticks;
use shared::crafting_bench::Craft;
use shared::door::Side;
use shared::facing::Heading;
use shared::furniture::{Furniture, Material, Piece, Stone};
use shared::id::Id;
use shared::item::{ItemKind, StackLimits};
use shared::material::Material as Wood;
use shared::protocol::{BenchAsk, BenchNews, ClientMessage, DevBench, ServerMessage};
use shared::site::{DevArea, DevLayout, DevMobs};
use shared::woodcutting::new_kindling;

use super::tests::run;
use crate::connections::tests::TestClient;

/// A client in an empty Dev Area before a Stove at `(6, 10)` (made by hand,
/// so not always lit), facing it, carrying two pieces of kindling.
fn at_a_stove() -> TestClient {
    let mut test = TestClient::new();
    test.send(&[ClientMessage::DevGenerateArea(DevArea {
        size: 32,
        layout: DevLayout::Empty,
        mobs: DevMobs::None,
        seed: 3,
    })]);
    let stove = Piece {
        at: (6, 10),
        size: (1, 1),
        facing: Side::South,
        material: Material::Stone(Stone::Granite),
        furniture: Furniture::CraftingBench {
            craft: Craft::Stove,
        },
    };
    test.send(&[ClientMessage::Bench(BenchAsk::Dev(DevBench {
        piece: stove,
        tools: Vec::new(),
    }))]);
    test.send(&[
        ClientMessage::DevTeleport { x: 6, y: 11 },
        ClientMessage::Face(Heading::NORTH),
    ]);
    let kindling = new_kindling(Wood::Pine, 2, Id::for_test(5), None);
    let stand_ins = &mut shared::id::stand_ins();
    (test.client.inventory)
        .give(kindling, StackLimits::NONE, stand_ins)
        .unwrap();
    assert!((test.client.sheet.abilities).learn_from(&test.client.inventory));
    test
}

/// Whether the players are told the stove is lit, as the tick shares it.
fn told_lit(test: &mut TestClient) -> Option<bool> {
    let clients = std::slice::from_mut(&mut test.client);
    crate::sites::share_furniture(clients, &mut test.world);
    crate::sites::share_benches(std::slice::from_mut(&mut test.client), &mut test.world);
    let received = std::iter::from_fn(|| test.end.receive().unwrap());
    let told = received.filter_map(|message| match message {
        ServerMessage::Bench(BenchNews::Tools(benches)) => Some(benches),
        _ => None,
    });
    let stove = told.flatten().filter(|seen| seen.at == (6, 10));
    stove.last().map(|seen| seen.lit)
}

#[test]
fn kindling_lights_a_hearth_and_feeds_it() {
    let mut test = at_a_stove();
    assert_eq!(told_lit(&mut test), Some(false), "cold until lit");
    let make = ClientMessage::UseAbility {
        ability: Ability::MakeCampfire,
        target: None,
    };
    test.send(std::slice::from_ref(&make));
    assert!(test.client.fight.pending.is_some(), "lighting the stove");
    run(&mut test, feed_ticks() + 1, false);
    assert_eq!(test.client.inventory.count_of(ItemKind::Kindling), 1);
    assert_eq!(told_lit(&mut test), Some(true));
    let visit = test.client.visit.as_ref().unwrap();
    assert!(
        visit.cells().campfires().next().is_none(),
        "no campfire made"
    );
    test.send(&[make]);
    run(&mut test, feed_ticks() + 1, false);
    assert_eq!(test.client.inventory.count_of(ItemKind::Kindling), 0, "fed");
}
