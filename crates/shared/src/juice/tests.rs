use super::*;
use crate::bonus::Source;
use crate::cask::{CaskForm, CaskSize};
use crate::fruit::{FruitTag, Fruits};
use crate::id::Id;
use crate::item::{Inventory, Item, ItemKind, StackLimits};
use crate::item_form::ItemForm;
use crate::material::Material;
use crate::potion::{BottleForm, Liquid, Water};
use crate::pour::{PourRefused, pour};
use crate::vessel::{VesselForm, VesselKind};

fn def(key: &str) -> &'static crate::fruit::FruitDef {
    Fruits::built_in().get(key).expect("a fruit")
}

fn fruit(key: &str, count: u32, id: u128) -> Item {
    crate::fruit::new_item(def(key), count, Id::for_test(id), None)
}

fn vessel(id: u128, form: VesselForm) -> Item {
    Item {
        form: Some(ItemForm::Vessel(form)),
        ..Item::new(Id::for_test(id), ItemKind::Vessel)
    }
}

fn cask(id: u128, form: CaskForm) -> Item {
    Item {
        form: Some(ItemForm::Cask(form)),
        ..Item::new(Id::for_test(id), ItemKind::Cask)
    }
}

fn carrying(items: Vec<Item>) -> Inventory {
    let mut inventory = Inventory::default();
    let stand_ins = &mut crate::id::stand_ins();
    for item in items {
        inventory.give(item, StackLimits::NONE, stand_ins).unwrap();
    }
    inventory
}

fn juice(key: &str) -> Liquid {
    Liquid::Juice(key.to_owned())
}

#[test]
fn juice_comes_by_weight_and_kcal() {
    // An apple, 94 kcal in 200 g: 88% water, four fifths of it pressed out.
    assert_eq!(ml_from(def("apple"), 200), 141);
    assert_eq!(ml_from(def("apple"), 1000), 705);
    // A litre of apple juice, about cider's 6% when fermented (÷ 68).
    assert_eq!(kcal_per_litre(def("apple")), 400);
    // Grapes are sweeter.
    assert!(kcal_per_litre(def("grape")) > 550);
}

#[test]
fn only_juicy_fruit_can_be_juiced() {
    assert!(juicable(def("apple")));
    assert!(!juicable(def("walnut")), "a nut");
    assert!(!juicable(def("date")), "too dry");
    assert_eq!(ml_from(def("date"), 1000), 0);
    assert!(juice_fruits().count() > 30);
    assert!(juice_fruits().all(|def| ml_from(def, def.grams) > 0));
}

#[test]
fn a_serving_of_juice_gives_mana_for_thirty_seconds() {
    let second = crate::energy::ACTION_TICKS_PER_SECOND as u64;
    let lasts = |liquid: &Liquid, ml| {
        drink_effect(liquid, ml, 100).map(|effect| (effect.end.unwrap() - 100) / second)
    };
    assert_eq!(lasts(&juice("apple"), 250), Some(30));
    assert_eq!(lasts(&Liquid::Cordial("apple".into()), 250), Some(20));
    // A bottle's 100 ml.
    assert_eq!(lasts(&juice("apple"), 100), Some(12));
    assert_eq!(lasts(&Liquid::Water(Water::Plain), 250), None);
    assert_eq!(lasts(&juice("apple"), 5), None, "under a second");
    let effect = drink_effect(&juice("apple"), 250, 0).unwrap();
    assert_eq!(effect.source, Source::Juice(FruitTag::of("apple")));
    assert!(
        effect
            .source
            .replaced_by(Source::Cordial(FruitTag::of("pear")))
    );
}

