//! Drinking through a connected client: each attribute an effect of its own,
//! a new dose replacing only its own stat's, the bottle back empty, and the
//! same stats and signs refused while cooling down.

use shared::bonus::Source;
use shared::herb::Stat;
use shared::id::Id;
use shared::item::{Item, ItemKind, Quality, StackLimits};
use shared::item_form::ItemForm;
use shared::potion::{BottleForm, Contents, Dose, Liquid};
use shared::protocol::ClientMessage;

use crate::connections::tests::TestClient;

fn potion(doses: &[(Stat, i16)], count: u32, id: u128) -> Item {
    let contents = Contents::new(doses.iter().map(|&(stat, step)| Dose { stat, step }));
    let mut bottle = Item::new(Id::for_test(id), ItemKind::Bottle);
    let liquid = Some(Liquid::Potion(contents));
    bottle.form = Some(ItemForm::Bottle(BottleForm::new(liquid, 3)));
    bottle.quality = Some(Quality::Common);
    bottle.count = count;
    bottle
}

fn give(test: &mut TestClient, item: Item) {
    let new_id = &mut shared::id::stand_ins();
    let given = test.client.inventory.give(item, StackLimits::NONE, new_id);
    assert!(given.is_ok());
}

fn drink(test: &mut TestClient, id: u128) {
    test.send(&[ClientMessage::Consume {
        item: Id::for_test(id),
    }]);
}

/// The stats with a potion's effect on them now, and each one's start.
fn potion_effects(test: &TestClient) -> Vec<(Stat, u64)> {
    let effects = test.client.sheet.effects.0.iter();
    let potions = effects.filter_map(|effect| match effect.source {
        Source::Potion(stat) => Some((stat, effect.start)),
        _ => None,
    });
    let mut found: Vec<_> = potions.collect();
    found.sort();
    found
}

fn empty_bottles(test: &TestClient) -> usize {
    let carried = test.client.inventory.items.iter();
    carried
        .filter(|placed| placed.item.bottle().is_some_and(BottleForm::is_empty))
        .map(|placed| placed.item.count as usize)
        .sum()
}

#[test]
fn drinking_gives_an_effect_per_stat_and_the_bottle_back() {
    let mut test = TestClient::new();
    test.enter_a_castle();
    give(
        &mut test,
        potion(&[(Stat::Strength, 4), (Stat::Agility, 2)], 2, 71),
    );
    drink(&mut test, 71);
    let first = potion_effects(&test);
    let stats: Vec<_> = first.iter().map(|&(stat, _)| stat).collect();
    assert_eq!(stats, [Stat::Strength, Stat::Agility]);
    assert_eq!(empty_bottles(&test), 1);
    // The same stats and signs wait: nothing is drunk.
    drink(&mut test, 71);
    assert_eq!(
        test.client
            .inventory
            .count_of_potion(key(&[(Stat::Strength, 4), (Stat::Agility, 2)])),
        1
    );
    assert_eq!(empty_bottles(&test), 1);
    // Another combination may be drunk at once; its Strength replaces the old.
    give(
        &mut test,
        potion(&[(Stat::Strength, 5), (Stat::Will, 2)], 1, 72),
    );
    drink(&mut test, 72);
    let second = potion_effects(&test);
    let agility = first.iter().find(|it| it.0 == Stat::Agility).copied();
    let strength = second.iter().filter(|it| it.0 == Stat::Strength).count();
    assert_eq!(strength, 1);
    assert!(second.contains(&agility.unwrap()), "{second:?}");
    assert!(second.iter().any(|it| it.0 == Stat::Will));
    assert_eq!(empty_bottles(&test), 2);
}

fn key(doses: &[(Stat, i16)]) -> shared::potion::PotionKey {
    let contents = Contents::new(doses.iter().map(|&(stat, step)| Dose { stat, step }));
    shared::potion::PotionKey::of(&contents)
}
