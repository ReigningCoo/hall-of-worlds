use super::*;
use crate::blade::Hands;
use crate::creature::CreatureKind;
use crate::dungeon::CARRIED_LIGHT;
use crate::id::Id;
use crate::item::{Inventory, Item, ItemKind, MoveRefused, Quality, StackLimits};
use crate::item_form::{ItemForm, ItemOrder, make_item};
use crate::potion::{BottleForm, Liquid, Oil, Water};
use crate::rng::Seed;
use crate::wear::WearRefused;

fn id(n: u128) -> Id {
    Id::for_test(n)
}

/// A lantern with id `n` holding `ml` of oil.
fn lantern(n: u128, ml: u32) -> Item {
    let mut item = Item::new(id(n), ItemKind::Lantern);
    item.form = Some(ItemForm::Lantern(LanternForm { ml, burnt: 0 }));
    item
}

fn bottle(n: u128, liquid: Option<Liquid>) -> Item {
    let mut item = Item::new(id(n), ItemKind::Bottle);
    item.form = Some(ItemForm::Bottle(BottleForm::new(liquid, 7)));
    item
}

fn oil(n: u128) -> Item {
    bottle(n, Some(Liquid::Oil(Oil::Seed)))
}

/// A two-handed sword with id `n`.
fn two_handed(n: u128) -> Item {
    let order = ItemOrder::plain(ItemKind::Longsword, Quality::Common);
    let mut new_id = || id(1000 + n);
    let seed = Seed::new(n as u64);
    let mut item = make_item(&order, CreatureKind::Human, id(n), seed, None, &mut new_id);
    if let Some(ItemForm::Blade(form)) = &mut item.form {
        form.hands = Hands::Two;
    }
    item
}

/// A human with a roomy grid carrying these, in order.
fn carrying(items: Vec<Item>) -> Inventory {
    let mut inventory = Inventory {
        creature: CreatureKind::Human,
        size: (20, 20),
        ..Inventory::default()
    };
    for item in items {
        inventory
            .give(item, StackLimits::NONE, &mut crate::id::stand_ins())
            .expect("room");
    }
    inventory
}

/// Wearing a Rare belt (id 1, three slots), carrying these after it.
fn belted(items: Vec<Item>) -> Inventory {
    let mut belt = Item::new(id(1), ItemKind::Belt);
    belt.quality = Some(Quality::Rare);
    let mut all = vec![belt];
    all.extend(items);
    let mut inventory = carrying(all);
    inventory.wear(id(1)).unwrap();
    inventory
}

#[test]
fn a_new_lantern_is_empty_and_an_old_one_without_a_form_too() {
    assert_eq!(
        Item::new(id(1), ItemKind::Lantern).lantern(),
        Some(LanternForm::default())
    );
    assert_eq!(Item::new(id(1), ItemKind::Knife).lantern(), None);
    let full = lantern(1, FOUNT_ML).lantern().unwrap();
    assert_eq!(full.room(), 0);
    assert_eq!(
        full.seconds_left(),
        2 * 3600,
        "a full fount burns two hours"
    );
}

#[test]
fn oil_weighs_a_gram_a_millilitre() {
    let empty = ItemKind::Lantern.def().weight;
    let form = ItemForm::Lantern(LanternForm { ml: 150, burnt: 0 });
    assert_eq!(
        form.grams_at(crate::creature::Size::Medium),
        Some(empty + 150)
    );
}

#[test]
fn it_burns_a_hundred_ml_an_hour_and_carries_part_seconds() {
    let mut fount = LanternForm { ml: 100, burnt: 0 };
    assert_eq!(fount.burn(35), Burnt::default(), "not a whole ml yet");
    assert_eq!((fount.ml, fount.burnt), (100, 35));
    let burnt = fount.burn(1);
    assert!(burnt.used && !burnt.went_out);
    assert_eq!((fount.ml, fount.burnt), (99, 0));
    fount.burn(3600);
    assert_eq!(fount.ml, 0, "a bottle lasts an hour");
    assert_eq!(fount.burnt, 0);
    assert_eq!(fount.burn(100), Burnt::default(), "dry stays dry");

    let mut fount = LanternForm { ml: 10, burnt: 20 };
    let burnt = fount.burn(10_000);
    assert!(burnt.used && burnt.went_out);
    assert_eq!((fount.ml, fount.burnt), (0, 0));
}

