use super::*;
use crate::bonus::{self, Source};
use crate::energy::ACTION_TICKS_PER_SECOND;
use crate::herb::{Herbs, eating_herb};
use crate::id::{Id, stand_ins};
use crate::item::{Item, ItemKind, Quality, StackLimits};

const SECOND: u64 = ACTION_TICKS_PER_SECOND as u64;

fn def(key: &str) -> &'static FruitDef {
    Fruits::built_in().get(key).expect("a built-in fruit")
}

#[test]
fn the_table_has_every_fruit_nut_and_herb_fruit() {
    let fruits = Fruits::built_in();
    assert_eq!(fruits.len(), 101);
    let nuts = fruits.iter().filter(|def| def.set == FruitSet::Nut).count();
    assert_eq!(nuts, 22);
    assert_eq!(fruits.items().count(), 95);
    let herbs = Herbs::built_in();
    for fruit in fruits.iter().filter(|def| def.herb) {
        let herb = herbs.get(&fruit.key).expect("a herb of the fruit's key");
        assert_eq!(herb.fruit.as_deref(), Some(fruit.key.as_str()));
    }
    for herb in herbs.iter() {
        if let Some(key) = &herb.fruit {
            assert!(fruits.get(key).is_some_and(|fruit| fruit.herb), "{key}");
        }
    }
}

#[test]
fn tags_find_their_fruit() {
    for fruit in Fruits::built_in().iter() {
        let found = Fruits::built_in().by_tag(FruitTag::of(&fruit.key));
        assert_eq!(found.map(|def| &def.key), Some(&fruit.key));
    }
}

#[test]
fn an_apple_heals_23_health_over_23_seconds() {
    let apple = def(APPLE);
    assert_eq!(healing(apple), 23);
    let effects = eating(apple, 100);
    assert_eq!(effects.len(), 1);
    assert_eq!(effects[0].source, Source::EatenFruit(FruitTag::of(APPLE)));
    assert_eq!(effects[0].end, Some(100 + ticks(apple)));
    assert_eq!(ticks(apple) / SECOND, 23);
    assert!(effects[0].ends_on.moving && effects[0].ends_on.hurt);
}

#[test]
fn exceptional_fruit_heals_half_as_long_again() {
    let blackcurrant = def("blackcurrant");
    assert!(blackcurrant.exceptional);
    let plain = u64::from(blackcurrant.kcal) * 3 / 2 / 4;
    assert_eq!(ticks(blackcurrant), plain * SECOND);
}

#[test]
fn effects_last_as_long_as_the_healing_and_end_on_nothing() {
    let kola = def("kola_nut");
    let effects = eating(kola, 0);
    let fruit = effects
        .iter()
        .find(|e| e.source == Source::FruitEffects(FruitTag::of("kola_nut")));
    let fruit = fruit.expect("a stimulant");
    assert_eq!(fruit.end, Some(ticks(kola)));
    assert_eq!(fruit.ends_on, bonus::EndsOn::NOTHING);
    assert_eq!(fruit.bonuses.len(), 2);
}

#[test]
fn harmful_raw_fruit_only_harms() {
    let acorn = def("acorn");
    assert!(acorn.harmful_raw);
    assert_eq!(healing(acorn), 0);
    let effects = eating(acorn, 0);
    assert_eq!(effects.len(), 1);
    assert_eq!(
        effects[0].source,
        Source::FruitEffects(FruitTag::of("acorn"))
    );
    let drains = effects[0].bonuses[0].stat == bonus::Stat::Drain(crate::vitals::Vital::Health);
    assert!(drains, "{:?}", effects[0].bonuses);
    assert!(
        effects[0].end > Some(0),
        "as long as its healing would have been"
    );
}

#[test]
fn a_whole_fruit_cuts_into_its_servings() {
    let watermelon = def("watermelon");
    let whole = new_item(watermelon, 1, Id::for_test(1), None);
    assert!(whole.fruit().unwrap().is_whole(watermelon));
    let (eaten, rest) = cut(&whole, &mut stand_ins()).expect("whole");
    let rest = rest.expect("servings left");
    assert_eq!(rest.count, u32::from(watermelon.servings) - 1);
    assert!(eaten.fruit().unwrap().serving && rest.fruit().unwrap().serving);
    assert!(eaten.total_weight() * u32::from(watermelon.servings) >= watermelon.grams);
    assert!(
        cut(&eaten, &mut stand_ins()).is_none(),
        "a serving is not cut again"
    );
    let apple = new_item(def(APPLE), 1, Id::for_test(2), None);
    assert!(cut(&apple, &mut stand_ins()).is_none());
    // A serving heals a share of the whole.
    assert_eq!(healing(watermelon), (ticks(watermelon) / SECOND) as u32);
    assert!(ticks(watermelon) < u64::from(watermelon.kcal) * SECOND / 4);
}

#[test]
fn fruit_stacks_by_fruit_whatever_its_look() {
    let apple = def(APPLE);
    let one = new_item(apple, 2, Id::for_test(1), None);
    let two = new_item(apple, 3, Id::for_test(2), None);
    assert_ne!(one.fruit().unwrap().look, two.fruit().unwrap().look);
    assert!(StackLimits::default().room(&one, &two) > 0);
    let pear = new_item(def("pear"), 1, Id::for_test(3), None);
    assert_eq!(StackLimits::default().room(&one, &pear), 0);
    assert_eq!(one.quality, None, "fruit has no quality");
    assert_eq!(one.name(), "Apple");
}

#[test]
fn old_apples_load_as_apple_fruit() {
    let mut old = Item::new(Id::for_test(7), ItemKind::Fruit);
    old.above.push(Item::new(Id::for_test(8), ItemKind::Fruit));
    let text = ron::to_string(&old).unwrap().replace("Fruit", "Apple");
    assert!(text.contains("Apple"));
    let loaded: Item = ron::from_str(&text).unwrap();
    assert_eq!(loaded.kind, ItemKind::Fruit);
    assert_eq!(loaded.fruit().map(|form| form.fruit.as_str()), Some(APPLE));
    assert_eq!(loaded.count, 2, "an old stack of apples is counted");
}

#[test]
fn herbs_bearing_fruit_give_their_dose_and_the_fruit() {
    let bilberry = Herbs::built_in().get("bilberry").unwrap().clone();
    let effects = eating_herb(&bilberry, Quality::Common, 0, 0);
    let sources: Vec<Source> = effects.iter().map(|e| e.source).collect();
    let tag = FruitTag::of("bilberry");
    assert!(sources.contains(&Source::EatenFruit(tag)));
    assert!(sources.contains(&Source::FruitEffects(tag)));
    assert_eq!(
        sources.contains(&Source::RawHerb),
        crate::herb::RAW_DOSE_WITH_FRUIT
    );
    let roseroot = Herbs::built_in().get("roseroot").unwrap().clone();
    let effects = eating_herb(&roseroot, Quality::Common, 0, 0);
    assert_eq!(effects.len(), 1, "a herb without fruit: its dose");
}

#[test]
fn every_fruit_has_a_dev_choice() {
    assert_eq!(
        dev_fruit(Some(APPLE), crate::rng::Seed::new(1)).map(|d| &d.key[..]),
        Some(APPLE)
    );
    assert!(
        dev_fruit(Some("bilberry"), crate::rng::Seed::new(1)).is_none(),
        "a herb"
    );
    assert!(dev_fruit(None, crate::rng::Seed::new(1)).is_some_and(FruitDef::is_item));
}
