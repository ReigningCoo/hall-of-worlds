//! Wearing Middle-layer clothes: one garment on the torso, long ones still leaving the
//! legs and waist free, and an arming doublet under unlined plate (owner, 2026-11-15).

use super::*;
use crate::clothing::{ClothingPiece, Garment, Length, roll};
use crate::creature::CreatureKind;
use crate::id::Id;
use crate::item::{Inventory, Item, ItemKind, StackLimits};
use crate::rng::Seed;

fn id(n: u128) -> Id {
    Id::for_test(n)
}

/// These items, made for a person, ids from 1.
fn carrying(items: Vec<Item>) -> Inventory {
    let mut inventory = Inventory {
        creature: CreatureKind::Human,
        size: (40, 40),
        ..Inventory::default()
    };
    for item in items {
        inventory
            .give(item, StackLimits::NONE, &mut crate::id::stand_ins())
            .unwrap();
    }
    inventory
}

fn plain(n: u128, kind: ItemKind) -> Item {
    Item::new(id(n), kind)
}

/// A rolled one of this piece that `wanted` likes.
fn rolled(n: u128, piece: ClothingPiece, wanted: impl Fn(&Garment) -> bool) -> Item {
    let form = (0..)
        .map(|seed| roll(&mut Seed::new(seed).rng(), piece, Quality::Rare))
        .find(|form| wanted(&form.garment))
        .unwrap();
    let mut item = plain(n, piece.kind());
    item.form = Some(ItemForm::Clothing(form));
    item
}

#[test]
fn a_long_tunic_goes_with_trousers_and_a_belt() {
    let long =
        |garment: &Garment| matches!(garment, Garment::Tunic(t) if t.length >= Length::Ankle);
    let mut inventory = carrying(vec![
        rolled(1, ClothingPiece::Tunic, long),
        plain(2, ItemKind::Trousers),
        plain(3, ItemKind::Belt),
        plain(4, ItemKind::Shirt),
    ]);
    for n in 1..=4 {
        assert_eq!(inventory.wear(id(n)), Ok(Fit::Exact), "item {n}");
    }
}

#[test]
fn one_garment_on_the_torso() {
    let mut inventory = carrying(vec![
        plain(1, ItemKind::Doublet),
        plain(2, ItemKind::Vest),
        plain(3, ItemKind::ArmingVest),
    ]);
    inventory.wear(id(1)).unwrap();
    assert_eq!(inventory.check_wear(id(2)), Err(WearRefused::Clash(id(1))));
    assert_eq!(inventory.check_wear(id(3)), Err(WearRefused::Clash(id(1))));
}

#[test]
fn an_arming_doublet_goes_under_unlined_plate() {
    let arming = |garment: &Garment| matches!(garment, Garment::Doublet(d) if d.arming());
    let mut inventory = carrying(vec![
        rolled(1, ClothingPiece::Doublet, arming),
        plain(2, ItemKind::TorsoArmour),
    ]);
    inventory.wear(id(1)).unwrap();
    // Without a form, torso armour is unlined.
    assert_eq!(inventory.wear(id(2)), Ok(Fit::Exact));
    assert_eq!(inventory.covered_by(id(1)), Some(id(2)));
}
