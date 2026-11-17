//! Who may eat what, and what eating it does.

use super::*;
use crate::creature::CreatureKind::{Dwarf, Goblin, Human, Orc};
use crate::id::Id;
use crate::item::{Item, ItemKind};
use crate::requirement::{Requirement, item_requirements};
use crate::vitals::Vital;

const SECOND: u64 = crate::energy::ACTION_TICKS_PER_SECOND as u64;

fn restoring(effect: &Effect) -> Option<i32> {
    effect.bonuses.iter().find_map(|bonus| match bonus {
        Bonus {
            stat: Stat::Restore(Vital::Health),
            amount: Amount::Flat(milli),
        } => Some(*milli),
        _ => None,
    })
}

#[test]
fn bread_restores_anyone_for_45_seconds() {
    let effects = eating(ItemKind::Bread, Human, 100).expect("anyone eats bread");
    assert_eq!(effects.len(), 1);
    assert_eq!(restoring(&effects[0]), Some(1000));
    assert_eq!(effects[0].end, Some(100 + 45 * SECOND));
    assert!(effects[0].ends_on.moving && effects[0].ends_on.hurt);
}

#[test]
fn dwarven_bread_is_for_dwarves_only() {
    let effects = eating(ItemKind::DwarvenBread, Dwarf, 0).expect("a dwarf eats it");
    assert_eq!(effects[0].end, Some(60 * SECOND));
    assert!(eating(ItemKind::DwarvenBread, Human, 0).is_none());
    assert!(may_eat(ItemKind::DwarvenBread, Dwarf));
    assert!(!may_eat(ItemKind::DwarvenBread, Orc));
}

#[test]
fn raw_meat_heals_orcs_and_goblins_and_poisons_the_rest() {
    for kind in [Orc, Goblin] {
        let effects = eating(ItemKind::RawMeat, kind, 0).expect("it suits them");
        assert_eq!(restoring(&effects[0]), Some(2000));
        assert_eq!(effects[0].source, Source::Eaten(ItemKind::RawMeat));
    }
    assert!(may_eat(ItemKind::RawMeat, Human));
    let effects = eating(ItemKind::RawMeat, Human, 10).expect("others may eat it");
    assert_eq!(effects, vec![poison(10)]);
    assert_eq!(effects[0].source, Source::Poison);
    assert_eq!(effects[0].ends_on, EndsOn::NOTHING);
    assert_eq!(effects[0].end, Some(10 + POISON_TICKS));
}

#[test]
fn stones_are_not_food() {
    assert!(!is_food(ItemKind::Stone));
    assert!(!may_eat(ItemKind::Stone, Human));
    assert!(eating(ItemKind::Stone, Human, 0).is_none());
}

#[test]
fn food_for_some_kinds_asks_for_them() {
    let item = |kind| Item::new(Id::for_test(1), kind);
    let asked = item_requirements(&item(ItemKind::RawMeat));
    assert_eq!(asked, vec![Requirement::Kinds(vec![Orc, Goblin])]);
    assert!(asked[0].suits(Orc) && !asked[0].suits(Dwarf));
    assert!(item_requirements(&item(ItemKind::Bread)).is_empty());
}

#[test]
fn herbs_are_eaten_raw_but_arent_food() {
    assert!(edible(ItemKind::Herb) && edible(ItemKind::Fruit));
    assert!(!is_food(ItemKind::Herb), "herbs never go on the action bar");
    assert!(may_eat(ItemKind::Herb, Orc) && may_eat(ItemKind::Herb, Dwarf));
    assert!(!edible(ItemKind::Stone));
}
