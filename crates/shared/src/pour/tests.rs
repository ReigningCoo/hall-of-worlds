use super::*;
use crate::cask::{CaskForm, CaskSize};
use crate::herb::Stat;
use crate::id::Id;
use crate::item::{Inventory, Item, ItemKind, StackLimits};
use crate::item_form::ItemForm;
use crate::material::Material;
use crate::potion::{BottleForm, Contents, Dose, Liquid, Oil, Water};
use crate::vessel::{VesselForm, VesselKind};

const WATER: Liquid = Liquid::Water(Water::Plain);

fn item(id: u128, kind: ItemKind, form: ItemForm) -> Item {
    Item {
        form: Some(form),
        ..Item::new(Id::for_test(id), kind)
    }
}

fn cask(id: u128, form: CaskForm) -> Item {
    item(id, ItemKind::Cask, ItemForm::Cask(form))
}

fn vessel(id: u128, form: VesselForm) -> Item {
    item(id, ItemKind::Vessel, ItemForm::Vessel(form))
}

fn bottle(id: u128, liquid: Option<Liquid>) -> Item {
    item(
        id,
        ItemKind::Bottle,
        ItemForm::Bottle(BottleForm::new(liquid, 5)),
    )
}

fn full_pin() -> Item {
    cask(
        1,
        CaskForm::new(CaskSize::Pin, Material::Oak).full_of(WATER),
    )
}

#[test]
fn drawing_from_a_cask_needs_a_tap_and_opens_it() {
    let tankard = vessel(2, VesselForm::new(VesselKind::Tankard));
    assert_eq!(
        pour(&full_pin(), &tankard, false),
        Err(PourRefused::NeedsTap)
    );
    let poured = pour(&full_pin(), &tankard, true).unwrap();
    assert_eq!(poured.ml, 500);
    let cask = poured.from.cask().unwrap();
    assert_eq!((cask.ml, cask.opened), (19_500, true));
    let tankard = poured.into.vessel().unwrap();
    assert_eq!((tankard.ml, tankard.sealed), (500, false));
}

#[test]
fn a_bottle_filled_from_a_cask_starts_sealed() {
    let wine = vessel(2, VesselForm::new(VesselKind::WineBottle));
    let poured = pour(&full_pin(), &wine, true).unwrap();
    assert!(poured.into.vessel().unwrap().sealed);
    let glass = pour(&full_pin(), &bottle(3, None), true).unwrap();
    assert_eq!(glass.ml, BOTTLE_ML);
    assert!(glass.into.bottle().unwrap().sealed);
    let jug = vessel(4, VesselForm::new(VesselKind::Jug).full_of(WATER));
    let from_jug = pour(&jug, &wine, false).unwrap();
    assert!(!from_jug.into.vessel().unwrap().sealed, "not from a cask");
    let opened = pour(&from_jug.into, &bottle(3, None), false).unwrap();
    assert!(!opened.from.vessel().unwrap().sealed, "poured from: opened");
}

#[test]
fn pouring_into_a_cask_opens_it_and_fills_to_the_brim() {
    let skin = vessel(2, VesselForm::new(VesselKind::Waterskin).full_of(WATER));
    let empty = cask(1, CaskForm::new(CaskSize::Pin, Material::Oak));
    let poured = pour(&skin, &empty, false).unwrap();
    assert_eq!(poured.ml, 2000);
    let into = poured.into.cask().unwrap();
    assert_eq!((into.ml, into.opened), (2000, true));
    assert!(poured.from.vessel().unwrap().is_empty());
    let cup = vessel(3, VesselForm::new(VesselKind::Cup));
    let poured = pour(&skin, &cup, false).unwrap();
    assert_eq!(poured.ml, 250, "only what the cup takes");
    assert_eq!(poured.from.vessel().unwrap().ml, 1750);
}

#[test]
fn never_two_liquids_nor_potions() {
    let oil = vessel(
        2,
        VesselForm::new(VesselKind::Jug).full_of(Liquid::Oil(Oil::Nut)),
    );
    let water = vessel(3, VesselForm::new(VesselKind::Cup).holding(Some(WATER), 10));
    assert_eq!(pour(&oil, &water, true), Err(PourRefused::Different));
    let dose = Dose {
        stat: Stat::Strength,
        step: 4,
    };
    let potion = bottle(4, Some(Liquid::Potion(Contents::new([dose]))));
    assert_eq!(pour(&potion, &water, true), Err(PourRefused::Potion));
    let stone = Item::new(Id::for_test(5), ItemKind::Stone);
    assert_eq!(pour(&oil, &stone, true), Err(PourRefused::NotContainers));
    assert_eq!(pour(&oil, &oil, true), Err(PourRefused::Itself));
    let empty = vessel(6, VesselForm::new(VesselKind::Cup));
    assert_eq!(pour(&empty, &water, true), Err(PourRefused::Empty));
}

#[test]
fn bottles_are_filled_and_emptied_whole() {
    let goblet = vessel(
        2,
        VesselForm::new(VesselKind::Goblet).holding(Some(WATER), 50),
    );
    assert_eq!(
        pour(&goblet, &bottle(3, None), false),
        Err(PourRefused::TooLittle)
    );
    let nearly = vessel(
        4,
        VesselForm::new(VesselKind::Goblet).holding(Some(WATER), 150),
    );
    assert_eq!(
        pour(&bottle(3, Some(WATER)), &nearly, false),
        Err(PourRefused::Full),
        "only 50 ml of room"
    );
    let poured = pour(&bottle(3, Some(WATER)), &goblet, false).unwrap();
    assert_eq!(poured.ml, BOTTLE_ML);
    assert!(poured.from.bottle().unwrap().is_empty());
    assert_eq!(pour_cost(poured.ml), crate::energy::handling_cost(100));
}

#[test]
fn a_carried_container_changes_in_place() {
    let mut inventory = Inventory::default();
    let stand_ins = &mut crate::id::stand_ins();
    let cup = vessel(2, VesselForm::new(VesselKind::Cup));
    inventory
        .give(cup.clone(), StackLimits::NONE, stand_ins)
        .unwrap();
    let full = vessel(2, VesselForm::new(VesselKind::Cup).full_of(WATER));
    assert!(inventory.replace_carried(full.clone()));
    assert_eq!(inventory.find(Id::for_test(2)), Some(&full));
    assert!(!inventory.replace_carried(vessel(9, VesselForm::new(VesselKind::Cup))));
}
