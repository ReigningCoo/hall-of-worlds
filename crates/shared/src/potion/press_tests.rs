use super::*;
use crate::fruit::Fruits;
use crate::herb::Herbs;
use crate::id::Id;
use crate::item::{Inventory, Item, ItemKind, StackLimits};
use crate::item_form::ItemForm;
use crate::skill::{SkillId, Skills};
use crate::vegetable::Vegetables;

fn fruit(key: &str, count: u32, id: u128) -> Item {
    let def = Fruits::built_in().get(key).expect("a fruit");
    crate::fruit::new_item(def, count, Id::for_test(id), None)
}

fn vegetable(key: &str, count: u32, id: u128) -> Item {
    let def = Vegetables::built_in().get(key).expect("a vegetable");
    crate::vegetable::new_item(def, count, Id::for_test(id), None)
}

fn herb(herbs: &Herbs, key: &str, count: u32, id: u128) -> Item {
    let def = herbs.get(key).expect("a herb");
    crate::herb::new_item(def, count, Id::for_test(id), None)
}

fn empty_bottles(count: u32, id: u128) -> Item {
    let mut bottle = Item::new(Id::for_test(id), ItemKind::Bottle);
    bottle.form = Some(ItemForm::Bottle(BottleForm::new(None, 3)));
    bottle.count = count;
    bottle
}

fn carrying(items: Vec<Item>) -> Inventory {
    let mut inventory = Inventory::default();
    let stand_ins = &mut crate::id::stand_ins();
    for item in items {
        inventory.give(item, StackLimits::NONE, stand_ins).unwrap();
    }
    inventory
}

#[test]
fn nuts_seeds_grain_and_vegetables_give_their_oils() {
    let herbs = Herbs::built_in();
    let oil = |item: &Item| Pressing::of(item, &herbs).and_then(|p| p.oil(&herbs));
    assert_eq!(oil(&fruit("walnut", 1, 1)), Some(Oil::Nut));
    assert_eq!(oil(&fruit("apple", 1, 2)), None, "not a nut");
    assert_eq!(oil(&vegetable("lentil", 1, 3)), Some(Oil::Seed));
    assert_eq!(oil(&vegetable("carrot", 1, 4)), Some(Oil::Neutral));
    assert_eq!(oil(&vegetable("wheat", 1, 8)), Some(Oil::Grain));
    assert_eq!(oil(&herb(&herbs, "mustard", 1, 5)), Some(Oil::Seed));
    assert_eq!(oil(&herb(&herbs, "chamomile", 1, 6)), None, "not seeds");
    assert_eq!(oil(&empty_bottles(1, 7)), None);
}

#[test]
fn a_bottle_takes_fifteen_seconds_down_to_eleven() {
    let mut skills = Skills::default();
    assert_eq!(press_seconds(&skills), 15);
    let alchemy = SkillId::find(crate::ability::ALCHEMY).expect("Alchemy");
    skills.set_percent(alchemy, 100);
    assert_eq!(press_seconds(&skills), 11);
    assert_eq!(press_ticks(&skills), 11 * 20);
}

#[test]
fn choices_need_a_bottles_worth_and_an_empty_bottle() {
    let herbs = Herbs::built_in();
    let walnut = fruit("walnut", 1, 1).own_weight();
    let enough = grams_per_bottle(Oil::Nut).div_ceil(walnut);
    let short = carrying(vec![fruit("walnut", enough - 1, 1), empty_bottles(2, 2)]);
    assert!(short.press_choices(&herbs).is_empty(), "not quite enough");
    let walnuts = Pressing::Fruit("walnut".into());
    assert_eq!(short.can_press(&walnuts, &herbs), None);

    let no_bottle = carrying(vec![fruit("walnut", enough * 3, 1)]);
    assert!(no_bottle.press_choices(&herbs).is_empty());

    let two = carrying(vec![fruit("walnut", enough * 3, 1), empty_bottles(2, 2)]);
    let choices = two.press_choices(&herbs);
    assert_eq!(choices.len(), 1);
    assert_eq!(choices[0].pressing, walnuts);
    assert_eq!(choices[0].liquid, Liquid::Oil(Oil::Nut));
    assert_eq!(choices[0].count, enough * 3);
    assert_eq!(choices[0].bottles, 2, "as many as the empty bottles");
    assert_eq!(two.can_press(&walnuts, &herbs), Some(Oil::Nut));
    assert_eq!(two.stack_to_press(&walnuts), Some(choices[0].stack));
}

#[test]
fn a_pressed_bottle_holds_the_oil_and_keeps_its_look() {
    let bottle = pressed(empty_bottles(1, 1), Oil::Seed);
    assert_eq!(bottle.bottle().unwrap().look, 3);
    assert_eq!(bottle.name(), "Bottle of Seed Oil");
}
