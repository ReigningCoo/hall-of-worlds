//! Cooking through a connected client (cooking plan PR 9): the order is
//! checked against what is carried and the kitchen, winds up, then uses its
//! food, gives what it made, practises the sub-skills, leaves Ash at a lit
//! hearth, and runs again while runs are left; heat steps never begin cold.

use shared::ability::Ability;
use shared::campfire::feed_ticks;
use shared::cooking::{CookOrder, FRYING, MIXING, OrderInput, OrderStep, Tool};
use shared::crafting_bench::Craft;
use shared::door::Side;
use shared::facing::Heading;
use shared::furniture::{Furniture, Material, Piece, Stone};
use shared::id::Id;
use shared::item::{Item, ItemKind, StackLimits};
use shared::livestock::Livestock;
use shared::material::Material as Wood;
use shared::protocol::{BenchAsk, ClientMessage, Cooked, DevBench, Landed, ServerMessage};
use shared::site::{DevArea, DevLayout, DevMobs};
use shared::skill::SkillId;
use shared::woodcutting::new_kindling;

use super::tests::{received, run};
use crate::connections::tests::TestClient;

/// Ticks a Pan step takes untrained, and a few more.
const FRYING_TICKS: u64 = 20 * 20 + 2;
/// Ticks a Bowl step takes untrained, and a few more.
const MIXING_TICKS: u64 = 10 * 20 + 2;

fn give(test: &mut TestClient, item: Item) {
    let new_id = &mut shared::id::stand_ins();
    let given = test.client.inventory.give(item, StackLimits::NONE, new_id);
    assert!(given.is_ok());
}

/// `count` hen's eggs, as stack 91, and a `tool`.
fn carrying(test: &mut TestClient, count: u32, tool: ItemKind) {
    let hen = Livestock::built_in().get("chicken").unwrap();
    let eggs = shared::farmyard::new_egg(hen, count, Id::for_test(91), None).unwrap();
    give(test, eggs);
    give(test, Item::new(Id::for_test(92), tool));
}

/// One step of `tool` with `eggs` of stack 91 in it, `count` times.
fn order(tool: Tool, eggs: u32, bench: Option<(i64, i64)>, count: u32) -> CookOrder {
    CookOrder {
        bench,
        rows: 1,
        sources: Vec::new(),
        steps: vec![OrderStep {
            tool,
            column: 0,
            row: 0,
            height: 1,
            inputs: vec![OrderInput::Food {
                stack: Id::for_test(91),
                count: eggs,
            }],
        }],
        culture: None,
        count,
    }
}

/// A client in an empty Dev Area facing a Stove at `(6, 10)` (made by hand,
/// so burning kindling), lit with Make Campfire if `lit`.
fn at_a_stove(lit: bool) -> TestClient {
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
    let clients = std::slice::from_mut(&mut test.client);
    crate::sites::share_furniture(clients, &mut test.world);
    if lit {
        give(
            &mut test,
            new_kindling(Wood::Pine, 2, Id::for_test(5), None),
        );
        assert!((test.client.sheet.abilities).learn_from(&test.client.inventory));
        test.send(&[ClientMessage::UseAbility {
            ability: Ability::MakeCampfire,
            target: None,
        }]);
        run(&mut test, feed_ticks() + 1, false);
        assert_eq!(test.client.inventory.count_of(ItemKind::Kindling), 1, "lit");
    }
    received(&mut test);
    test
}

fn count_of(test: &TestClient, kind: ItemKind) -> u32 {
    test.client.inventory.count_of(kind)
}

fn practised(test: &TestClient, path: &str) -> bool {
    let skill = SkillId::find(path).unwrap();
    test.client.sheet.skills.practice(skill) > 0
}

/// The `Landed::Cooked` messages received since last asked.
fn landed(test: &mut TestClient) -> Vec<(Cooked, u32, u32)> {
    let messages = received(test);
    let landed = messages.iter().filter_map(|message| match message {
        ServerMessage::AbilityLanded {
            landed: Landed::Cooked { made, ash, left },
            ..
        } => Some((*made, *ash, *left)),
        _ => None,
    });
    landed.collect()
}

#[test]
fn a_lit_hearth_fries_an_egg_and_leaves_ash() {
    let mut test = at_a_stove(true);
    carrying(&mut test, 2, ItemKind::Pan);
    let fry = order(Tool::Pan, 1, Some((6, 10)), 1);
    test.send(&[ClientMessage::Cook(fry)]);
    assert!(test.client.fight.pending.is_some(), "it winds up");
    run(&mut test, FRYING_TICKS, false);
    assert_eq!(count_of(&test, ItemKind::Egg), 1, "one egg used");
    let landed = landed(&mut test);
    // Untrained, one step is certain: it never fails.
    let [(Cooked::Made { kind, outputs, .. }, ash, 0)] = landed[..] else {
        panic!("{landed:?}");
    };
    assert_eq!((kind, outputs, ash), (ItemKind::Dish, 1, 1));
    assert!(count_of(&test, ItemKind::Dish) >= 1);
    assert_eq!(count_of(&test, ItemKind::Ash), 1, "a hearth's Ash");
    assert!(practised(&test, FRYING));
}

#[test]
fn heat_steps_never_begin_cold() {
    let mut test = at_a_stove(false);
    carrying(&mut test, 2, ItemKind::Pan);
    test.send(&[ClientMessage::Cook(order(Tool::Pan, 1, Some((6, 10)), 1))]);
    assert!(test.client.fight.pending.is_none(), "an unlit hearth");
    test.send(&[ClientMessage::Cook(order(Tool::Pan, 1, None, 1))]);
    assert!(test.client.fight.pending.is_none(), "nothing lit at all");
    assert_eq!(count_of(&test, ItemKind::Egg), 2);
}

#[test]
fn a_bowl_works_anywhere_and_runs_again() {
    let mut test = TestClient::new();
    test.enter_a_castle();
    carrying(&mut test, 4, ItemKind::Bowl);
    test.send(&[ClientMessage::Cook(order(Tool::Bowl, 2, None, 2))]);
    run(&mut test, MIXING_TICKS * 2 + 20, false);
    assert_eq!(count_of(&test, ItemKind::Egg), 0, "two runs, two eggs each");
    let lefts: Vec<_> = landed(&mut test)
        .iter()
        .map(|(_, ash, left)| (*ash, *left))
        .collect();
    assert_eq!(lefts, vec![(0, 1), (0, 0)], "no Ash away from a hearth");
    assert!(practised(&test, MIXING));
}

#[test]
fn moving_calls_cooking_off() {
    let mut test = TestClient::new();
    test.enter_a_castle();
    carrying(&mut test, 4, ItemKind::Bowl);
    test.send(&[ClientMessage::Cook(order(Tool::Bowl, 2, None, 2))]);
    assert!(test.client.stop_collecting(test.world.action_clock.ticks()));
    assert!(test.client.cooking.is_none(), "the runs left go too");
    run(&mut test, MIXING_TICKS, false);
    assert_eq!(count_of(&test, ItemKind::Egg), 4);
}
