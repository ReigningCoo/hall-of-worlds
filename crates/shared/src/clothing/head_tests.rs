//! Tests for head, neck and shoulder pieces and hats.

use super::*;
use crate::creature::{Layer, Location};
use crate::id::Id;
use crate::item::{Item, ItemKind, Quality};

#[test]
fn rank_tails_and_ruffs_come_at_rare_and_better() {
    let long_tail = |quality| {
        rolled(ClothingPiece::Hood, quality)
            .any(|form| matches!(form.garment, Garment::Hood(h) if h.tail == Tail::LongLiripipe))
    };
    assert!(!long_tail(Quality::Common));
    assert!(long_tail(Quality::Rare));
    let cartwheel = |quality| {
        rolled(ClothingPiece::Neckwear, quality).any(|form| {
            matches!(form.garment, Garment::Neckwear(n) if n.ruff == Some(RuffSize::Cartwheel))
        })
    };
    assert!(!cartwheel(Quality::Common));
    assert!(cartwheel(Quality::Rare));
}

#[test]
fn names_left_out_never_turn_up() {
    let names: Vec<_> = every(Quality::Rare)
        .iter()
        .map(ClothingForm::name)
        .collect();
    for unwanted in [
        "Keffiyeh",
        "Dastar",
        "Niqab",
        "Burqa",
        "Jewish",
        "Balaclava",
    ] {
        assert!(
            !names.iter().any(|name| name.contains(unwanted)),
            "{unwanted} turned up"
        );
    }
}

#[test]
fn names_do_not_repeat_their_material() {
    for form in every(Quality::Common) {
        let name = form.name();
        assert!(!name.contains("Straw Straw"), "{name}");
        assert!(!name.contains("Fur Ushanka"), "{name}");
        assert!(!name.contains("Felt Fez"), "{name}");
    }
    let hats: Vec<_> = rolled(ClothingPiece::Hat, Quality::Common)
        .map(|form| form.name())
        .collect();
    assert!(hats.iter().any(|name| name == "Leather Hat"));
}

#[test]
fn hats_go_over_hoods_and_caps_and_a_hood_with_a_shawl() {
    let worn = |kind: ItemKind| kind.def().wear.unwrap();
    assert_eq!(worn(ItemKind::Hat).layer, Layer::Outer);
    for kind in [
        ItemKind::Hood,
        ItemKind::Cap,
        ItemKind::Headcloth,
        ItemKind::Headdress,
        ItemKind::Neckwear,
        ItemKind::Shawl,
    ] {
        assert_eq!(worn(kind).layer, Layer::Middle, "{kind:?}");
    }
    let hood = Item::new(Id::for_test(2), ItemKind::Hood);
    let shawl = Item {
        form: Some(crate::item_form::ItemForm::Clothing(
            rolled(ClothingPiece::Shawl, Quality::Common)
                .find(|form| matches!(form.garment, Garment::Shawl(s) if s.collar == ShawlCollar::None))
                .unwrap(),
        )),
        ..Item::new(Id::for_test(3), ItemKind::Shawl)
    };
    let hood_places = hood.worn_locations();
    assert!(
        shawl
            .worn_locations()
            .iter()
            .all(|place| !hood_places.contains(place))
    );
    assert_eq!(shawl.worn_locations(), vec![Location::Shoulders]);
}

#[test]
fn old_hats_without_forms_still_load_and_wear_as_before() {
    let old = Item::new(Id::for_test(7), ItemKind::Hat);
    let text = ron::to_string(&old).unwrap();
    let loaded: Item = ron::from_str(&text).unwrap();
    assert_eq!(loaded.form, None);
    assert_eq!(loaded.worn_locations(), vec![Location::Head]);
    assert_eq!(loaded.name(), "Hat");
}
