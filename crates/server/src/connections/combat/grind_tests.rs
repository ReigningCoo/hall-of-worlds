//! Grind through a connected client: the order is checked, winds up, then uses
//! its herbs and oil, gives the ointment and the empty bottle, practises
//! Alchemy and Herbalism and remembers the mix; nothing begins without a
//! mortar and pestle. Ointment is rubbed on like food is eaten.

use shared::ability::ALCHEMY;
use shared::bonus::Source;
use shared::brewing::{GrindOrder, new_ointment};
use shared::herb::{HERBALISM, Stat};
use shared::id::Id;
use shared::item::{Item, ItemKind, StackLimits};
use shared::item_form::ItemForm;
use shared::potion::{BottleForm, Contents, Dose, Liquid, Oil};
use shared::protocol::{ClientMessage, Landed, ServerMessage};
use shared::skill::SkillId;

use super::tests::{received, run};
use crate::connections::tests::TestClient;

/// Ticks a grinding takes untrained, and a few more.
const GRINDING: u64 = 10 * 20 + 2;

fn give(test: &mut TestClient, item: Item) {
    let new_id = &mut shared::id::stand_ins();
    let given = test.client.inventory.give(item, StackLimits::NONE, new_id);
    assert!(given.is_ok());
}

/// Three of a herb, a bottle of Nut Oil and a mortar and pestle if `mortar`:
/// the order grinding one dose of them.
fn carrying(test: &mut TestClient, mortar: bool) -> GrindOrder {
    let def = test.world.herbs.iter().next().unwrap().clone();
    give(
        test,
        shared::herb::new_item(&def, 3, Id::for_test(91), None),
    );
    let mut oil = Item::new(Id::for_test(92), ItemKind::Bottle);
    let liquid = Some(Liquid::Oil(Oil::Nut));
    oil.form = Some(ItemForm::Bottle(BottleForm::new(liquid, 1)));
    give(test, oil);
    if mortar {
        give(test, Item::new(Id::for_test(93), ItemKind::MortarAndPestle));
    }
    GrindOrder {
        herbs: vec![Id::for_test(91)],
        oil: Id::for_test(92),
        doses: 1,
    }
}

fn count_of(test: &TestClient, kind: ItemKind) -> u32 {
    test.client.inventory.count_of(kind)
}

#[test]
fn grinding_makes_ointment_from_the_herbs_and_oil() {
    let mut test = TestClient::new();
    test.enter_a_castle();
    let order = carrying(&mut test, true);
    test.send(&[ClientMessage::Grind(order)]);
    assert!(test.client.fight.pending.is_some(), "it winds up");
    run(&mut test, GRINDING, false);
    assert_eq!(count_of(&test, ItemKind::Herb), 2, "one of the herb used");
    assert_eq!(count_of(&test, ItemKind::Ointment), 1);
    let bottle = test.client.inventory.find(Id::for_test(92));
    let bottle = bottle.and_then(Item::bottle).expect("the bottle");
    assert!(bottle.is_empty(), "the oil is used up, the bottle kept");
    assert_eq!(test.client.sheet.recipes.iter().count(), 1);
    let ground = received(&mut test).iter().any(|message| {
        matches!(
            message,
            ServerMessage::AbilityLanded {
                landed: Landed::Ground(1),
                ..
            }
        )
    });
    assert!(ground);
    for path in [ALCHEMY, HERBALISM] {
        let skill = SkillId::find(path).unwrap();
        assert!(test.client.sheet.skills.practice(skill) > 0, "{path}");
    }
}

#[test]
fn nothing_is_ground_without_a_mortar_and_pestle() {
    let mut test = TestClient::new();
    test.enter_a_castle();
    let order = carrying(&mut test, false);
    test.send(&[ClientMessage::Grind(order)]);
    assert!(test.client.fight.pending.is_none());
}

#[test]
fn being_harmed_stops_it_and_leaves_everything() {
    let mut test = TestClient::new();
    test.enter_a_castle();
    let order = carrying(&mut test, true);
    test.send(&[ClientMessage::Grind(order)]);
    let hurt = shared::protocol::DevVitals::Hurt(1);
    test.send(&[ClientMessage::DevVitals(hurt)]);
    run(&mut test, GRINDING, false);
    assert_eq!(count_of(&test, ItemKind::Herb), 3);
    assert_eq!(count_of(&test, ItemKind::Ointment), 0);
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
