use crate::bottle_rack::{BottleRack, BottleRackKind, RackBottles};
use crate::fermentation::{
    Base, Drink, DrinkKind, GAME_DAY_SECS, Sweetness, glass_aged, glass_cap, glass_step_secs,
};
use crate::id::Id;
use crate::item::{Item, ItemKind, Quality};
use crate::item_form::ItemForm;
use crate::potion::{BottleForm, Liquid};
use crate::vessel::{VesselForm, VesselKind};

const DAY: u64 = GAME_DAY_SECS;

fn drink(fruit: &str) -> Liquid {
    Liquid::Drink(Box::new(Drink {
        base: Base::Fruit(fruit.into()),
        strength: 120,
        sweetness: Sweetness::Dry,
        quality: Quality::Common,
        vinegar: 0,
    }))
}

fn wine_bottle(n: u128, liquid: Liquid, sealed: bool) -> Item {
    let form = VesselForm::new(VesselKind::WineBottle).full_of(liquid);
    let form = if sealed { form.sealed() } else { form };
    Item {
        form: Some(ItemForm::Vessel(form)),
        ..Item::new(Id::for_test(n), ItemKind::Vessel)
    }
}

fn quality(item: &Item) -> Quality {
    let liquid = match &item.form {
        Some(ItemForm::Vessel(vessel)) => vessel.liquid.as_ref(),
        Some(ItemForm::Bottle(bottle)) => bottle.liquid.as_ref(),
        _ => None,
    };
    match liquid {
        Some(Liquid::Drink(drink)) => drink.quality,
        other => panic!("not a drink: {other:?}"),
    }
}

#[test]
fn glass_ages_wine_two_steps_fruit_wine_one_and_ale_none() {
    assert_eq!(glass_cap(DrinkKind::Wine), 2);
    assert_eq!(glass_cap(DrinkKind::FruitWine), 1);
    for kind in [DrinkKind::Ale, DrinkKind::Cider, DrinkKind::Perry] {
        assert_eq!(glass_cap(kind), 0);
        assert_eq!(
            glass_aged(Quality::Common, kind, 1000 * DAY),
            Quality::Common
        );
    }
    let step = glass_step_secs(DrinkKind::Wine);
    assert_eq!(step, 60 * DAY);
    assert_eq!(
        glass_aged(Quality::Common, DrinkKind::Wine, step - 1),
        Quality::Common
    );
    assert_eq!(
        glass_aged(Quality::Common, DrinkKind::Wine, step),
        Quality::Uncommon
    );
    assert_eq!(
        glass_aged(Quality::Common, DrinkKind::Wine, 10 * step),
        Quality::Rare
    );
}

#[test]
fn a_sealed_wine_bottle_carried_ages_by_playtime() {
    let mut item = wine_bottle(1, drink("grape"), true);
    let step = glass_step_secs(DrinkKind::Wine);
    let settled = item.age_casks(step as u32, 0);
    assert!(settled.seen);
    assert_eq!(quality(&item), Quality::Uncommon);
    // Opened, it ages no more.
    let mut open = wine_bottle(2, drink("grape"), false);
    let _ = open.age_casks(step as u32, 0);
    assert_eq!(quality(&open), Quality::Common);
}

#[test]
fn bottles_on_the_ground_go_by_world_time_and_count_it_when_picked_up() {
    let step = glass_step_secs(DrinkKind::Wine);
    let mut item = wine_bottle(1, drink("grape"), true);
    item.lay_casks_down(100);
    let _ = item.settle_lying_casks(100 + step / 2);
    assert_eq!(quality(&item), Quality::Common);
    // Picked up: the rest of its time lying counts, not the playtime.
    let _ = item.age_casks(0, 100 + step);
    assert_eq!(quality(&item), Quality::Uncommon);
}

#[test]
fn a_small_bottle_of_wine_sealed_from_a_cask_ages_too_and_stacks_whatever_its_time() {
    let bottle = |n| Item {
        form: Some(ItemForm::Bottle(
            BottleForm::new(Some(drink("plum")), n as u64).with_seal(true),
        )),
        ..Item::new(Id::for_test(n), ItemKind::Bottle)
    };
    let mut stack = Item {
        count: 2,
        ..bottle(1)
    };
    let _ = stack.age_casks(glass_step_secs(DrinkKind::FruitWine) as u32, 0);
    assert_eq!(quality(&stack), Quality::Uncommon);
    // Two that aged apart still stack (a stack keeps its bottom's time).
    let mut fresh = bottle(3);
    let _ = fresh.age_casks(1, 0);
    let mut older = bottle(4);
    let _ = older.age_casks(100, 0);
    assert!(fresh.looks_like(&older));
}

#[test]
fn on_a_rack_bottles_age_ten_percent_faster() {
    let step = glass_step_secs(DrinkKind::Wine);
    let rack = BottleRack {
        kind: BottleRackKind::Wine,
        length: 1,
    };
    let mut bottles = RackBottles::new(rack);
    let mut item = wine_bottle(1, drink("grape"), true);
    item.lay_casks_down(0);
    bottles.put(item).unwrap();
    // 110% of 10/11 of a step is a step.
    let now = step * 10 / 11 + 10;
    assert!(bottles.settle(now));
    assert_eq!(quality(bottles.items().next().unwrap()), Quality::Uncommon);
    let mut ground = wine_bottle(2, drink("grape"), true);
    ground.lay_casks_down(0);
    let _ = ground.settle_lying_casks(now);
    assert_eq!(quality(&ground), Quality::Common);
}
