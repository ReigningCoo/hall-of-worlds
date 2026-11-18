use super::*;
use crate::ground::Load;
use crate::id::Id;
use crate::item::{Inventory, Item, ItemKind, StackLimits};
use crate::item_form::{Field, ItemForm, Refused, RollRequest};
use crate::material::Material;
use crate::potion::{Liquid, LiquidKind, Oil, Water};
use crate::rng::Seed;
use crate::skill::{SkillId, Skills};

fn cask_item(id: u128, form: CaskForm) -> Item {
    let mut item = Item::new(Id::for_test(id), ItemKind::Cask);
    item.form = Some(ItemForm::Cask(form));
    item
}

#[test]
fn sizes_roughly_double_and_get_heavier() {
    let litres = CaskSize::ALL.map(CaskSize::litres);
    assert_eq!(litres, [20, 40, 80, 160, 240, 320, 480, 960]);
    for pair in CaskSize::ALL.windows(2) {
        assert!(pair[0].oak_grams() < pair[1].oak_grams());
        let (a, b) = (pair[0].shape(), pair[1].shape());
        assert!(a.width * a.height < b.width * b.height, "{pair:?}");
    }
}

#[test]
fn the_wood_changes_the_weight_and_the_liquid_adds_to_it() {
    let oak = CaskForm::new(CaskSize::Pin, Material::Oak);
    assert_eq!(oak.grams(), 8_000);
    let pine = CaskForm::new(CaskSize::Pin, Material::Pine);
    assert!(pine.grams() < oak.grams());
    let acacia = CaskForm::new(CaskSize::Pin, Material::Acacia);
    assert!(acacia.grams() > oak.grams());
    let full = oak.full_of(Liquid::Water(Water::Plain));
    assert_eq!(full.ml, 20_000);
    assert_eq!(full.grams(), 28_000);
    assert_eq!(cask_item(1, full).own_weight(), 28_000);
}

#[test]
fn a_cask_holds_one_liquid_up_to_its_size() {
    let water = Liquid::Water(Water::Moon);
    let cask = CaskForm::new(CaskSize::Firkin, Material::Chestnut);
    assert_eq!(cask.name(), "Chestnut Firkin");
    assert!(!cask.opened, "sealed when new");
    let (cask, added) = cask.poured(water.clone(), 30_000);
    assert_eq!((cask.ml, added), (30_000, 30_000));
    assert!(cask.opened, "opened by adding to it");
    assert_eq!(cask.name(), "Chestnut Firkin of Moon Water");
    let (cask, added) = cask.poured(water.clone(), 30_000);
    assert_eq!((cask.ml, added), (40_000, 10_000), "only to the brim");
    assert!(!cask.takes(&water), "full");
    let half = CaskForm::new(CaskSize::Pin, Material::Oak).holding(Some(water), 5_000);
    let oil = Liquid::Oil(Oil::Nut);
    assert!(!half.takes(&oil), "never two liquids");
    let (same, added) = half.clone().poured(oil, 5_000);
    assert_eq!((same, added), (half, 0));
    let empty = CaskForm::new(CaskSize::Pin, Material::Oak).holding(None, 5);
    assert!(empty.is_empty() && empty.ml == 0);
}

#[test]
fn give_item_rolls_or_keeps_to_what_is_asked() {
    let mut rng = Seed::new(7).rng();
    let asked = RollRequest {
        cask_size: Some(CaskSize::Hogshead),
        material: Some(Material::Cherry),
        liquid: Some(LiquidKind::Water(Water::Salt)),
        ..RollRequest::NONE
    };
    let cask = cask_to(&mut rng, &asked).unwrap();
    assert_eq!(
        (cask.size, cask.wood, cask.litres()),
        (CaskSize::Hogshead, Material::Cherry, 240)
    );
    assert_eq!(cask.liquid, Some(Liquid::Water(Water::Salt)));
    let steel = RollRequest {
        material: Some(Material::Steel),
        ..RollRequest::NONE
    };
    assert_eq!(cask_to(&mut rng, &steel), Err(Refused(Field::Material)));
    let potion = RollRequest {
        liquid: Some(LiquidKind::Potion),
        ..RollRequest::NONE
    };
    assert_eq!(cask_to(&mut rng, &potion), Err(Refused(Field::Liquid)));
    for _ in 0..50 {
        let cask = cask_to(&mut rng, &RollRequest::NONE).unwrap();
        assert!(WOODS.contains(&cask.wood) && cask.is_empty());
    }
}

