//! Press Oil on a moromi (cooking plan PR 12): pressed whole into bottles of
//! soy sauce at its quality, only with enough empty bottles.

use shared::ability::Ability;
use shared::food::{FoodForm, Rest, dev_food, soy_sauce_bottles};
use shared::id::Id;
use shared::item::{Item, ItemKind, Quality, StackLimits};
use shared::item_form::ItemForm;
use shared::potion::{BottleForm, Liquid};
use shared::protocol::{ClientMessage, Landed, ServerMessage};
use shared::rng::Seed;

use super::tests::{received, run};
use crate::connections::tests::TestClient;

/// Ticks a press takes untrained, and a few more.
const ONE_PRESS: u64 = 15 * 20 + 2;

/// Give Item's moromi at Rare.
fn moromi() -> Item {
    let form: FoodForm = (0..200)
        .filter_map(|seed| dev_food(ItemKind::Intermediate, Quality::Rare, Seed::new(seed)))
        .find(|form| form.rest.as_ref().map(|rest| rest.kind) == Some(Rest::Moromi))
        .expect("a moromi sample");
    let mut item = Item::new(Id::for_test(91), ItemKind::Intermediate);
    item.quality = Some(Quality::Rare);
    item.form = Some(ItemForm::Food(Box::new(form)));
    item
}

/// A moromi, `empty` empty bottles and an oil press: the moromi's bottles.
fn carrying(test: &mut TestClient, empty: u32) -> u32 {
    let moromi = moromi();
    let bottles = soy_sauce_bottles(&moromi);
    let mut bottle = Item::new(Id::for_test(92), ItemKind::Bottle);
    bottle.form = Some(ItemForm::Bottle(BottleForm::new(None, 1)));
    bottle.count = empty;
    let press = Item::new(Id::for_test(93), ItemKind::OilPress);
    let new_id = &mut shared::id::stand_ins();
    for item in [moromi, bottle, press] {
        let given = test.client.inventory.give(item, StackLimits::NONE, new_id);
        assert!(given.is_ok());
    }
    bottles
}

fn press() -> ClientMessage {
    ClientMessage::UseAbility {
        ability: Ability::PressOil,
        target: Some(Id::for_test(91)),
    }
}

/// The bottles of soy sauce carried, with their quality.
fn soy_sauce(test: &TestClient) -> Vec<(u32, Option<Quality>)> {
    let items = test
        .client
        .inventory
        .items
        .iter()
        .map(|placed| &placed.item);
    items
        .filter(|item| {
            let liquid = item.bottle().and_then(|bottle| bottle.liquid.clone());
            liquid == Some(Liquid::SoySauce)
        })
        .map(|item| (item.count(), item.quality))
        .collect()
}

#[test]
fn a_moromi_is_pressed_whole_into_soy_sauce() {
    let mut test = TestClient::new();
    test.enter_a_castle();
    let bottles = carrying(&mut test, 12);
    assert!(bottles > 1);
    test.send(&[press()]);
    assert!(test.client.fight.pending.is_some());
    run(&mut test, ONE_PRESS, false);
    let made = soy_sauce(&test);
    assert_eq!(made.iter().map(|(count, _)| count).sum::<u32>(), bottles);
    assert!(
        made.iter()
            .all(|(_, quality)| *quality == Some(Quality::Rare))
    );
    assert_eq!(test.client.inventory.count_of(ItemKind::Intermediate), 0);
    let landed = received(&mut test).iter().any(|message| {
        matches!(
            message,
            ServerMessage::AbilityLanded {
                landed: Landed::SoySauce { bottles: b },
                ..
            } if *b == bottles
        )
    });
    assert!(landed);
    assert!(test.client.fight.pending.is_none(), "nothing left to press");
}

#[test]
fn not_without_bottles_for_all_of_it() {
    let mut test = TestClient::new();
    test.enter_a_castle();
    let bottles = carrying(&mut test, 1);
    assert!(bottles > 1);
    test.send(&[press()]);
    assert!(test.client.fight.pending.is_none());
}
