//! Grind's pieces: the ointment item, the order checked against what is
//! carried, recipes kept, and ointment on the skin.

use super::*;
use crate::bonus::{Amount, Source, Stat as BonusStat};
use crate::herb::{Herbs, Stat};
use crate::id::Id;
use crate::item::{Inventory, Item, ItemKind, Quality, StackLimits};
use crate::item_form::ItemForm;
use crate::potion::{BottleForm, Contents, Dose, Liquid, Oil};
use crate::vitals::{PER_POINT, Vital};

fn contents(doses: &[(Stat, i16)]) -> Contents {
    Contents::new(doses.iter().map(|&(stat, step)| Dose { stat, step }))
}

fn herb(herbs: &Herbs, key: &str, count: u32, id: u128) -> Item {
    let def = herbs.get(key).expect("a herb");
    crate::herb::new_item(def, count, Id::for_test(id), None)
}

fn oil(oil: Oil, count: u32, id: u128) -> Item {
    let mut bottle = Item::new(Id::for_test(id), ItemKind::Bottle);
    let liquid = Some(Liquid::Oil(oil));
    bottle.form = Some(ItemForm::Bottle(BottleForm::new(liquid, 3)));
    bottle.count = count;
    bottle
}

fn carrying(items: Vec<Item>) -> Inventory {
    let mut inventory = Inventory::default();
    let stand_ins = &mut crate::id::stand_ins();
    for item in items {
        inventory
            .give(item, StackLimits::default(), stand_ins)
            .unwrap();
    }
    inventory
}

/// Two built-in herbs' keys.
fn two_herbs(herbs: &Herbs) -> (String, String) {
    let mut keys = herbs.iter().map(|def| def.key.clone());
    (keys.next().unwrap(), keys.next().unwrap())
}

#[test]
fn ointment_stacks_by_contents_and_has_their_quality() {
    let strong = contents(&[(Stat::Strength, 12)]);
    let one = new_ointment(strong.clone(), 2, Id::for_test(1));
    assert_eq!(one.kind, ItemKind::Ointment);
    assert_eq!(one.count(), 2);
    assert_eq!(one.quality, Some(quality(&strong)));
    assert_eq!(one.ointment().unwrap().contents, strong);
    assert_eq!(one.name(), "Ointment");
    let same = new_ointment(strong, 1, Id::for_test(2));
    let other = new_ointment(contents(&[(Stat::Will, 12)]), 1, Id::for_test(3));
    let inventory = carrying(vec![one, same, other]);
    let stacks: Vec<u32> = inventory.items.iter().map(|p| p.item.count()).collect();
    assert_eq!(stacks.len(), 2, "the same contents stack: {stacks:?}");
}

#[test]
fn an_order_is_checked_against_what_is_carried() {
    let herbs = Herbs::built_in();
    let (a, b) = two_herbs(&herbs);
    let carried = carrying(vec![
        herb(&herbs, &a, 3, 1),
        herb(&herbs, &b, 1, 2),
        oil(Oil::Nut, 1, 3),
    ]);
    let skills = BrewSkills::default();
    let stack = |key: &str| {
        let stacks = carried.herb_stacks();
        stacks.iter().find(|s| s.herb == key).unwrap().stack
    };
    let oil_stack = carried.oil_stacks()[0].stack;
    let order = GrindOrder {
        herbs: vec![stack(&a), stack(&b)],
        oil: oil_stack,
        doses: 1,
    };
    let (made, used) = carried.check_grind(&order, &herbs, skills).unwrap();
    assert_eq!(used, Oil::Nut);
    assert_eq!(made.doses, 1);
    let more = GrindOrder {
        doses: 2,
        ..order.clone()
    };
    let refused = carried.check_grind(&more, &herbs, skills);
    assert!(
        refused.is_err(),
        "only one of the second herb, and Alchemy allows one dose"
    );
    let no_oil = GrindOrder {
        oil: Id::for_test(99),
        ..order.clone()
    };
    assert_eq!(
        carried.check_grind(&no_oil, &herbs, skills),
        Err(GrindRefused::Oil)
    );
    let twice = GrindOrder {
        herbs: vec![stack(&a), stack(&a)],
        ..order
    };
    assert_eq!(
        carried.check_grind(&twice, &herbs, skills),
        Err(GrindRefused::Rules(Refused::SameHerb))
    );
}

#[test]
fn recipes_keep_the_latest_of_each_mix_first() {
    let mut recipes = Recipes::default();
    let made = contents(&[(Stat::Strength, 4)]);
    recipes.remember(Recipe::new(
        ["b".into(), "a".into()],
        Oil::Nut,
        made.clone(),
    ));
    recipes.remember(Recipe::new(["c".into()], Oil::Seed, made.clone()));
    let again = contents(&[(Stat::Strength, 6)]);
    recipes.remember(Recipe::new(
        ["a".into(), "b".into()],
        Oil::Nut,
        again.clone(),
    ));
    let kept: Vec<_> = recipes.iter().cloned().collect();
    assert_eq!(kept.len(), 2);
    assert_eq!(kept[0].herbs, vec!["a".to_owned(), "b".to_owned()]);
    assert_eq!(kept[0].made, again, "what it made last");
    for n in 0..MOST_RECIPES + 5 {
        recipes.remember(Recipe::new([n.to_string()], Oil::Nut, made.clone()));
    }
    assert_eq!(recipes.iter().count(), MOST_RECIPES);
}

#[test]
fn on_the_skin_a_quarter_lasts_and_vitals_come_at_once() {
    let ointment = contents(&[(Stat::Strength, 8), (Stat::Health, 4), (Stat::Will, -3)]);
    let applied = skin_effect(&ointment, 100);
    let effect = applied.effect.expect("an effect on Strength");
    assert_eq!(effect.source, Source::Ointment);
    assert_eq!(effect.start, 100);
    let lasts = duration_ticks(quality(&ointment));
    assert_eq!(effect.end, Some(100 + lasts));
    assert_eq!(effect.bonuses.len(), 1, "Will's −3 is 0 on the skin");
    let bonus = &effect.bonuses[0];
    assert!(matches!(bonus.stat, BonusStat::Attribute(_)));
    assert_eq!(bonus.amount, Amount::Flat(2 * ATTRIBUTE_PER_STEP));
    let health = VITAL_PER_STEP * PER_POINT as i32;
    assert_eq!(applied.vitals, vec![(Vital::Health, health)]);
}

#[test]
fn give_item_rolls_an_ointment() {
    let mut item = Item::new(Id::for_test(1), ItemKind::Ointment);
    item.quality = Some(Quality::Common);
    let seed = crate::rng::Seed::new(5);
    let request = crate::item_form::RollRequest {
        effects: Some(2),
        ..crate::item_form::RollRequest::NONE
    };
    crate::item_form::give_form_to(&mut item, None, seed, &request, &mut || Id::for_test(2))
        .unwrap();
    let form = item.ointment().expect("an ointment");
    assert_eq!(form.contents.doses().len(), 2);
    assert_eq!(item.quality, Some(quality(&form.contents)));
}
