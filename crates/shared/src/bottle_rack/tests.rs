use super::*;
use crate::fermentation::{Base, Drink, Sweetness};
use crate::id::Id;
use crate::item::{Item, ItemKind, Quality};
use crate::item_form::ItemForm;
use crate::potion::{BottleForm, Liquid, Water};
use crate::vessel::{VesselForm, VesselKind};

fn rack(kind: BottleRackKind, length: u8) -> BottleRack {
    BottleRack { kind, length }
}

fn vessel(n: u128, kind: VesselKind) -> Item {
    Item {
        form: Some(ItemForm::Vessel(VesselForm::new(kind))),
        ..Item::new(Id::for_test(n), ItemKind::Vessel)
    }
}

fn bottle(n: u128, liquid: Liquid) -> Item {
    Item {
        form: Some(ItemForm::Bottle(BottleForm::new(Some(liquid), 0))),
        ..Item::new(Id::for_test(n), ItemKind::Bottle)
    }
}

fn ale() -> Liquid {
    Liquid::Drink(Box::new(Drink {
        base: Base::Grain("barley".into()),
        strength: 50,
        sweetness: Sweetness::Dry,
        quality: Quality::Common,
        vinegar: 0,
    }))
}

#[test]
fn racks_hold_their_counts() {
    for (kind, counts) in [
        (BottleRackKind::Wine, [20, 44, 68]),
        (BottleRackKind::Spirit, [9, 18, 27]),
    ] {
        for (length, count) in (1..=3).zip(counts) {
            assert_eq!(rack(kind, length).capacity(), count, "{kind:?} {length}");
        }
    }
    assert!(!rack(BottleRackKind::Wine, 4).is_well_made());
    assert_eq!(BottleRack::all().len(), 6);
}

#[test]
fn wine_places_make_a_diamond_lattice_inside_the_rack() {
    for length in 1..=3 {
        let places = rack(BottleRackKind::Wine, length).places();
        for place in &places {
            assert!(place.along > 0 && place.along < 12 * length);
            assert!(place.row < WINE_ROWS);
            if place.row > 0 && place.along > 2 && place.along < 12 * length - 2 {
                // Between two in the row below.
                let below = |along| {
                    places.contains(&BottlePlace {
                        along,
                        row: place.row - 1,
                    })
                };
                assert!(
                    below(place.along - 2) && below(place.along + 2),
                    "{place:?}"
                );
            }
        }
    }
}

#[test]
fn a_wine_rack_takes_wine_bottles_and_a_spirit_rack_drinks_too() {
    let wine = vessel(1, VesselKind::WineBottle);
    let flask = vessel(2, VesselKind::Flask);
    let tankard = vessel(3, VesselKind::Tankard);
    let of_ale = bottle(4, ale());
    let of_alcohol = bottle(5, Liquid::Alcohol);
    let of_water = bottle(6, Liquid::Water(Water::Plain));
    let wine_rack = RackBottles::new(rack(BottleRackKind::Wine, 1));
    let spirit_rack = RackBottles::new(rack(BottleRackKind::Spirit, 1));
    assert!(wine_rack.takes(&wine));
    for other in [&flask, &tankard, &of_ale, &of_alcohol] {
        assert!(!wine_rack.takes(other), "{other:?}");
    }
    for taken in [&wine, &flask, &of_ale, &of_alcohol] {
        assert!(spirit_rack.takes(taken), "{taken:?}");
    }
    assert!(!spirit_rack.takes(&tankard));
    assert!(!spirit_rack.takes(&of_water));
    // One at a time, never a stack.
    let stack = Item {
        count: 2,
        ..bottle(7, ale())
    };
    assert!(!spirit_rack.takes(&stack));
}

#[test]
fn bottles_fill_the_lowest_place_first_until_full() {
    let mut bottles = RackBottles::new(rack(BottleRackKind::Spirit, 1));
    for n in 0..9 {
        bottles.put(bottle(n, ale())).unwrap();
    }
    assert!(bottles.put(bottle(9, ale())).is_err());
    assert_eq!(
        bottles.take(Id::for_test(0)).map(|b| b.id),
        Some(Id::for_test(0))
    );
    bottles.put(bottle(10, ale())).unwrap();
    assert_eq!(
        bottles.places()[0].as_ref().map(|b| b.id),
        Some(Id::for_test(10))
    );
}
