//! Potion keys, and carried potions found by them.

use super::*;
use crate::herb::Stat;
use crate::id::Id;
use crate::item::{Inventory, Item, ItemKind, StackLimits};
use crate::item_form::ItemForm;

fn contents(doses: &[(Stat, i16)]) -> Contents {
    Contents::new(doses.iter().map(|&(stat, step)| Dose { stat, step }))
}

fn potion(held: &Contents, count: u32, id: u128) -> Item {
    let mut bottle = Item::new(Id::for_test(id), ItemKind::Bottle);
    let liquid = Some(Liquid::Potion(held.clone()));
    bottle.form = Some(ItemForm::Bottle(BottleForm::new(liquid, id as u64)));
    bottle.count = count;
    bottle
}

#[test]
fn a_key_stands_for_its_contents_and_round_trips() {
    let strong = contents(&[(Stat::Mana, 4), (Stat::Strength, -2)]);
    let key = PotionKey::of(&strong);
    assert_eq!(key.contents(), strong);
    assert!(!key.is_empty());
    assert!(PotionKey::of(&Contents::EMPTY).is_empty());
    assert_ne!(key, PotionKey::of(&contents(&[(Stat::Mana, 4)])));
    let saved = ron::to_string(&key).unwrap();
    assert_eq!(ron::from_str::<PotionKey>(&saved).unwrap(), key);
    let sent = postcard::to_allocvec(&key).unwrap();
    assert_eq!(postcard::from_bytes::<PotionKey>(&sent).unwrap(), key);
}

#[test]
fn carried_potions_are_counted_and_the_smallest_stack_drunk() {
    let wanted = contents(&[(Stat::Health, 5)]);
    let other = contents(&[(Stat::Health, 6)]);
    let mut carried = Inventory::default();
    let stand_ins = &mut crate::id::stand_ins();
    for item in [potion(&wanted, 3, 1), potion(&other, 1, 2)] {
        carried
            .give(item, StackLimits::default(), stand_ins)
            .unwrap();
    }
    let key = PotionKey::of(&wanted);
    assert_eq!(carried.count_of_potion(key), 3);
    assert_eq!(carried.potion_to_use(key), Some(Id::for_test(1)));
    let none = PotionKey::of(&contents(&[(Stat::Will, 4)]));
    assert_eq!(carried.potion_to_use(none), None);
    assert!(potion(&wanted, 1, 3).is_potion());
    assert_eq!(potion(&wanted, 1, 3).potion_key(), Some(key));
}
