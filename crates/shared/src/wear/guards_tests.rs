//! What worn things keep off: gloves the hands, a bandana or litham the mouth.

use crate::clothing::{ClothingPiece, Garment, NeckwearForm, roll};
use crate::creature::CreatureKind;
use crate::id::Id;
use crate::item::{Inventory, Item, Quality, StackLimits};
use crate::item_form::ItemForm;
use crate::rng::Seed;

/// A rolled one of this piece that `wanted` likes, with id `n`.
fn rolled(n: u128, piece: ClothingPiece, wanted: impl Fn(&Garment) -> bool) -> Item {
    let form = (0..)
        .map(|seed| roll(&mut Seed::new(seed).rng(), piece, Quality::Common))
        .find(|form| wanted(&form.garment))
        .unwrap();
    let mut item = Item::new(Id::for_test(n), piece.kind());
    item.form = Some(ItemForm::Clothing(form));
    item
}

/// A person wearing these.
fn wearing(items: Vec<Item>) -> Inventory {
    let mut inventory = Inventory {
        creature: CreatureKind::Human,
        size: (40, 40),
        ..Inventory::default()
    };
    for item in items {
        let id = item.id;
        (inventory.give(item, StackLimits::NONE, &mut crate::id::stand_ins())).unwrap();
        inventory.wear(id).unwrap();
    }
    inventory
}

fn neckwear(n: u128, form: NeckwearForm) -> Item {
    rolled(
        n,
        ClothingPiece::Neckwear,
        |g| matches!(g, Garment::Neckwear(neck) if neck.form == form),
    )
}

#[test]
fn a_bandana_covers_the_mouth_and_a_scarf_does_not() {
    assert!(neckwear(1, NeckwearForm::Bandana).covers_mouth());
    assert!(!neckwear(1, NeckwearForm::Scarf).covers_mouth());
    assert!(wearing(vec![neckwear(1, NeckwearForm::Bandana)]).mouth_covered());
    assert!(!wearing(vec![neckwear(1, NeckwearForm::Neckerchief)]).mouth_covered());
    assert!(!wearing(Vec::new()).mouth_covered());
}

#[test]
fn a_litham_covers_the_mouth() {
    let litham = rolled(
        1,
        ClothingPiece::Headcloth,
        |g| matches!(g, Garment::Headcloth(cloth) if cloth.face_covered),
    );
    assert!(litham.name().ends_with("Litham"));
    assert!(litham.covers_mouth());
}

#[test]
fn gloves_cover_the_hands() {
    let gloves = rolled(1, ClothingPiece::Gloves, |_| true);
    let inventory = wearing(vec![gloves]);
    assert!(inventory.hands_covered());
    assert!(!inventory.mouth_covered());
    assert!(!wearing(vec![neckwear(1, NeckwearForm::Bandana)]).hands_covered());
}
