//! Alchemy through a connected client: the order is checked, winds up, then
//! uses its herbs and liquids, gives the product (or empty bottles), practises
//! the sub-skills and Herbalism, and runs again while runs are left; nothing
//! begins without the apparatus. Ointment is rubbed on like food is eaten.

use shared::alchemy::{AlchemyOrder, GRINDING, INFUSING, OrderStep};
use shared::bonus::Source;
use shared::brewing::new_ointment;
use shared::herb::{HERBALISM, Stat};
use shared::id::Id;
use shared::item::{Item, ItemKind, StackLimits};
use shared::item_form::ItemForm;
use shared::potion::{BottleForm, Contents, Dose, Liquid, Oil, Water};
use shared::protocol::{Alchemised, ClientMessage, Landed, ServerMessage};
use shared::skill::SkillId;

use super::tests::{received, run};
use crate::connections::tests::TestClient;

/// Ticks a Mortar step takes untrained, and a few more.
const GRINDING_TICKS: u64 = 10 * 20 + 2;
/// Ticks a Mortar then a Retort take untrained, and a few more.
const BOTH_TICKS: u64 = 25 * 20 + 2;

fn give(test: &mut TestClient, item: Item) {
    let new_id = &mut shared::id::stand_ins();
    let given = test.client.inventory.give(item, StackLimits::NONE, new_id);
    assert!(given.is_ok());
}

fn bottle(liquid: Liquid, count: u32, id: u128) -> Item {
    let mut bottle = Item::new(Id::for_test(id), ItemKind::Bottle);
    bottle.form = Some(ItemForm::Bottle(BottleForm::new(Some(liquid), 1)));
    bottle.count = count;
    bottle
}

/// Three of a herb, two bottles of Nut Oil, and the mortar and pestle if
/// `mortar`: an order grinding them into ointment `count` times.
fn carrying(test: &mut TestClient, mortar: bool, count: u32) -> AlchemyOrder {
    let def = test.world.herbs.iter().next().unwrap().clone();
    give(
        test,
        shared::herb::new_item(&def, 3, Id::for_test(91), None),
    );
    give(test, bottle(Liquid::Oil(Oil::Nut), 2, 92));
    if mortar {
        give(test, Item::new(Id::for_test(93), ItemKind::MortarAndPestle));
    }
    AlchemyOrder {
        source: None,
        steps: vec![OrderStep::Mortar {
            herbs: vec![Id::for_test(91)],
            solvent: Some(Id::for_test(92)),
        }],
        count,
    }
}

fn count_of(test: &TestClient, kind: ItemKind) -> u32 {
    test.client.inventory.count_of(kind)
}

fn practised(test: &TestClient, path: &str) -> bool {
    let skill = SkillId::find(path).unwrap();
    test.client.sheet.skills.practice(skill) > 0
}

/// The `Landed::Alchemy` messages received since last asked.
fn landed(test: &mut TestClient) -> Vec<(Alchemised, u32)> {
    let messages = received(test);
    let landed = messages.iter().filter_map(|message| match message {
        ServerMessage::AbilityLanded {
            landed: Landed::Alchemy { made, left, .. },
            ..
        } => Some((*made, *left)),
        _ => None,
    });
    landed.collect()
}

#[test]
fn a_mortar_makes_ointment_and_practises_grinding() {
    let mut test = TestClient::new();
    test.enter_a_castle();
    let order = carrying(&mut test, true, 1);
    test.send(&[ClientMessage::Alchemy(order)]);
    assert!(test.client.fight.pending.is_some(), "it winds up");
    run(&mut test, GRINDING_TICKS, false);
    assert_eq!(count_of(&test, ItemKind::Herb), 2, "one of the herb used");
    let empty = test.client.inventory.empty_bottle();
    assert!(empty.is_some(), "one oil used, its bottle kept");
    let landed = landed(&mut test);
    assert_eq!(landed.len(), 1);
    // Untrained, one step is certain: it never fails.
    match landed[0] {
        (Alchemised::Made { kind, count, .. }, 0) => {
            assert_eq!((kind, count), (ItemKind::Ointment, 1));
            assert_eq!(count_of(&test, ItemKind::Ointment), 1);
        }
        (Alchemised::Nothing, 0) => assert_eq!(count_of(&test, ItemKind::Ointment), 0),
        other => panic!("{other:?}"),
    }
    assert!(practised(&test, GRINDING) && practised(&test, HERBALISM));
}

