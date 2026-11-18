//! Brew through a connected client: the order is checked, winds up, then uses
//! an ointment and turns the bottle of water into the potion (or empties it
//! when none forms), practises Alchemy and Herbalism and teaches the herbs'
//! effects; nothing begins without a cauldron.

use shared::ability::ALCHEMY;
use shared::brewing::{BrewOrder, Recipe, new_ointment};
use shared::herb::HERBALISM;
use shared::id::Id;
use shared::item::{Item, ItemKind, StackLimits};
use shared::item_form::ItemForm;
use shared::potion::{BottleForm, Contents, Dose, Liquid, Oil, Water};
use shared::protocol::{ClientMessage, Landed, ServerMessage};
use shared::skill::SkillId;

use super::tests::{received, run};
use crate::connections::tests::TestClient;

/// Ticks a brewing takes untrained, and a few more.
const BREWING: u64 = 15 * 20 + 2;

fn give(test: &mut TestClient, item: Item) {
    let new_id = &mut shared::id::stand_ins();
    let given = test.client.inventory.give(item, StackLimits::NONE, new_id);
    assert!(given.is_ok());
}

/// An ointment of `step` on the first effect's stat of the first herb (its mix
/// remembered), a bottle of `water` and a cauldron if `cauldron`: the order.
fn carrying(test: &mut TestClient, step: i16, water: Water, cauldron: bool) -> (BrewOrder, String) {
    let def = test.world.herbs.iter().next().unwrap().clone();
    let stat = def.effects[0].stat;
    let contents = Contents::new([Dose { stat, step }]);
    let recipe = Recipe::new([def.key.clone()], Oil::Nut, contents.clone());
    test.client.sheet.recipes.remember(recipe);
    give(test, new_ointment(contents, 2, Id::for_test(91)));
    let mut bottle = Item::new(Id::for_test(92), ItemKind::Bottle);
    let liquid = Some(Liquid::Water(water));
    bottle.form = Some(ItemForm::Bottle(BottleForm::new(liquid, 1)));
    give(test, bottle);
    if cauldron {
        give(test, Item::new(Id::for_test(93), ItemKind::Cauldron));
    }
    let order = BrewOrder {
        ointment: Id::for_test(91),
        water: Id::for_test(92),
    };
    (order, def.key)
}

fn landed(test: &mut TestClient) -> Option<Landed> {
    received(test)
        .into_iter()
        .find_map(|message| match message {
            ServerMessage::AbilityLanded { landed, .. } => Some(landed),
            _ => None,
        })
}

/// The one bottle carried.
fn bottle(test: &TestClient) -> BottleForm {
    let found = test
        .client
        .inventory
        .items
        .iter()
        .map(|placed| &placed.item);
    let mut bottles = found.filter_map(Item::bottle);
    bottles.next().expect("a bottle").clone()
}

#[test]
fn brewing_turns_the_water_into_a_potion() {
    let mut test = TestClient::new();
    test.enter_a_castle();
    let (order, key) = carrying(&mut test, 5, Water::Plain, true);
    test.send(&[ClientMessage::Brew(order)]);
    assert!(test.client.fight.pending.is_some(), "it winds up");
    run(&mut test, BREWING, false);
    assert_eq!(test.client.inventory.count_of(ItemKind::Ointment), 1);
    assert!(bottle(&test).is_potion());
    assert!(matches!(landed(&mut test), Some(Landed::Brewed(Some(_)))));
    assert!(test.client.sheet.herbs.knows_effect(&key, 0), "taught");
    let recipe = test.client.sheet.recipes.iter().next().unwrap();
    assert_eq!(recipe.brewed.as_ref().unwrap().water, Water::Plain);
    for path in [ALCHEMY, HERBALISM] {
        let skill = SkillId::find(path).unwrap();
        assert!(test.client.sheet.skills.practice(skill) > 0, "{path}");
    }
}

#[test]
fn a_brew_with_no_potion_wastes_both() {
    let mut test = TestClient::new();
    test.enter_a_castle();
    let (order, key) = carrying(&mut test, 2, Water::Plain, true);
    test.send(&[ClientMessage::Brew(order)]);
    run(&mut test, BREWING, false);
    assert_eq!(test.client.inventory.count_of(ItemKind::Ointment), 1);
    assert!(bottle(&test).is_empty(), "the water is gone");
    assert!(matches!(landed(&mut test), Some(Landed::Brewed(None))));
    assert!(!test.client.sheet.herbs.knows_effect(&key, 0));
    let skill = SkillId::find(ALCHEMY).unwrap();
    assert!(test.client.sheet.skills.practice(skill) > 0);
}

#[test]
fn nothing_is_brewed_without_a_cauldron() {
    let mut test = TestClient::new();
    test.enter_a_castle();
    let (order, _) = carrying(&mut test, 5, Water::Plain, false);
    test.send(&[ClientMessage::Brew(order)]);
    assert!(test.client.fight.pending.is_none());
}