#[test]
fn juice_and_water_pour_into_cordial_never_more_water_than_juice() {
    let jug = |id, liquid, ml| vessel(id, VesselForm::new(VesselKind::Jug).holding(liquid, ml));
    let water = Some(Liquid::Water(Water::Plain));
    // A litre of water into 250 ml of juice: only 250 ml goes.
    let poured = pour(
        &jug(1, water.clone(), 1000),
        &jug(2, Some(juice("apple")), 250),
        false,
    );
    let poured = poured.unwrap();
    assert_eq!(poured.ml, 250);
    let cordial = poured.into.vessel().unwrap();
    assert_eq!(cordial.liquid, Some(Liquid::Cordial("apple".into())));
    assert_eq!(cordial.ml, 500);
    assert_eq!(poured.from.vessel().unwrap().ml, 750);
    // Juice into water: at least as much juice as the water there.
    let little = pour(
        &jug(3, Some(juice("apple")), 200),
        &jug(4, water.clone(), 300),
        false,
    );
    assert_eq!(little, Err(PourRefused::TooWatery));
    let enough = pour(
        &jug(3, Some(juice("apple")), 300),
        &jug(4, water, 300),
        false,
    )
    .unwrap();
    assert_eq!(enough.into.vessel().unwrap().ml, 600);
    // Salt water doesn't mix.
    let salt = Some(Liquid::Water(Water::Salt));
    let refused = pour(
        &jug(5, salt, 100),
        &jug(6, Some(juice("apple")), 300),
        false,
    );
    assert_eq!(refused, Err(PourRefused::Different));
}

#[test]
fn juice_goes_into_a_cask_or_vessel_that_takes_it() {
    let pin = cask(9, CaskForm::new(CaskSize::Pin, Material::Oak));
    let pear_cup = vessel(
        8,
        VesselForm::new(VesselKind::Cup).holding(Some(juice("pear")), 100),
    );
    let apple_cup = vessel(
        7,
        VesselForm::new(VesselKind::Cup).holding(Some(juice("apple")), 100),
    );
    let inventory = carrying(vec![fruit("apple", 10, 1), pin.clone(), pear_cup]);
    assert_eq!(inventory.juice_target("apple"), Some(pin.id));
    assert!(inventory.can_press_juice("apple"));
    assert!(!inventory.can_press_juice("pear"), "none carried");
    let inventory = carrying(vec![fruit("apple", 10, 1), pin, apple_cup.clone()]);
    assert_eq!(
        inventory.juice_target("apple"),
        Some(apple_cup.id),
        "started first"
    );
    let choices = inventory.juice_choices();
    assert_eq!(choices.len(), 1);
    assert_eq!((choices[0].count, choices[0].ml), (10, 1411));
    assert_eq!(choices[0].room, 150);
    let nothing = carrying(vec![fruit("apple", 10, 1)]);
    assert!(nothing.juice_choices().is_empty(), "nothing takes it");
}

#[test]
fn pressed_juice_fills_and_opens() {
    let pin = cask(9, CaskForm::new(CaskSize::Pin, Material::Oak));
    let filled = with_juice(&pin, juice("apple"), 1400);
    let form = filled.cask().unwrap();
    assert_eq!((form.ml, form.opened), (1400, true));
    assert_eq!(form.liquid, Some(juice("apple")));
    assert_eq!(form.name(), "Oak Pin of Apple Juice");
}

#[test]
fn bottles_of_drink_are_drunk_whole() {
    let bottle = |liquid| Item {
        form: Some(ItemForm::Bottle(BottleForm::new(liquid, 0))),
        ..Item::new(Id::for_test(1), ItemKind::Bottle)
    };
    assert!(bottle(Some(juice("apple"))).is_drinkable_bottle());
    assert!(bottle(Some(Liquid::Water(Water::Cave))).is_drinkable_bottle());
    assert!(!bottle(None).is_drinkable_bottle());
    assert!(!bottle(Some(Liquid::Oil(crate::potion::Oil::Nut))).is_drinkable_bottle());
    assert!(crate::bonus::usable_on_self(&bottle(Some(juice("apple")))));
}

#[test]
fn give_items_juice_is_a_juicy_fruit() {
    let mut rng = crate::rng::Seed::new(4).rng();
    for _ in 0..20 {
        assert!(juicable(def(&juice_fruit(&mut rng))));
    }
}
