use super::*;
use crate::id::Id;
use crate::item::{Item, ItemKind};
use crate::item_form::{Field, ItemForm, Refused, RollRequest};
use crate::potion::{Liquid, LiquidKind, Oil, Water};
use crate::rng::Seed;
use crate::site::BuildingKind;

fn vessel_item(form: VesselForm) -> Item {
    Item {
        form: Some(ItemForm::Vessel(form)),
        ..Item::new(Id::for_test(1), ItemKind::Vessel)
    }
}

#[test]
fn vessels_hold_what_was_asked_in_metric() {
    let ml = VesselKind::ALL.map(VesselKind::most_ml);
    assert_eq!(ml, [200, 250, 300, 500, 500, 750, 1500, 2000]);
    assert!(ml.is_sorted(), "smallest first");
    assert_eq!(VesselKind::Tankard.most_ml(), 500);
    assert_eq!(VesselKind::Goblet.most_ml(), 200);
    assert_eq!(VesselKind::WineBottle.most_ml(), 750);
}

#[test]
fn a_vessel_weighs_what_is_in_it_and_takes_its_cells() {
    let jug = VesselForm::new(VesselKind::Jug);
    assert_eq!(vessel_item(jug.clone()).own_weight(), 1100);
    let full = jug.full_of(Liquid::Water(Water::Plain));
    assert_eq!(full.ml, 1500);
    let item = vessel_item(full);
    assert_eq!(item.own_weight(), 2600);
    assert_eq!(item.name(), "Jug of Water");
    assert_eq!(item.shape(), VesselKind::Jug.shape());
}

#[test]
fn only_a_wine_bottle_seals() {
    let water = Liquid::Water(Water::Plain);
    let bottle = VesselForm::new(VesselKind::WineBottle).full_of(water.clone());
    assert!(bottle.clone().sealed().sealed);
    assert!(
        !VesselForm::new(VesselKind::WineBottle).sealed().sealed,
        "empty"
    );
    let tankard = VesselForm::new(VesselKind::Tankard).full_of(water);
    assert!(!tankard.sealed().sealed);
}

#[test]
fn water_is_drunk_a_serving_at_a_time() {
    let skin = VesselForm::new(VesselKind::Waterskin).full_of(Liquid::Water(Water::Moon));
    let (skin, ml) = skin.drunk();
    assert_eq!((skin.ml, ml), (1750, SERVING_ML));
    let goblet = VesselForm::new(VesselKind::Goblet).full_of(Liquid::Water(Water::Plain));
    let (goblet, ml) = goblet.drunk();
    assert_eq!(ml, 200, "what is left");
    assert!(goblet.is_empty());
    let oil = VesselForm::new(VesselKind::Jug).full_of(Liquid::Oil(Oil::Nut));
    assert!(!oil.drinkable());
    assert_eq!(oil.clone().drunk(), (oil, 0));
}

#[test]
fn give_item_rolls_or_keeps_to_what_is_asked() {
    let mut rng = Seed::new(3).rng();
    let asked = RollRequest {
        vessel: Some(VesselKind::Flask),
        liquid: Some(LiquidKind::Oil(Oil::Seed)),
        ..RollRequest::NONE
    };
    let flask = vessel_to(&mut rng, &asked).unwrap();
    assert_eq!((flask.vessel, flask.ml), (VesselKind::Flask, 500));
    let empty = vessel_to(&mut rng, &RollRequest::NONE).unwrap();
    assert!(empty.is_empty());
    let potion = RollRequest {
        liquid: Some(LiquidKind::Potion),
        ..RollRequest::NONE
    };
    assert_eq!(vessel_to(&mut rng, &potion), Err(Refused(Field::Liquid)));
}

#[test]
fn shops_make_their_own_vessels() {
    for shop in [
        BuildingKind::Carpenter,
        BuildingKind::Leatherworker,
        BuildingKind::Jeweller,
        BuildingKind::Herbalist,
    ] {
        assert!(!vessel_for_shop(shop).is_empty(), "{shop:?}");
        let sells = crate::shop::sells(shop);
        assert!(sells.iter().any(|&(kind, _)| kind == ItemKind::Vessel));
        let asked = crate::shop::stock_request(shop, ItemKind::Vessel, Seed::new(9));
        assert!(vessel_for_shop(shop).contains(&asked.vessel.unwrap()));
    }
    let every: Vec<VesselKind> = (BuildingKind::ALL.iter())
        .flat_map(|&shop| vessel_for_shop(shop).iter().copied())
        .collect();
    for vessel in VesselKind::ALL {
        assert!(every.contains(&vessel), "someone makes {vessel:?}");
    }
}