#[test]
fn a_pile_holding_a_cask_shows_the_biggest() {
    let small = cask_item(1, CaskForm::new(CaskSize::Pin, Material::Oak));
    let big = cask_item(2, CaskForm::new(CaskSize::Butt, Material::Pine));
    let bottle = Item::new(Id::for_test(3), ItemKind::Bottle);
    let load = Load::of(&[bottle.clone(), small, big]);
    assert_eq!(
        load,
        Load::Cask(CaskLook {
            size: CaskSize::Butt,
            wood: Material::Pine
        })
    );
    assert!(!matches!(Load::of(&[bottle]), Load::Cask(_)));
}

#[test]
fn collect_water_pours_into_a_carried_cask() {
    use crate::ability::Ability;
    let mut inventory = Inventory::default();
    assert!(!Ability::CollectWater.has_tool(&inventory));
    let stand_ins = &mut crate::id::stand_ins();
    let oil = CaskForm::new(CaskSize::Pin, Material::Oak).full_of(Liquid::Oil(Oil::Seed));
    inventory
        .give(cask_item(1, oil), StackLimits::NONE, stand_ins)
        .unwrap();
    assert!(!Ability::CollectWater.has_tool(&inventory), "a cask of oil");
    let moon = Liquid::Water(Water::Moon);
    let some = CaskForm::new(CaskSize::Pin, Material::Oak).holding(Some(moon.clone()), 4_000);
    inventory
        .give(cask_item(2, some), StackLimits::NONE, stand_ins)
        .unwrap();
    let empty = CaskForm::new(CaskSize::Pin, Material::Oak);
    inventory
        .give(cask_item(3, empty), StackLimits::NONE, stand_ins)
        .unwrap();
    assert!(Ability::CollectWater.has_tool(&inventory));
    assert_eq!(
        inventory.cask_to_fill(&moon),
        Some(Id::for_test(2)),
        "the one started"
    );
    let plain = Liquid::Water(Water::Plain);
    assert_eq!(inventory.cask_to_fill(&plain), Some(Id::for_test(3)));
    assert_eq!(
        fill_cask_with(&mut inventory, Id::for_test(2), moon.clone(), 30),
        16
    );
    let filled = inventory
        .find(Id::for_test(2))
        .and_then(Item::cask)
        .unwrap();
    assert_eq!(filled.litres(), 20);
    assert_eq!(
        fill_cask_with(&mut inventory, Id::for_test(1), moon, 5),
        0,
        "oil"
    );
}

#[test]
fn pours_grow_with_the_water_skill() {
    let mut skills = Skills::default();
    assert_eq!(cask_litres(&skills), 5);
    let water = SkillId::find(crate::potion::WATER_SKILL).expect("the skill tree has Water");
    skills.set_percent(water, 100);
    assert_eq!(cask_litres(&skills), 10);
    assert_eq!(cask_fill_ticks(), 5 * 20);
}

#[test]
fn casks_load_from_saves_in_litres_sealed() {
    let old = "(size: Pin, wood: Oak, liquid: Some(Water(Moon)), litres: 12)";
    let cask: CaskForm = ron::from_str(old).unwrap();
    assert_eq!((cask.ml, cask.opened), (12_000, false));
    let now = CaskForm::new(CaskSize::Pin, Material::Oak)
        .full_of(Liquid::Water(Water::Plain))
        .opened();
    let again: CaskForm = ron::from_str(&ron::to_string(&now).unwrap()).unwrap();
    assert_eq!(again, now);
    let wire: CaskForm = postcard::from_bytes(&postcard::to_allocvec(&now).unwrap()).unwrap();
    assert_eq!(wire, now);
}
