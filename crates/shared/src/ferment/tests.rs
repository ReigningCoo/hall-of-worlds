use super::*;
use crate::cask::{CaskForm, CaskSize};
use crate::fermentation::{Base, DrinkKind, GAME_DAY_SECS, JUICE_TURNS_SECS, best_before_secs};
use crate::fruit::Fruits;
use crate::id::Id;
use crate::item::{Inventory, Item, ItemKind, Quality, StackLimits};
use crate::item_form::ItemForm;
use crate::material::Material;
use crate::potion::{Liquid, Water};
use crate::pour::{PourRefused, pour};
use crate::vegetable::Vegetables;

const DAY: u64 = GAME_DAY_SECS;

fn cask_of(liquid: Liquid, wood: Material) -> Item {
    let form = CaskForm::new(CaskSize::Pin, wood).full_of(liquid);
    Item {
        form: Some(ItemForm::Cask(form)),
        ..Item::new(Id::for_test(1), ItemKind::Cask)
    }
}

fn apple_juice() -> Item {
    cask_of(Liquid::Juice("apple".into()), Material::Oak)
}

fn form(item: &Item) -> &CaskForm {
    item.cask().unwrap()
}

fn drink(item: &Item) -> crate::fermentation::Drink {
    match &form(item).liquid {
        Some(Liquid::Drink(drink)) => (**drink).clone(),
        other => panic!("not a drink: {other:?}"),
    }
}

/// `item` with Ferment started on it with one Common yeast.
fn started(item: &Item) -> Item {
    let plan = plan(item, None, &[Quality::Common]).unwrap();
    Item {
        form: Some(ItemForm::Cask(plan.started(form(item)))),
        ..item.clone()
    }
}

fn passed(item: &mut Item, secs: u64) -> CaskSettled {
    match &mut item.form {
        Some(ItemForm::Cask(cask)) => cask.pass(secs),
        _ => panic!("not a cask"),
    }
}

#[test]
fn juice_ferments_into_a_dry_cider_with_a_common_yeast() {
    let plan = plan(&apple_juice(), None, &[Quality::Common]).unwrap();
    assert_eq!(plan.drink.base, Base::Fruit("apple".into()));
    assert_eq!(plan.drink.base.kind(), DrinkKind::Cider);
    assert_eq!(plan.drink.strength, 58);
    assert_eq!(plan.drink.quality, Quality::Common);
    assert_eq!(plan.ml, 20_000);
    assert_eq!(plan.secs, 36 * DAY);
}

#[test]
fn water_needs_fruit_or_grain_and_yeast() {
    let water = cask_of(Liquid::Water(Water::Plain), Material::Oak);
    let common = [Quality::Common];
    assert_eq!(
        plan(&water, None, &common),
        Err(FermentRefused::NeedsAdding)
    );
    let salt = cask_of(Liquid::Water(Water::Salt), Material::Oak);
    assert_eq!(plan(&salt, None, &common), Err(FermentRefused::Nothing));
    let barley = Vegetables::built_in().get("barley").unwrap();
    let sheaves = crate::vegetable::new_item(barley, 150, Id::for_test(2), None);
    assert_eq!(
        plan(&water, Some(&sheaves), &[]),
        Err(FermentRefused::NoYeast)
    );
    let ale = plan(&water, Some(&sheaves), &common).unwrap();
    assert_eq!(ale.drink.name(), "Ale");
    assert!(ale.drink.strength >= 10);
    // Juice takes nothing more.
    assert_eq!(
        plan(&apple_juice(), Some(&sheaves), &common),
        Err(FermentRefused::TakesNothing)
    );
}

#[test]
fn one_apple_in_a_cask_of_water_is_too_weak() {
    let water = cask_of(Liquid::Water(Water::Plain), Material::Oak);
    let apple = Fruits::built_in().get("apple").unwrap();
    let one = crate::fruit::new_item(apple, 1, Id::for_test(2), None);
    assert_eq!(
        plan(&water, Some(&one), &[Quality::Common]),
        Err(FermentRefused::TooWeak)
    );
}

#[test]
fn a_fermenting_cask_is_sealed_and_left_alone_until_done() {
    let mut cask = started(&apple_juice());
    assert!(!form(&cask).opened && form(&cask).time.is_fermenting());
    let tankard = Item {
        form: Some(ItemForm::Vessel(crate::vessel::VesselForm::new(
            crate::vessel::VesselKind::Tankard,
        ))),
        ..Item::new(Id::for_test(3), ItemKind::Vessel)
    };
    assert_eq!(pour(&cask, &tankard, true), Err(PourRefused::Fermenting));
    assert_eq!(
        plan(&cask, None, &[Quality::Common]),
        Err(FermentRefused::Fermenting)
    );
    assert!(passed(&mut cask, 35 * DAY).seen);
    assert!(form(&cask).time.is_fermenting());
    assert!(passed(&mut cask, DAY).seen);
    assert!(!form(&cask).time.is_fermenting());
    assert!(pour(&cask, &tankard, true).is_ok());
}

