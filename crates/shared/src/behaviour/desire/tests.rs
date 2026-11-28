use super::super::{Behaviours, Desire};
use crate::creature::CreatureKind;
use crate::item::ItemKind;

#[test]
fn each_desire_wants_its_own_things() {
    assert!(Desire::Food.wants(ItemKind::Fruit));
    assert!(Desire::Food.wants(ItemKind::RawMeat));
    assert!(!Desire::Food.wants(ItemKind::GoldCoin));
    assert!(Desire::Coins.wants(ItemKind::CopperCoin));
    assert!(Desire::Coins.wants(ItemKind::GoldCoin));
    assert!(!Desire::Coins.wants(ItemKind::Gem));
    assert!(Desire::Weapons.wants(ItemKind::Dagger));
    assert!(!Desire::Weapons.wants(ItemKind::Stone));
}

#[test]
fn any_of_several() {
    let desires = [Desire::Coins, Desire::Weapons];
    assert!(Desire::any_wants(&desires, ItemKind::Sword));
    assert!(!Desire::any_wants(&desires, ItemKind::Fruit));
    assert!(!Desire::any_wants(&[], ItemKind::Sword));
}

#[test]
fn some_races_want_something() {
    let behaviours = Behaviours::built_in();
    let wanting = (CreatureKind::ALL.iter())
        .filter(|&&kind| !behaviours.of(kind).desires.is_empty())
        .count();
    assert!(wanting > 0);
}
