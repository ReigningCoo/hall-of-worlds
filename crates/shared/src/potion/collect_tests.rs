use super::*;
use crate::dungeon::{Cell, Level};
use crate::facing::Heading;
use crate::id::Id;
use crate::item::{Inventory, Item, ItemKind, StackLimits};
use crate::skill::{SkillId, Skills};
use crate::spot::Spot;

/// A level of floor, with water from column `water_from` east.
fn shore(water_from: i64) -> Level {
    let mut level = Level::new(0, 0, 10, 10);
    for y in 0..10 {
        for x in 0..10 {
            let cell = if x >= water_from {
                Cell::Water
            } else {
                Cell::Ground
            };
            level.set(x, y, cell);
        }
    }
    level
}

#[test]
fn the_place_decides_the_water() {
    let at = |underground, sea, moonlit| {
        water_from(Source {
            underground,
            sea,
            moonlit,
        })
    };
    assert_eq!(at(false, false, false), Water::Plain);
    assert_eq!(at(true, false, true), Water::Cave);
    assert_eq!(at(false, true, true), Water::Salt);
    assert_eq!(at(false, false, true), Water::Moon);
}

#[test]
fn rain_is_caught_outside_in_steady_warm_rain() {
    assert!(catches_rain(true, RAIN_AT, 0.5));
    assert!(!catches_rain(false, 1.0, 0.5), "under a roof");
    assert!(!catches_rain(true, RAIN_AT - 0.01, 0.5), "a drizzle");
    assert!(!catches_rain(true, 1.0, 0.1), "snow");
}

#[test]
fn water_is_filled_from_only_when_faced_and_near() {
    let level = shore(5);
    let bank = Spot::middle_of((4, 5));
    assert_eq!(water_faced(&level, bank, Heading::EAST), Some((5, 5)));
    assert_eq!(water_faced(&level, bank, Heading::WEST), None, "behind");
    let north_west = Heading::NORTH.turned(-i32::from(crate::facing::EIGHTH));
    assert_eq!(water_faced(&level, bank, north_west), None, "beside");
    let diagonal = water_faced(&level, bank, Heading::NORTH);
    assert_eq!(diagonal, Some((5, 4)), "45° off still counts");
    let back = Spot::middle_of((2, 5));
    assert_eq!(water_faced(&level, back, Heading::EAST), None, "too far");
    let wading = Spot::middle_of((6, 5));
    assert_eq!(water_faced(&level, wading, Heading::WEST), Some((6, 5)));
}

#[test]
fn a_bottle_takes_ten_seconds_down_to_six() {
    let mut skills = Skills::default();
    assert_eq!(fill_seconds(&skills), 10);
    let water = SkillId::find(WATER_SKILL).expect("the skill tree has Water");
    skills.set_percent(water, 100);
    assert_eq!(fill_seconds(&skills), 6);
    assert_eq!(fill_ticks(&skills), 6 * 20);
}

#[test]
fn empty_bottles_are_filled_and_teach_collect_water() {
    use crate::ability::{Ability, Learned};
    let mut inventory = Inventory::default();
    let mut learned = Learned::default();
    assert!(!learned.learn_from(&inventory));
    let mut bottle = Item::new(Id::for_test(1), ItemKind::Bottle);
    bottle.form = Some(crate::item_form::ItemForm::Bottle(BottleForm::new(None, 7)));
    let water = filled(bottle.clone(), Water::Moon);
    assert_eq!(water.bottle().unwrap().look, 7);
    assert_eq!(water.name(), "Bottle of Moon Water");
    let stand_ins = &mut crate::id::stand_ins();
    inventory.give(water, StackLimits::NONE, stand_ins).unwrap();
    assert_eq!(inventory.empty_bottle(), None, "a full one isn't empty");
    assert!(!learned.learn_from(&inventory));
    inventory
        .give(bottle, StackLimits::NONE, stand_ins)
        .unwrap();
    assert_eq!(inventory.empty_bottle(), Some(Id::for_test(1)));
    assert!(learned.learn_from(&inventory));
    assert!(learned.knows(Ability::CollectWater));
    assert!(!learned.learn_from(&inventory), "learned once");
}
