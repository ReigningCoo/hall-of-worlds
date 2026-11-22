use super::*;
use crate::cask::{CaskForm, CaskSize};
use crate::fermentation::GAME_DAY_SECS;
use crate::id::Id;
use crate::item::{Item, ItemKind, Quality};
use crate::item_form::ItemForm;
use crate::material::Material;
use crate::potion::Liquid;

fn rack(size: CaskSize, length: u8) -> CaskRack {
    CaskRack { size, length }
}

fn cask(n: u128, size: CaskSize) -> Item {
    Item {
        form: Some(ItemForm::Cask(CaskForm::new(size, Material::Oak))),
        ..Item::new(Id::for_test(n), ItemKind::Cask)
    }
}

#[test]
fn racks_hold_the_owners_counts() {
    for (size, counts) in [
        (CaskSize::Pin, [3, 9, 15]),
        (CaskSize::Firkin, [3, 9, 15]),
        (CaskSize::Kilderkin, [3, 7, 11]),
        (CaskSize::Barrel, [1, 2, 3]),
        (CaskSize::Butt, [1, 2, 3]),
    ] {
        for (length, count) in (1..=3).zip(counts) {
            assert_eq!(rack(size, length).capacity(), count, "{size:?} {length}");
        }
    }
    assert_eq!(CaskRack::lengths(CaskSize::Tun), &[2]);
    assert_eq!(rack(CaskSize::Tun, 2).capacity(), 1);
    assert!(!rack(CaskSize::Tun, 1).is_well_made());
    assert_eq!(rack(CaskSize::Hogshead, 2).name(), "Hogshead Cradle");
}

#[test]
fn places_lie_in_the_grooves_within_the_piece() {
    for rack in CaskRack::all() {
        let places = rack.places();
        for place in &places {
            assert!(place.along > 0 && place.along < 4 * rack.length, "{rack:?}");
            if place.layer > 0 {
                // Resting between two below it.
                let below = |along| {
                    places.contains(&RackPlace {
                        along,
                        layer: place.layer - 1,
                    })
                };
                assert!(below(place.along - 1) && below(place.along + 1), "{rack:?}");
            }
        }
    }
}

#[test]
fn takes_only_casks_of_its_size_while_room() {
    let mut casks = RackCasks::new(rack(CaskSize::Barrel, 2));
    assert!(casks.put(cask(1, CaskSize::Pin)).is_err());
    assert!(
        casks
            .put(Item::new(Id::for_test(2), ItemKind::Knife))
            .is_err()
    );
    assert!(casks.put(cask(3, CaskSize::Barrel)).is_ok());
    assert!(casks.put(cask(4, CaskSize::Barrel)).is_ok());
    assert!(casks.put(cask(5, CaskSize::Barrel)).is_err());
    assert_eq!(
        casks.take(Id::for_test(3)).map(|i| i.id),
        Some(Id::for_test(3))
    );
    assert!(casks.places()[0].is_none());
    // The lowest free place is filled first.
    assert!(casks.put(cask(6, CaskSize::Barrel)).is_ok());
    assert_eq!(
        casks.places()[0].as_ref().map(|i| i.id),
        Some(Id::for_test(6))
    );
}

#[test]
fn sealed_casks_on_a_rack_age_ten_percent_faster() {
    let juice = CaskForm::new(CaskSize::Pin, Material::Oak).full_of(Liquid::Juice("apple".into()));
    let juice = Item {
        form: Some(ItemForm::Cask(juice)),
        ..Item::new(Id::for_test(1), ItemKind::Cask)
    };
    let plan = crate::ferment::plan(&juice, None, &[Quality::Common]).unwrap();
    let mut form = plan.started(juice.cask().unwrap());
    form.time.fermenting = 0;
    let mut item = Item {
        form: Some(ItemForm::Cask(form)),
        ..juice
    };
    item.lay_casks_down(1_000);
    let mut casks = RackCasks::new(rack(CaskSize::Pin, 1));
    casks.put(item).unwrap();
    // Settled often, a second at a time: no ageing is lost to rounding.
    for now in 1_001..=1_000 + GAME_DAY_SECS {
        casks.settle(now);
    }
    let aged = casks
        .items()
        .next()
        .and_then(Item::cask)
        .map(|cask| cask.time.aged);
    assert_eq!(aged, Some(GAME_DAY_SECS * 110 / 100));
}
