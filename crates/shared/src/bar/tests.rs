use super::*;
use crate::bottle_rack::{BottleRack, BottleRackKind, RackBottles};
use crate::cask::CaskSize;
use crate::cask_rack::CaskRack;
use crate::crafting_bench::{BenchTools, Craft};
use crate::door::Side;
use crate::dungeon::{Cell, Level, RoomId};
use crate::furniture::{Furniture, Held, Holder, Material, Piece, Wood};
use crate::id::Id;
use crate::item::{Item, ItemKind};
use crate::item_form::ItemForm;
use crate::potion::{Liquid, Water};
use crate::vessel::{VesselForm, VesselKind};

fn counter(kind: BarKind, length: u8) -> BarCounter {
    BarCounter { kind, length }
}

fn vessel(n: u128, kind: VesselKind) -> Item {
    Item {
        form: Some(ItemForm::Vessel(VesselForm::new(kind))),
        ..Item::new(Id::for_test(n), ItemKind::Vessel)
    }
}

fn piece(at: (i64, i64), length: u8, furniture: Furniture) -> Piece {
    Piece {
        at,
        size: (length, 1),
        facing: Side::South,
        material: Material::Wood(Wood::Oak),
        furniture,
    }
}

#[test]
fn counters_hold_four_a_cell_and_flaps_none() {
    for kind in [BarKind::Single, BarKind::Double] {
        for length in 1..=3 {
            let bar = counter(kind, length);
            assert!(bar.is_well_made());
            assert_eq!(bar.capacity(), 4 * usize::from(length));
            let places = bar.places();
            assert!(places.iter().all(|p| p.along < 24 * length));
            assert!(places.windows(2).all(|w| w[0].along < w[1].along));
        }
    }
    let flap = counter(BarKind::Flap, 1);
    assert!(flap.is_well_made() && flap.capacity() == 0);
    assert!(!counter(BarKind::Flap, 2).is_well_made());
    assert!(!counter(BarKind::Single, 4).is_well_made());
    assert_eq!(BarCounter::all().len(), 7);
}

#[test]
fn bars_take_drinking_vessels_only() {
    let mut vessels = BarVessels::new(counter(BarKind::Single, 1));
    for (n, kind) in [
        VesselKind::WineBottle,
        VesselKind::Flask,
        VesselKind::Waterskin,
    ]
    .into_iter()
    .enumerate()
    {
        assert!(vessels.put(vessel(n as u128, kind)).is_err(), "{kind:?}");
    }
    let tankards = [10, 11, 12, 13].map(|n| vessel(n, VesselKind::Tankard));
    for tankard in tankards {
        assert!(vessels.put(tankard).is_ok());
    }
    assert!(vessels.put(vessel(14, VesselKind::Cup)).is_err(), "full");
    assert!(vessels.take(Id::for_test(11)).is_some());
    assert!(vessels.put(vessel(14, VesselKind::Jug)).is_ok());
    assert_eq!(
        vessels.places()[1].as_ref().map(|v| v.id),
        Some(Id::for_test(14))
    );
}

#[test]
fn replace_keeps_the_place() {
    let mut vessels = BarVessels::new(counter(BarKind::Double, 1));
    vessels.put(vessel(1, VesselKind::Cup)).unwrap();
    vessels.put(vessel(2, VesselKind::Jug)).unwrap();
    let water = Liquid::Water(Water::Plain);
    let mut filled = vessel(2, VesselKind::Jug);
    filled.form = Some(ItemForm::Vessel(
        VesselForm::new(VesselKind::Jug).full_of(water),
    ));
    assert!(vessels.replace(filled.clone()));
    assert_eq!(vessels.places()[1].as_ref(), Some(&filled));
    assert!(!vessels.replace(vessel(3, VesselKind::Cup)));
}

#[test]
fn counters_hold_spirit_racks_and_flaps_cost_nothing() {
    let single = Furniture::BarCounter {
        counter: counter(BarKind::Single, 2),
    };
    let flap = Furniture::BarCounter {
        counter: counter(BarKind::Flap, 1),
    };
    assert!(single.holds_furniture() && !flap.holds_furniture());
    assert_eq!(flap.cost_per_mille(), 1000);
    assert_eq!(Holder::of(flap), None);
    assert!(Holder::of(single).is_some_and(Holder::is_drink_storage));
    assert!(!Holder::Bench(Craft::Alchemy).is_drink_storage());
}

/// A level of two rooms side by side, 0..4 and 4..8 across.
fn two_rooms() -> Level {
    let mut level = Level::new(0, 0, 8, 4);
    for y in 0..4 {
        for x in 0..8 {
            level.set(x, y, Cell::Floor);
            level.set_room(x, y, Some(RoomId(u16::from(x >= 4))));
        }
    }
    level
}

#[test]
fn a_bar_pours_from_its_own_rooms_storage() {
    let mut level = two_rooms();
    let bar = piece(
        (0, 2),
        2,
        Furniture::BarCounter {
            counter: counter(BarKind::Single, 2),
        },
    );
    let cask_rack = CaskRack {
        size: CaskSize::Pin,
        length: 1,
    };
    let wine = BottleRack {
        kind: BottleRackKind::Wine,
        length: 1,
    };
    let spirit = BottleRack {
        kind: BottleRackKind::Spirit,
        length: 1,
    };
    let here = piece((0, 0), 1, Furniture::CaskRack { rack: cask_rack });
    let there = piece((5, 0), 1, Furniture::BottleRack { rack: wine });
    let bench = piece(
        (2, 0),
        2,
        Furniture::CraftingBench {
            craft: Craft::Alchemy,
        },
    );
    let on_bar = piece((1, 2), 1, Furniture::BottleRack { rack: spirit });
    for p in [bar, here, there, bench, on_bar] {
        level.place_furniture(p);
    }
    let found: Vec<(i64, i64)> = room_storage(&level, &bar).iter().map(|p| p.at).collect();
    assert_eq!(found, vec![(0, 0), (0, 2), (1, 2)]);
}

#[test]
fn open_ground_is_one_room() {
    let mut level = Level::new(0, 0, 8, 4);
    for y in 0..4 {
        for x in 0..8 {
            level.set(x, y, Cell::Floor);
        }
    }
    let bar = piece(
        (0, 2),
        1,
        Furniture::BarCounter {
            counter: counter(BarKind::Double, 1),
        },
    );
    let rack = CaskRack {
        size: CaskSize::Firkin,
        length: 1,
    };
    let far = piece((7, 0), 1, Furniture::CaskRack { rack });
    level.place_furniture(bar);
    level.place_furniture(far);
    assert_eq!(room_storage(&level, &bar).len(), 2);
}

#[test]
fn drinks_are_whatever_holds_something() {
    let mut bottles = RackBottles::new(BottleRack {
        kind: BottleRackKind::Spirit,
        length: 1,
    });
    let mut full = vessel(2, VesselKind::Flask);
    full.form = Some(ItemForm::Vessel(
        VesselForm::new(VesselKind::Flask).full_of(Liquid::Water(Water::Plain)),
    ));
    bottles.put(full).unwrap();
    bottles.put(vessel(3, VesselKind::Flask)).unwrap();
    let held = Held::Bottles(bottles);
    let ids: Vec<Id> = drinks_in(&held).map(|item| item.id).collect();
    assert_eq!(ids, vec![Id::for_test(2)]);
    assert_eq!(
        drinks_in(&Held::Bench(BenchTools::new(Craft::Alchemy))).count(),
        0
    );
}
