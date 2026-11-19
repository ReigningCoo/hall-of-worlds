use super::*;
use crate::id::Id;
use crate::item::{Inventory, Item, ItemKind, Quality, StackLimits};
use crate::rng::Seed;

const HOUR: u32 = SECONDS_PER_HOUR;

fn yeast(n: u128, quality: Quality) -> Item {
    new_item(quality, Id::for_test(n), None)
}

fn unfed(item: &Item) -> u32 {
    item.yeast().unwrap().unfed
}

#[test]
fn a_new_yeast_is_fed_and_usable() {
    let item = yeast(1, Quality::Common);
    let form = item.yeast().unwrap();
    assert_eq!(item.kind, ItemKind::Yeast);
    assert!(form.usable());
    assert_eq!(
        (form.feed_in(), form.weakens_in()),
        (FEED_AFTER, WEAKENS_EVERY)
    );
    assert!(item.kind.def().has_quality && !item.kind.def().counted);
}

#[test]
fn it_loses_a_step_each_day_unfed_down_to_poor() {
    let mut item = yeast(1, Quality::Rare);
    let settled = item.age_yeast(23 * HOUR, 0);
    assert!(settled.hour_passed && !settled.weakened);
    assert_eq!(item.quality, Some(Quality::Rare));
    assert_eq!(item.yeast().unwrap().feed_in(), 0);
    assert!(item.age_yeast(HOUR, 0).weakened);
    assert_eq!(item.quality, Some(Quality::Uncommon));
    // Three days at once: Uncommon, Common, Poor, and Poor stays Poor.
    item.age_yeast(3 * WEAKENS_EVERY, 0);
    assert_eq!(item.quality, Some(Quality::Poor));
    assert_eq!(unfed(&item), 4 * WEAKENS_EVERY);
    assert_eq!(item.yeast().unwrap().weakens_in(), WEAKENS_EVERY);
}

#[test]
fn less_than_an_hour_needs_no_telling() {
    let mut item = yeast(1, Quality::Common);
    assert!(!item.age_yeast(60, 0).seen());
    assert_eq!(unfed(&item), 60);
}

#[test]
fn on_the_ground_it_goes_by_real_time() {
    let mut item = yeast(1, Quality::Common);
    item.lay_yeast_down(1000);
    // Laid down again, it keeps the time it was first put down.
    item.lay_yeast_down(5000);
    assert_eq!(item.yeast().unwrap().lying_since, Some(1000));
    // Looked at on the ground: the real time so far, still lying.
    item.settle_lying_yeast(1000 + u64::from(HOUR));
    assert_eq!(unfed(&item), HOUR);
    // Picked up: the rest of its real time, then playtime.
    let now = 1000 + u64::from(25 * HOUR);
    assert!(item.age_yeast(60, now).weakened);
    assert_eq!(unfed(&item), 25 * HOUR);
    assert_eq!(item.yeast().unwrap().lying_since, None);
    item.age_yeast(60, now);
    assert_eq!(unfed(&item), 25 * HOUR + 60);
}

#[test]
fn every_yeast_carried_ages_each_by_its_own_clock() {
    let limits = StackLimits::default();
    let mut inventory = Inventory::default();
    let mut older = yeast(1, Quality::Common);
    older.age_yeast(20 * HOUR, 0);
    let ids = &mut crate::id::stand_ins();
    inventory.give(older, limits, ids).unwrap();
    inventory
        .give(yeast(2, Quality::Common), limits, ids)
        .unwrap();
    // They stack, each keeping its own clock.
    assert_eq!(inventory.items.len(), 1);
    assert!(inventory.age_yeast(4 * HOUR, 0).weakened);
    let stack = &inventory.items[0].item;
    assert_eq!(stack.quality, Some(Quality::Poor));
    assert_eq!(stack.above[0].quality, Some(Quality::Common));
    assert_eq!(
        (unfed(stack), unfed(&stack.above[0])),
        (24 * HOUR, 4 * HOUR)
    );
}

#[test]
fn shops_sell_it_nearly_always_common() {
    let mut rng = Seed::new(7).rng();
    let rolls = 1000;
    let common = (0..rolls)
        .filter(|_| shop_quality(Quality::Rare, &mut rng) == Quality::Common)
        .count();
    assert!((900..rolls).contains(&common), "{common} of {rolls} Common");
}

#[test]
fn it_is_given_a_fresh_form() {
    let mut item = Item::new(Id::for_test(1), ItemKind::Yeast);
    crate::item_form::give_form(&mut item, None, &mut || Id::for_test(2));
    assert_eq!(item.yeast(), Some(&YeastForm::default()));
    assert_eq!(item.name(), "Yeast");
}

#[test]
fn quality_steps_down_once_and_stops_at_poor() {
    assert_eq!(Quality::Common.worse(), Quality::Poor);
    assert_eq!(Quality::Poor.worse(), Quality::Poor);
    assert_eq!(Quality::Artifact.worse(), Quality::Legendary);
}
