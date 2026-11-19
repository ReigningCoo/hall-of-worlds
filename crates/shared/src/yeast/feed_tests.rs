use super::*;
use crate::id::Id;
use crate::item::{Inventory, Item, ItemKind, Quality, StackLimits};
use crate::item_form::ItemForm;
use crate::potion::{BottleForm, Liquid, Water};
use crate::vessel::{VesselForm, VesselKind};

const HOUR: u32 = SECONDS_PER_HOUR;

fn yeast(n: u128, quality: Quality, unfed: u32) -> Item {
    let mut item = new_item(quality, Id::for_test(n), None);
    item.yeast_mut().unwrap().unfed = unfed;
    item
}

fn bottle(n: u128, liquid: Option<Liquid>) -> Item {
    let mut item = Item::new(Id::for_test(n), ItemKind::Bottle);
    item.form = Some(ItemForm::Bottle(BottleForm::new(liquid, 0)));
    item
}

fn jug(n: u128, ml: u32) -> Item {
    let mut item = Item::new(Id::for_test(n), ItemKind::Vessel);
    let full = VesselForm::new(VesselKind::Jug).holding(water(), ml);
    item.form = Some(ItemForm::Vessel(full));
    item
}

fn flour(n: u128, sacks: u32) -> Item {
    let grain = crate::flour::grains().next().unwrap();
    crate::flour::new_item(grain, sacks, Id::for_test(n), None).unwrap()
}

fn water() -> Option<Liquid> {
    Some(Liquid::Water(Water::Plain))
}

fn form(item: &Item) -> &YeastForm {
    item.yeast().unwrap()
}

fn supplies(flour: u32, water: u32, bottles: u32) -> Supplies {
    Supplies {
        flour,
        water,
        bottles,
    }
}

#[test]
fn a_feed_falls_in_one_of_three_windows() {
    let window = |hours: u32| form(&yeast(1, Quality::Common, hours * HOUR)).window();
    assert_eq!(window(11), Window::Early);
    assert_eq!(window(12), Window::OnTime);
    assert_eq!(window(30), Window::Late);
}

#[test]
fn an_early_feed_splits_and_uses_the_yeast() {
    let mut one = yeast(1, Quality::Common, HOUR);
    one.yeast_mut().unwrap().progress = 40;
    assert_eq!(feed(&mut one, 0, 0), Some(Fed::Split));
    let fed = form(&one);
    assert_eq!((fed.unfed, fed.progress, fed.used), (0, 0, true));
    // It can't split again until fed on time.
    assert!(!fed.feedable());
    assert_eq!(feed(&mut one, 0, 0), None);
    one.age_yeast(12 * HOUR, 0);
    assert!(matches!(feed(&mut one, 0, 99), Some(Fed::OnTime { .. })));
    assert!(form(&one).usable() && form(&one).feedable());
}

#[test]
fn an_on_time_feed_adds_progress_by_fermenting_and_may_step_up() {
    let mut one = yeast(1, Quality::Common, 15 * HOUR);
    one.yeast_mut().unwrap().used = true;
    let fed = feed(&mut one, 500, 99);
    assert_eq!(fed, Some(Fed::OnTime { stepped_up: false }));
    assert_eq!(feed_points(500), 15);
    assert_eq!(form(&one).progress, 15);
    assert!(form(&one).usable());
    one.age_yeast(15 * HOUR, 0);
    let fed = feed(&mut one, 500, 0);
    assert_eq!(fed, Some(Fed::OnTime { stepped_up: true }));
    assert_eq!(one.quality, Some(Quality::Uncommon));
    assert_eq!(form(&one).progress, 0);
}

#[test]
fn a_late_feed_only_feeds() {
    let mut one = yeast(1, Quality::Common, 0);
    one.age_yeast(30 * HOUR, 0);
    assert_eq!(one.quality, Some(Quality::Poor));
    assert_eq!(feed(&mut one, 0, 0), Some(Fed::Late));
    assert_eq!((form(&one).unfed, form(&one).progress), (0, 0));
}

#[test]
fn the_chance_grows_with_progress_and_slows_with_quality() {
    assert_eq!(step_up_chance(Quality::Poor, 30), 30);
    assert_eq!(step_up_chance(Quality::Common, 30), 15);
    assert_eq!(step_up_chance(Quality::Poor, 500), 100);
    assert_eq!(step_up_chance(BEST_BY_FEEDING, 500), 0);
}