#[test]
fn a_count_runs_again_until_done() {
    let mut test = TestClient::new();
    test.enter_a_castle();
    let order = carrying(&mut test, true, 2);
    test.send(&[ClientMessage::Alchemy(order)]);
    run(&mut test, GRINDING_TICKS * 2 + 20, false);
    assert_eq!(count_of(&test, ItemKind::Herb), 1, "two runs, a herb each");
    let lefts: Vec<u32> = landed(&mut test).iter().map(|(_, left)| *left).collect();
    assert_eq!(lefts, vec![1, 0]);
}

#[test]
fn a_retort_uses_up_its_solid() {
    let mut test = TestClient::new();
    test.enter_a_castle();
    let mut order = carrying(&mut test, true, 1);
    give(&mut test, Item::new(Id::for_test(94), ItemKind::Retort));
    give(&mut test, bottle(Liquid::Water(Water::Plain), 1, 95));
    give(
        &mut test,
        shared::alchemy::new_solid(ItemKind::Sulphur, 2, Id::for_test(96)),
    );
    order.steps.push(OrderStep::Retort {
        liquids: vec![Id::for_test(95)],
        solid: Some(Id::for_test(96)),
    });
    test.send(&[ClientMessage::Alchemy(order)]);
    run(&mut test, BOTH_TICKS, false);
    assert_eq!(landed(&mut test).len(), 1);
    assert_eq!(count_of(&test, ItemKind::Sulphur), 1, "one Sulphur a run");
}

#[test]
fn a_retort_turns_its_waters_bottles_into_potions() {
    let mut test = TestClient::new();
    test.enter_a_castle();
    let mut order = carrying(&mut test, true, 1);
    give(&mut test, Item::new(Id::for_test(94), ItemKind::Retort));
    give(&mut test, bottle(Liquid::Water(Water::Plain), 2, 95));
    order.steps.push(OrderStep::Retort {
        liquids: vec![Id::for_test(95), Id::for_test(95)],
        solid: None,
    });
    test.send(&[ClientMessage::Alchemy(order)]);
    run(&mut test, BOTH_TICKS, false);
    assert!(practised(&test, INFUSING), "a mixture infused");
    let waters = test.client.inventory.water_stacks();
    assert!(waters.is_empty(), "both waters used");
    let landed = landed(&mut test);
    assert_eq!(landed.len(), 1);
    // Two bottles back: holding the potion, or empty.
    let bottles = count_of(&test, ItemKind::Bottle);
    assert_eq!(bottles, 4, "two from the waters, two from the oils");
}

#[test]
fn nothing_begins_without_the_apparatus() {
    let mut test = TestClient::new();
    test.enter_a_castle();
    let order = carrying(&mut test, false, 1);
    test.send(&[ClientMessage::Alchemy(order)]);
    assert!(test.client.fight.pending.is_none());
}

#[test]
fn being_harmed_stops_it_and_leaves_everything() {
    let mut test = TestClient::new();
    test.enter_a_castle();
    let order = carrying(&mut test, true, 3);
    test.send(&[ClientMessage::Alchemy(order)]);
    let hurt = shared::protocol::DevVitals::Hurt(1);
    test.send(&[ClientMessage::DevVitals(hurt)]);
    run(&mut test, GRINDING_TICKS * 3, false);
    assert_eq!(count_of(&test, ItemKind::Herb), 3);
    assert_eq!(count_of(&test, ItemKind::Ointment), 0);
    assert!(test.client.alchemy.is_none(), "the runs left go too");
}

#[test]
fn ointment_is_rubbed_on_for_its_effect() {
    let mut test = TestClient::new();
    test.enter_a_castle();
    let contents = Contents::new([Dose {
        stat: Stat::Strength,
        step: 8,
    }]);
    give(&mut test, new_ointment(contents, 2, Id::for_test(94)));
    test.send(&[ClientMessage::Consume {
        item: Id::for_test(94),
    }]);
    assert_eq!(count_of(&test, ItemKind::Ointment), 1);
    let effects = &test.client.sheet.effects.0;
    assert!(
        effects
            .iter()
            .any(|effect| effect.source == Source::Ointment)
    );
}

/// A cider (6%) or a wine (12%).
fn drink(strength: u32) -> Liquid {
    Liquid::Drink(Box::new(shared::fermentation::Drink {
        base: shared::fermentation::Base::Fruit("apple".into()),
        strength,
        sweetness: shared::fermentation::Sweetness::Dry,
        quality: shared::item::Quality::Common,
        vinegar: 0,
    }))
}

