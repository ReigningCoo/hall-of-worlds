use super::*;
use crate::bonus::{self, Source};
use crate::energy::ACTION_TICKS_PER_SECOND;
use crate::herb::Stat;
use crate::id::{Id, stand_ins};
use crate::item::{ItemKind, StackLimits};
use crate::rng::Seed;

const SECOND: u64 = ACTION_TICKS_PER_SECOND as u64;

fn def(key: &str) -> &'static VegetableDef {
    Vegetables::built_in()
        .get(key)
        .expect("a built-in vegetable")
}

#[test]
fn the_table_has_every_vegetable_pulse_and_grain() {
    let vegetables = Vegetables::built_in();
    assert_eq!(vegetables.len(), 85);
    let count = |set| vegetables.iter().filter(|def| def.set == set).count();
    assert_eq!(count(VegetableSet::Vegetable), 51);
    assert_eq!(count(VegetableSet::Pulse), 17);
    assert_eq!(count(VegetableSet::Grain), 17);
    assert_eq!(vegetables.iter().filter(|def| def.wild_only).count(), 17);
    let exceptional = vegetables.iter().filter(|def| def.exceptional).count();
    assert_eq!(exceptional, 10);
}

#[test]
fn tags_find_their_vegetable() {
    for vegetable in Vegetables::built_in().iter() {
        let found = Vegetables::built_in().by_tag(VegetableTag::of(&vegetable.key));
        assert_eq!(found.map(|def| &def.key), Some(&vegetable.key));
    }
}

#[test]
fn raw_healing_is_a_quarter_of_cooked() {
    let turnip = def("turnip");
    assert_eq!(turnip.raw, Raw::Sometimes);
    // 60 kcal: 15 s cooked, 3.75 s raw, rounded to 4.
    assert_eq!(effect_ticks(turnip), 15 * SECOND);
    assert_eq!(healing(turnip), 4);
    let effects = eating(turnip, 100);
    assert_eq!(effects.len(), 1);
    assert_eq!(
        effects[0].source,
        Source::EatenVegetable(VegetableTag::of("turnip"))
    );
    assert_eq!(effects[0].end, Some(100 + 4 * SECOND));
    assert!(effects[0].ends_on.moving && effects[0].ends_on.hurt);
    // A swede's serving: a quarter of its 323 kcal.
    assert_eq!(healing(def("swede")), 5);
}

#[test]
fn rarely_raw_and_grain_heal_nothing() {
    for key in ["burdock", "taro", "lentil", "wheat"] {
        assert_eq!(healing(def(key)), 0, "{key}");
    }
    assert!(eating(def("wheat"), 0).is_empty(), "a sheaf does nothing");
    assert!(
        (Vegetables::built_in().iter())
            .filter(|def| def.set == VegetableSet::Grain)
            .all(|def| def.unit == Unit::Sheaf && def.grams == 30)
    );
}

#[test]
fn harm_raw_is_a_real_effect_lasting_as_cooking_would_heal() {
    let cassava = def("cassava");
    let effects = eating(cassava, 0);
    assert_eq!(effects.len(), 1, "no healing raw, only harm");
    assert_eq!(
        effects[0].source,
        Source::VegetableEffects(VegetableTag::of("cassava"))
    );
    let drain = bonus::Stat::Drain(crate::vitals::Vital::Health);
    assert_eq!(effects[0].bonuses[0].stat, drain);
    assert_eq!(effects[0].end, Some(effect_ticks(cassava)));
    assert_eq!(effects[0].ends_on, bonus::EndsOn::NOTHING);
}

#[test]
fn only_raw_and_both_effects_apply_now() {
    // Carrot's Perception +1 applies raw too (owner).
    let carrot = eating(def("carrot"), 0);
    let effects = carrot
        .iter()
        .find(|e| matches!(e.source, Source::VegetableEffects(_)));
    let perception = bonus::Stat::Attribute(Stat::Perception.attribute().unwrap());
    assert_eq!(effects.expect("Perception").bonuses[0].stat, perception);
    // Barley's Toughness is for cooking.
    let barley = def("barley");
    assert_eq!(barley.effects[0].when, When::Cooked);
    assert!(eating(barley, 0).is_empty());
}

#[test]
fn exceptional_is_only_for_cooking() {
    let kale = def("kale");
    assert!(kale.exceptional);
    assert_eq!(cooked_ticks(kale), 39 * 3 / 2 / 4 * SECOND);
    assert_eq!(effect_ticks(kale), 39 / 4 * SECOND);
    assert_eq!(healing(kale), 2, "raw is never exceptional");
}

#[test]
fn pulses_are_fresh_or_dry_handfuls() {
    for vegetable in Vegetables::built_in().iter() {
        if vegetable.set != VegetableSet::Pulse {
            continue;
        }
        let fresh = ["pea", "broad_bean", "runner_bean", "winged_bean"];
        let grams = match fresh.contains(&vegetable.key.as_str()) {
            true => 100,
            false => 30,
        };
        assert_eq!(vegetable.unit, Unit::Handful, "{}", vegetable.key);
        assert_eq!(vegetable.grams, grams, "{}", vegetable.key);
    }
}

#[test]
fn a_whole_one_cuts_into_its_servings() {
    let pumpkin = def("pumpkin");
    let whole = new_item(pumpkin, 1, Id::for_test(1), None);
    assert!(whole.vegetable().unwrap().is_whole(pumpkin));
    let (eaten, rest) = cut(&whole, &mut stand_ins()).expect("whole");
    let rest = rest.expect("servings left");
    assert_eq!(rest.count, u32::from(pumpkin.servings) - 1);
    assert!(eaten.vegetable().unwrap().serving && rest.vegetable().unwrap().serving);
    assert!(eaten.total_weight() * u32::from(pumpkin.servings) >= pumpkin.grams);
    assert_eq!(eaten.name(), "Pumpkin Slice");
    assert!(cut(&eaten, &mut stand_ins()).is_none(), "not cut again");
    let yam = new_item(def("yam"), 1, Id::for_test(2), None);
    assert!(cut(&yam, &mut stand_ins()).is_some(), "a piece is cut too");
    let carrot = new_item(def("carrot"), 1, Id::for_test(3), None);
    assert!(cut(&carrot, &mut stand_ins()).is_none());
}

#[test]
fn vegetables_stack_by_vegetable_whatever_their_look() {
    let carrot = def("carrot");
    let one = new_item(carrot, 2, Id::for_test(1), None);
    let two = new_item(carrot, 3, Id::for_test(2), None);
    assert_ne!(one.vegetable().unwrap().look, two.vegetable().unwrap().look);
    assert!(StackLimits::default().room(&one, &two) > 0);
    let leek = new_item(def("leek"), 1, Id::for_test(3), None);
    assert_eq!(StackLimits::default().room(&one, &leek), 0);
    assert_eq!(one.quality, None, "vegetables have no quality");
    assert_eq!(one.kind, ItemKind::Vegetable);
    assert_eq!(one.name(), "Carrot");
}

#[test]
fn every_vegetable_has_a_dev_choice() {
    let seed = Seed::new(1);
    assert_eq!(
        dev_vegetable(Some("rice"), seed).map(|d| &d.key[..]),
        Some("rice")
    );
    assert!(dev_vegetable(Some("apple"), seed).is_none());
    assert!(dev_vegetable(None, seed).is_some());
}