#[test]
fn a_sealed_cider_ages_up_a_step_in_oak_and_never_spoils() {
    let mut cask = started(&apple_juice());
    passed(&mut cask, 36 * DAY + 13 * DAY);
    assert_eq!(drink(&cask).quality, Quality::Common);
    assert!(passed(&mut cask, DAY).seen);
    assert_eq!(drink(&cask).quality, Quality::Uncommon);
    // Capped at a step, and sealed it never turns.
    passed(&mut cask, 1000 * DAY);
    assert_eq!(drink(&cask).quality, Quality::Uncommon);
}

#[test]
fn an_opened_drink_warns_once_then_turns_to_vinegar() {
    let mut cask = started(&apple_juice());
    passed(&mut cask, 36 * DAY);
    if let Some(ItemForm::Cask(form)) = &mut cask.form {
        *form = form.clone().opened();
    }
    let best = best_before_secs(58, 0).unwrap();
    assert_eq!(form(&cask).turns_in(), Some(best));
    let near = best - best / 10;
    assert!(passed(&mut cask, near - 1).warned.is_empty());
    assert_eq!(passed(&mut cask, 1).warned.len(), 1);
    assert!(passed(&mut cask, 1).warned.is_empty());
    passed(&mut cask, best);
    assert_eq!(
        form(&cask).liquid,
        Some(Liquid::Vinegar(Box::new(Base::Fruit("apple".into()))))
    );
    assert_eq!(form(&cask).liquid.as_ref().unwrap().name(), "Cider Vinegar");
}

#[test]
fn vinegar_poured_in_brings_the_end_nearer() {
    let mut cask = started(&apple_juice());
    passed(&mut cask, 36 * DAY);
    if let Some(ItemForm::Cask(form)) = &mut cask.form {
        *form = form.clone().opened().holding(form.liquid.clone(), 10_000);
    }
    let before = form(&cask).turns_in().unwrap();
    let vinegar = Item {
        id: Id::for_test(5),
        ..cask_of(
            Liquid::Vinegar(Box::new(Base::Fruit("apple".into()))),
            Material::Oak,
        )
    };
    let vinegar = Item {
        form: Some(ItemForm::Cask(
            form(&vinegar)
                .clone()
                .holding(form(&vinegar).liquid.clone(), 1000),
        )),
        ..vinegar
    };
    let poured = pour(&vinegar, &cask, true).unwrap();
    assert!(drink(&poured.into).vinegar > 0);
    assert!(form(&poured.into).turns_in().unwrap() < before);
}

#[test]
fn opened_juice_ferments_wild_into_a_poor_cider() {
    let mut cask = apple_juice();
    if let Some(ItemForm::Cask(form)) = &mut cask.form {
        *form = form.clone().opened();
    }
    passed(&mut cask, JUICE_TURNS_SECS - 1);
    assert_eq!(form(&cask).liquid, Some(Liquid::Juice("apple".into())));
    assert!(passed(&mut cask, 1).seen);
    let cider = drink(&cask);
    assert_eq!(
        (cider.name().as_str(), cider.quality),
        ("Cider", Quality::Poor)
    );
    assert!(form(&cask).turns_in().is_some());
}

#[test]
fn casks_on_the_ground_go_by_real_time() {
    let mut cask = started(&apple_juice());
    cask.lay_casks_down(1000);
    assert!(cask.settle_lying_casks(1000 + 36 * DAY).seen);
    assert!(!form(&cask).time.is_fermenting());
    // Picked up: the time since it was last settled counts once.
    cask.age_casks(0, 1000 + 37 * DAY);
    assert_eq!(form(&cask).time.aged, DAY);
    assert_eq!(form(&cask).time.lying_since, None);
}

#[test]
fn ferment_uses_yeasts_without_using_them_up() {
    let limits = StackLimits::default();
    let ids = &mut crate::id::stand_ins();
    let mut inventory = Inventory::default();
    for n in 0..3 {
        let one = crate::yeast::new_item(Quality::Common, Id::for_test(10 + n), None);
        inventory.give(one, limits, ids).unwrap();
    }
    let stack = inventory.items[0].item.id;
    assert_eq!(inventory.usable_yeasts(stack, 9).len(), 3);
    assert_eq!(inventory.use_yeasts(stack, 2), 2);
    assert_eq!(inventory.usable_yeasts(stack, 9).len(), 1);
    assert_eq!(inventory.items[0].item.count(), 3);
    let choices = inventory.ferment_choices();
    assert_eq!(choices.yeasts.len(), 1);
    assert_eq!(choices.yeasts[0].usable, 1);
}

#[test]
fn a_cask_started_on_the_ground_keeps_going_by_world_time() {
    let mut juice = apple_juice();
    juice.lay_casks_down(500);
    let mut cask = started(&juice);
    assert_eq!(form(&cask).time.lying_since, Some(500));
    cask.settle_lying_casks(500 + 36 * DAY);
    assert!(!form(&cask).time.is_fermenting());
}