#[test]
fn only_held_or_tied_lanterns_burn_and_light() {
    let mut inventory = belted(vec![lantern(2, 100), lantern(3, 100)]);
    assert!(!inventory.lantern_lit(), "carried in the grid");
    assert_eq!(inventory.light_reach(), CARRIED_LIGHT);
    assert_eq!(inventory.burn_lanterns(3600), Burnt::default());
    assert_eq!(inventory.find(id(2)).unwrap().lantern().unwrap().ml, 100);

    inventory.wear(id(2)).unwrap();
    assert!(inventory.lantern_lit());
    assert_eq!(inventory.light_reach(), LANTERN_LIGHT);
    inventory.tie_bag(id(3), 0).unwrap();
    assert_eq!(inventory.burning_places().count(), 2);

    let burnt = inventory.burn_lanterns(1800);
    assert!(burnt.used && !burnt.went_out);
    for lantern in [2, 3] {
        let fount = inventory.find(id(lantern)).unwrap().lantern().unwrap();
        assert_eq!(fount.ml, 50, "each burns its own oil");
    }
    let burnt = inventory.burn_lanterns(1800);
    assert!(burnt.went_out);
    assert!(!inventory.lantern_lit());
    assert_eq!(inventory.light_reach(), CARRIED_LIGHT);
}

#[test]
fn an_empty_lantern_gives_no_light() {
    let mut inventory = carrying(vec![lantern(1, 0)]);
    inventory.wear(id(1)).unwrap();
    assert_eq!(inventory.burning_places().count(), 1);
    assert!(!inventory.lantern_lit());
}

#[test]
fn only_one_lantern_is_tied_and_it_adds_no_cells() {
    let mut inventory = belted(vec![lantern(2, 50), lantern(3, 50)]);
    let cells = inventory.grid_cells();
    inventory.tie_bag(id(2), 0).unwrap();
    assert!(inventory.is_tied(id(2)));
    assert!(!inventory.adds_to_grid(id(2)));
    assert_eq!(inventory.grid_cells(), cells);
    assert_eq!(inventory.tie_bag(id(3), 1), Err(MoveRefused::HolderTied));
    assert_eq!(
        inventory.tie_bag(id(3), 0),
        Ok(()),
        "swapped on its own slot"
    );
    assert!(inventory.is_tied(id(3)) && !inventory.is_tied(id(2)));
    assert_eq!(inventory.check_lift(id(3)), Ok(()));
}

#[test]
fn held_it_takes_a_hand_so_no_two_handed_weapon_with_it() {
    let mut inventory = carrying(vec![lantern(1, 100), two_handed(2)]);
    inventory.wear(id(1)).unwrap();
    assert_eq!(inventory.wear(id(2)), Err(WearRefused::Clash(id(1))));
    let mut inventory = carrying(vec![lantern(1, 100), two_handed(2)]);
    inventory.wear(id(2)).unwrap();
    assert_eq!(inventory.wear(id(1)), Err(WearRefused::Clash(id(2))));
}

#[test]
fn a_bottle_of_oil_fills_as_much_as_fits_and_comes_back_empty() {
    let filled = fill(&lantern(1, 0), &oil(2)).unwrap();
    assert_eq!((filled.ml, filled.spilt), (100, 0));
    assert_eq!(filled.lantern.lantern().unwrap().ml, 100);
    assert_eq!(filled.bottle.id, id(2));
    let Some(ItemForm::Bottle(empty)) = &filled.bottle.form else {
        panic!("still a bottle");
    };
    assert!(empty.is_empty());

    let filled = fill(&lantern(1, 150), &oil(2)).unwrap();
    assert_eq!((filled.ml, filled.spilt), (50, 50));
    assert_eq!(filled.lantern.lantern().unwrap().ml, FOUNT_ML);

    // A lantern made before it had a form fills too.
    let old = Item::new(id(1), ItemKind::Lantern);
    assert_eq!(
        fill(&old, &oil(2)).unwrap().lantern.lantern().unwrap().ml,
        100
    );
}

#[test]
fn only_oil_fills_and_never_a_full_lantern() {
    let water = bottle(2, Some(Liquid::Water(Water::Plain)));
    assert_eq!(fill(&lantern(1, 0), &water), Err(FillRefused::NotOil));
    assert_eq!(
        fill(&lantern(1, 0), &bottle(2, None)),
        Err(FillRefused::NotOil)
    );
    assert_eq!(fill(&lantern(1, FOUNT_ML), &oil(2)), Err(FillRefused::Full));
    assert_eq!(fill(&oil(3), &oil(2)), Err(FillRefused::NotALantern));
}