#[test]
fn an_alembic_distils_drinks_into_pure_alcohol_and_salt_water_into_salt() {
    let mut test = TestClient::new();
    test.enter_a_castle();
    // Enough Alchemy that two steps are certain.
    let alchemy = SkillId::find(shared::ability::ALCHEMY).unwrap();
    test.client.sheet.skills.practise(alchemy, 100_000);
    give(&mut test, Item::new(Id::for_test(94), ItemKind::Retort));
    give(&mut test, Item::new(Id::for_test(95), ItemKind::Alembic));
    give(&mut test, bottle(drink(60), 5, 96));
    give(&mut test, bottle(drink(120), 1, 97));
    give(&mut test, bottle(Liquid::Water(Water::Salt), 2, 98));
    let order = AlchemyOrder {
        source: None,
        steps: vec![
            OrderStep::Retort {
                liquids: vec![Id::for_test(96); 4]
                    .into_iter()
                    .chain([Id::for_test(98)])
                    .collect(),
                solid: None,
            },
            OrderStep::Alembic {
                alcohol: Some(Id::for_test(97)),
            },
        ],
        count: 1,
    };
    test.send(&[ClientMessage::Alchemy(order)]);
    assert!(test.client.fight.pending.is_some(), "it winds up");
    run(&mut test, 35 * 20 + 2, false);
    assert!(practised(&test, shared::alchemy::DISTILLING));
    let landed = landed(&mut test);
    assert_eq!(landed, vec![(Alchemised::Alcohol { count: 2 }, 0)]);
    let count = test.client.inventory.bottles_of(&Liquid::Alcohol);
    assert_eq!(count, 2, "5 units halved, 24 ml of alcohol enough for both");
    assert_eq!(
        count_of(&test, ItemKind::Salt),
        1,
        "one Salt Water distilled"
    );
    assert_eq!(count_of(&test, ItemKind::Bottle), 8, "every bottle back");
}

#[test]
fn a_filter_uses_a_powder_and_gives_a_powder() {
    let mut test = TestClient::new();
    test.enter_a_castle();
    give(&mut test, Item::new(Id::for_test(94), ItemKind::Filter));
    let aura = Contents::new([Dose {
        stat: Stat::Aura,
        step: 5,
    }]);
    let quality = shared::item::Quality::Common;
    let powders = shared::alchemy::new_mixture(
        shared::alchemy::Form::Powder,
        aura,
        quality,
        2,
        Id::for_test(81),
    );
    give(&mut test, powders);
    let order = AlchemyOrder {
        source: Some(Id::for_test(81)),
        steps: vec![OrderStep::Filter {
            powder: Some(Id::for_test(81)),
        }],
        count: 1,
    };
    test.send(&[ClientMessage::Alchemy(order)]);
    run(&mut test, GRINDING_TICKS, false);
    assert!(practised(&test, shared::alchemy::FILTERING));
    match landed(&mut test).as_slice() {
        [(Alchemised::Made { kind, count, .. }, 0)] => {
            assert_eq!((*kind, *count), (ItemKind::Powder, 1));
        }
        other => panic!("{other:?}"),
    }
    assert_eq!(count_of(&test, ItemKind::Powder), 1, "both used, one made");
}

#[test]
fn a_pelican_circulates_a_powder_into_a_purer_one() {
    let mut test = TestClient::new();
    test.enter_a_castle();
    give(&mut test, Item::new(Id::for_test(95), ItemKind::Pelican));
    // Aura 13 is Common (13 ÷ 3); circulated, 15 is Uncommon.
    let aura = Contents::new([Dose {
        stat: Stat::Aura,
        step: 13,
    }]);
    let quality = shared::item::Quality::Common;
    let powders = shared::alchemy::new_mixture(
        shared::alchemy::Form::Powder,
        aura,
        quality,
        1,
        Id::for_test(82),
    );
    give(&mut test, powders);
    let order = AlchemyOrder {
        source: Some(Id::for_test(82)),
        steps: vec![OrderStep::Pelican],
        count: 1,
    };
    test.send(&[ClientMessage::Alchemy(order)]);
    // 30 s untrained.
    run(&mut test, 30 * 20 + 2, false);
    assert!(practised(&test, shared::alchemy::CIRCULATING));
    // Uncommon, or better by luck.
    use shared::item::Quality::{Common, Poor};
    match landed(&mut test).as_slice() {
        [
            (
                Alchemised::Made {
                    kind,
                    count,
                    quality,
                },
                0,
            ),
        ] => {
            assert_eq!((*kind, *count), (ItemKind::Powder, 1));
            assert!(*quality != Poor && *quality != Common, "{quality:?}");
        }
        other => panic!("{other:?}"),
    }
}
