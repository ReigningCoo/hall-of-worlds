//! Fireplaces (tavern furniture plan PR 4): kindling goes in the log basket,
//! Make Campfire lights the fire, players are told it is lit, and a lit one
//! cooks as a hearth with no Oven, leaving Ash.

use shared::ability::Ability;
use shared::campfire::feed_ticks;
use shared::cooking::{CookOrder, OrderInput, OrderStep, Tool};
use shared::door::Side;
use shared::facing::Heading;
use shared::fireplace::Fireplace;
use shared::furniture::{Furniture, Held, Material, Piece, Stone};
use shared::id::Id;
use shared::item::{Item, ItemKind, StackLimits};
use shared::livestock::Livestock;
use shared::material::Material as Wood;
use shared::protocol::{BenchAsk, BenchNews, ClientMessage, DevFurniture, ServerMessage};
use shared::site::{DevArea, DevLayout, DevMobs};
use shared::woodcutting::new_kindling;

use super::tests::run;
use crate::connections::tests::TestClient;

/// Ticks a Pan step takes untrained, and a few more.
const FRYING_TICKS: u64 = 20 * 20 + 2;

fn give(test: &mut TestClient, item: Item) {
    let new_id = &mut shared::id::stand_ins();
    let given = test.client.inventory.give(item, StackLimits::NONE, new_id);
    assert!(given.is_ok());
}

/// A client in an empty Dev Area facing a Wide Fireplace at `(6, 10)` (fire
/// in its first cell), carrying kindling (two pieces as stack 5, three as
/// stack 6), a Pan and two hen's eggs (stack 91).
fn at_a_fireplace() -> TestClient {
    let mut test = TestClient::new();
    test.send(&[ClientMessage::DevGenerateArea(DevArea {
        size: 32,
        layout: DevLayout::Empty,
        mobs: DevMobs::None,
        seed: 3,
    })]);
    let piece = Piece {
        at: (6, 10),
        size: (2, 1),
        facing: Side::South,
        material: Material::Stone(Stone::Granite),
        furniture: Furniture::Fireplace {
            fireplace: Fireplace { length: 2 },
        },
    };
    test.send(&[ClientMessage::DevFurniture(DevFurniture { piece })]);
    test.send(&[
        ClientMessage::DevTeleport { x: 6, y: 11 },
        ClientMessage::Face(Heading::NORTH),
    ]);
    give(
        &mut test,
        new_kindling(Wood::Pine, 2, Id::for_test(5), None),
    );
    give(&mut test, new_kindling(Wood::Oak, 3, Id::for_test(6), None));
    let hen = Livestock::built_in().get("chicken").unwrap();
    let eggs = shared::farmyard::new_egg(hen, 2, Id::for_test(91), None).unwrap();
    give(&mut test, eggs);
    give(&mut test, Item::new(Id::for_test(92), ItemKind::Pan));
    assert!((test.client.sheet.abilities).learn_from(&test.client.inventory));
    test
}

/// What players were last told of the fireplace, as the tick shares it.
fn told(test: &mut TestClient) -> Option<(Held, bool)> {
    let clients = std::slice::from_mut(&mut test.client);
    crate::sites::share_furniture(clients, &mut test.world);
    crate::sites::share_benches(std::slice::from_mut(&mut test.client), &mut test.world);
    let received = std::iter::from_fn(|| test.end.receive().unwrap());
    let told = received.filter_map(|message| match message {
        ServerMessage::Bench(BenchNews::Tools(benches)) => Some(benches),
        _ => None,
    });
    let fireplace = told.flatten().filter(|seen| seen.at == (6, 10));
    fireplace.last().map(|seen| (seen.held, seen.lit))
}

/// Frying one egg in a Pan, at the fireplace's fire cell, or an Oven step.
fn order(tool: Tool) -> CookOrder {
    CookOrder {
        bench: Some((6, 10)),
        rows: 1,
        sources: Vec::new(),
        steps: vec![OrderStep {
            tool,
            column: 0,
            row: 0,
            height: 1,
            inputs: vec![OrderInput::Food {
                stack: Id::for_test(91),
                count: 1,
            }],
        }],
        culture: None,
        count: 1,
    }
}

#[test]
fn kindling_goes_in_the_basket() {
    let mut test = at_a_fireplace();
    assert!(matches!(told(&mut test), Some((_, false))), "cold");
    test.send(&[ClientMessage::Bench(BenchAsk::Put {
        bench: (6, 10),
        item: Id::for_test(6),
    })]);
    let Some((Held::Fire(fireside), false)) = told(&mut test) else {
        panic!("not told of the basket");
    };
    assert_eq!(fireside.items().map(|item| item.count).sum::<u32>(), 3);
    assert_eq!(test.client.inventory.count_of(ItemKind::Kindling), 2);
    test.send(&[ClientMessage::Bench(BenchAsk::Put {
        bench: (6, 10),
        item: Id::for_test(92),
    })]);
    assert_eq!(
        test.client.inventory.count_of(ItemKind::Pan),
        1,
        "a Pan back"
    );
}

#[test]
fn a_lit_fireplace_fries_but_never_bakes() {
    let mut test = at_a_fireplace();
    assert!(matches!(told(&mut test), Some((_, false))), "cold");
    test.send(&[ClientMessage::Cook(order(Tool::Pan))]);
    assert!(test.client.fight.pending.is_none(), "cold until lit");
    test.send(&[ClientMessage::UseAbility {
        ability: Ability::MakeCampfire,
        target: None,
    }]);
    assert!(test.client.fight.pending.is_some(), "lighting it");
    run(&mut test, feed_ticks() + 1, false);
    assert_eq!(test.client.inventory.count_of(ItemKind::Kindling), 4);
    assert!(matches!(told(&mut test), Some((_, true))), "lit");
    let visit = test.client.visit.as_ref().unwrap();
    assert!(visit.cells().campfires().next().is_none(), "no campfire");
    test.send(&[ClientMessage::Cook(order(Tool::Oven))]);
    assert!(test.client.fight.pending.is_none(), "no Oven");
    test.send(&[ClientMessage::Cook(order(Tool::Pan))]);
    assert!(test.client.fight.pending.is_some(), "it fries");
    run(&mut test, FRYING_TICKS, false);
    assert_eq!(test.client.inventory.count_of(ItemKind::Egg), 1);
    assert_eq!(test.client.inventory.count_of(ItemKind::Ash), 1, "Ash");
}