#[test]
fn a_stack_is_fed_neediest_first_while_supplies_last() {
    let mut stack = yeast(1, Quality::Common, 13 * HOUR);
    stack.put_on(yeast(2, Quality::Common, 20 * HOUR));
    stack.put_on(yeast(3, Quality::Common, 2 * HOUR));
    let all = supplies(9, 9, 1);
    let plan = plan_feeding(&stack, all).unwrap();
    let ids: Vec<Id> = plan.yeasts.iter().map(|(id, _)| *id).collect();
    assert_eq!(ids, [Id::for_test(2), Id::for_test(1), Id::for_test(3)]);
    assert_eq!(plan.took, supplies(4, 4, 1));
    // Without a bottle the early one is left; without flour, nothing.
    let no_bottle = plan_feeding(&stack, supplies(9, 9, 0)).unwrap();
    assert_eq!(no_bottle.yeasts.len(), 2);
    let no_flour = plan_feeding(&stack, supplies(0, 9, 1));
    assert_eq!(no_flour, Err(FeedRefused::NoFlour));
    let one = yeast(4, Quality::Common, HOUR);
    let no_bottles = plan_feeding(&one, supplies(9, 9, 0));
    assert_eq!(no_bottles, Err(FeedRefused::NoBottle));
    assert_eq!(plan_feeding(&flour(5, 1), all), Err(FeedRefused::NotYeast));
}

#[test]
fn feeding_carried_yeast_uses_flour_and_water_and_splits() {
    let limits = StackLimits::default();
    let ids = &mut crate::id::stand_ins();
    let mut inventory = Inventory::default();
    let mut give = |item: Item| inventory.give(item, limits, ids).unwrap();
    give(yeast(1, Quality::Common, 15 * HOUR));
    give(yeast(2, Quality::Common, HOUR));
    give(flour(3, 5));
    give(bottle(4, water()));
    give(bottle(5, None));
    give(jug(6, 250));
    give(bottle(7, Some(Liquid::Water(Water::Salt))));
    let found = inventory.feed_supplies(Id::for_test(3));
    assert_eq!(found, supplies(5, 3, 1));
    assert_eq!(inventory.water_source(), Some(Id::for_test(4)));
    assert_eq!(inventory.empty_bottles(), Some(Id::for_test(5)));
    let stack = inventory.find(Id::for_test(1)).unwrap();
    let plan = plan_feeding(stack, found).unwrap();
    assert_eq!(plan.took, supplies(3, 3, 1));
    let (splits, stepped) = inventory.feed_yeasts(Id::for_test(1), &plan, 0, &mut || 99);
    assert_eq!((splits.len(), stepped), (1, 0));
    assert!(form(&splits[0]).used && splits[0].above.is_empty());
    assert!(inventory.use_flour(Id::for_test(3), 3));
    assert_eq!(inventory.find(Id::for_test(3)).unwrap().count(), 2);
    assert!(!inventory.use_flour(Id::for_test(3), 3));
}

#[test]
fn water_is_drawn_a_bottle_or_100_ml_at_a_time() {
    let emptied = drawn(&bottle(1, water())).unwrap();
    assert!(emptied.bottle().unwrap().is_empty());
    assert_eq!(drawn(&bottle(1, Some(Liquid::Water(Water::Salt)))), None);
    let jug = jug(2, 150);
    assert_eq!(water_portions(&jug), 1);
    let less = drawn(&jug).unwrap();
    assert_eq!(less.vessel().unwrap().ml, 50);
    assert_eq!(drawn(&less), None);
}

#[test]
fn a_starter_develops_over_a_day_of_its_clock() {
    let sack = flour(1, 1);
    assert_eq!(can_start(&sack, &bottle(2, water())), Ok(()));
    let empty = can_start(&sack, &bottle(2, None));
    assert_eq!(empty, Err(StartRefused::NotWater));
    let backwards = can_start(&bottle(2, water()), &sack);
    assert_eq!(backwards, Err(StartRefused::NotFlour));
    let mut starter = new_starter(Id::for_test(3), None);
    assert_eq!(starter.quality, Some(Quality::Poor));
    assert!(!form(&starter).usable() && !form(&starter).feedable());
    // A developing starter and a ready yeast don't stack.
    assert!(!starter.looks_like(&yeast(4, Quality::Poor, 0)));
    assert!(starter.age_yeast(STARTS_IN - HOUR, 0).hour_passed);
    assert!(!form(&starter).usable());
    starter.age_yeast(2 * HOUR, 0);
    let ready = form(&starter);
    assert_eq!((ready.developing, ready.unfed), (0, HOUR));
    assert!(ready.usable());
    assert!(starter.looks_like(&yeast(4, Quality::Poor, 0)));
}
