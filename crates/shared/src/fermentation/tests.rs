use super::*;
use crate::cask::CaskSize;
use crate::fruit::{FruitDef, Fruits};
use crate::item::Quality;
use crate::material::Material;
use crate::vegetable::{VegetableDef, Vegetables};

fn fruit(key: &str) -> &'static FruitDef {
    Fruits::built_in().get(key).expect("a fruit")
}

fn grain(key: &str) -> &'static VegetableDef {
    Vegetables::built_in().get(key).expect("a vegetable")
}

#[test]
fn drinks_are_named_by_their_base() {
    let name = |base: Base| (base.kind(), base.name());
    let f = |key: &str| Base::Fruit(key.into());
    let g = |key: &str| Base::Grain(key.into());
    assert_eq!(name(f("apple")), (DrinkKind::Cider, "Cider".into()));
    assert_eq!(name(f("pear")), (DrinkKind::Perry, "Perry".into()));
    assert_eq!(name(f("grape")), (DrinkKind::Wine, "Wine".into()));
    let plum = fruit("plum").name.clone();
    assert_eq!(
        name(f("plum")),
        (DrinkKind::FruitWine, format!("{plum} Wine"))
    );
    assert_eq!(name(g("barley")), (DrinkKind::Ale, "Ale".into()));
    assert_eq!(name(g("wheat")), (DrinkKind::Ale, "Wheat Ale".into()));
}

#[test]
fn apple_juice_ferments_to_a_dry_cider() {
    let mash = Mash::juice(fruit("apple"), 20_000).unwrap();
    let cider = ferment(&mash, tolerance(Quality::Common)).unwrap();
    assert_eq!(cider.kind, DrinkKind::Cider);
    assert_eq!(cider.strength, 58);
    assert_eq!(cider.sweetness, Sweetness::Dry);
}

#[test]
fn strength_is_capped_by_the_yeast_and_the_rest_is_sweet() {
    // 1360 kcal a litre could make 20%.
    let mash = Mash {
        base: Base::Fruit("grape".into()),
        source: MashSource::Juice,
        kcal: 1360,
        ml: 1000,
    };
    let common = ferment(&mash, tolerance(Quality::Common)).unwrap();
    assert_eq!(common.strength, 90);
    assert_eq!(common.left_kcal, 1360 - 612);
    assert_eq!(common.sweetness, Sweetness::Sweet);
    let legendary = ferment(&mash, tolerance(Quality::Legendary)).unwrap();
    assert_eq!(legendary.strength, 170);
    assert!(tolerance(Quality::Poor) < tolerance(Quality::Common));
    assert_eq!(best_tolerance(&[Quality::Poor, Quality::Rare]), 130);
    assert_eq!(best_tolerance(&[]), WILD_TOLERANCE);
}

#[test]
fn eight_apples_a_litre_make_about_six_percent_and_one_in_a_butt_nothing() {
    let apple = fruit("apple");
    let eight = Mash::fruit(apple, 8 * apple.grams, 0).unwrap();
    let strength = ferment(&eight, tolerance(Quality::Common))
        .unwrap()
        .strength;
    assert!((55..=65).contains(&strength), "{strength}");
    let one_in_a_butt = Mash::fruit(apple, apple.grams, 480_000).unwrap();
    assert_eq!(ferment(&one_in_a_butt, tolerance(Quality::Artifact)), None);
}

#[test]
fn grain_ferments_directly_into_ale() {
    let barley = grain("barley");
    let mash = Mash::grain(barley, 7, 1000).unwrap();
    let ale = ferment(&mash, tolerance(Quality::Common)).unwrap();
    assert_eq!(ale.kind, DrinkKind::Ale);
    assert!((45..=60).contains(&ale.strength), "{}", ale.strength);
    assert_eq!(Mash::grain(barley, 7, 0), None);
    let not_grain = Vegetables::built_in()
        .iter()
        .find(|def| def.set != crate::vegetable::VegetableSet::Grain)
        .unwrap();
    assert_eq!(Mash::grain(not_grain, 7, 1000), None);
}

#[test]
fn cordial_is_half_as_sweet_as_juice() {
    let apple = fruit("apple");
    let juice = Mash::juice(apple, 1000).unwrap().kcal_per_litre();
    let cordial = Mash::cordial(apple, 1000).unwrap().kcal_per_litre();
    assert_eq!(cordial, juice / 2);
}

#[test]
fn time_by_kind_size_and_yeast() {
    let pin = |kind| ferment_secs(kind, CaskSize::Pin, 20_000, &[Quality::Common]);
    assert_eq!(pin(DrinkKind::Ale), 24 * GAME_DAY_SECS);
    assert_eq!(pin(DrinkKind::Cider), 36 * GAME_DAY_SECS);
    assert_eq!(pin(DrinkKind::Wine), 90 * GAME_DAY_SECS);
    let ale = |size, ml, yeasts: &[Quality]| ferment_secs(DrinkKind::Ale, size, ml, yeasts);
    let one = ale(CaskSize::Tun, 960_000, &[Quality::Common]);
    let many = ale(CaskSize::Tun, 960_000, &[Quality::Common; 48]);
    assert!(many < one);
    assert!(ale(CaskSize::Tun, 20_000, &[Quality::Common]) > pin(DrinkKind::Ale));
    assert!(ale(CaskSize::Pin, 20_000, &[Quality::Epic]) < pin(DrinkKind::Ale));
    assert!(ale(CaskSize::Pin, 20_000, &[]) > pin(DrinkKind::Ale));
    // Yeast beyond four times the usual pace does nothing more.
    let lots = ale(CaskSize::Pin, 20_000, &[Quality::Common; 4]);
    assert_eq!(lots, ale(CaskSize::Pin, 20_000, &[Quality::Common; 10]));
    assert_eq!(lots, 24 * GAME_DAY_SECS * 2 / 5);
}

#[test]
fn quality_from_yeast_making_and_wood() {
    let common = [Quality::Common];
    let q = |yeasts: &[Quality], source, wood| drink_quality(yeasts, source, wood);
    assert_eq!(
        q(&common, MashSource::Juice, Material::Oak),
        Quality::Common
    );
    assert_eq!(
        q(&common, MashSource::FruitInWater, Material::Oak),
        Quality::Poor
    );
    assert_eq!(q(&common, MashSource::Grain, Material::Pine), Quality::Poor);
    assert_eq!(q(&[], MashSource::Juice, Material::Oak), Quality::Poor);
    let mixed = [Quality::Poor, Quality::Rare];
    assert_eq!(mean_quality(&mixed), Quality::Common);
    let artifact = [Quality::Artifact];
    assert_eq!(
        q(&artifact, MashSource::Juice, Material::Oak),
        Quality::Legendary
    );
}

#[test]
fn ageing_by_wood_up_to_a_cap() {
    let wine = DrinkKind::Wine;
    let step = ageing_step_secs(wine);
    let age = |wood, secs| aged(Quality::Common, wine, wood, secs);
    assert_eq!(age(Material::Oak, step - 1), Quality::Common);
    assert_eq!(age(Material::Oak, step), Quality::Uncommon);
    assert_eq!(age(Material::Oak, step * 10), Quality::Epic);
    assert_eq!(age(Material::Cherry, step), Quality::Common);
    assert_eq!(age(Material::Cherry, step * 10), Quality::Uncommon);
    assert_eq!(age(Material::Chestnut, step * 10), Quality::Common);
    assert_eq!(age(Material::Pine, step * 10), Quality::Poor);
    let ale = aged(Quality::Common, DrinkKind::Ale, Material::Oak, u64::MAX);
    assert_eq!(ale, Quality::Uncommon);
    let best = aged(Quality::Legendary, wine, Material::Oak, u64::MAX);
    assert_eq!(best, Quality::Legendary);
}

#[test]
fn opened_drinks_keep_longer_when_stronger() {
    assert_eq!(best_before_secs(50, 0), Some(44_100));
    assert_eq!(best_before_secs(120, 0), Some(42 * GAME_DAY_SECS));
    assert_eq!(
        best_before_secs(199, 0).map(|s| s > 87 * GAME_DAY_SECS),
        Some(true)
    );
    assert_eq!(best_before_secs(KEEPS_FROM, 0), None);
    let sped = best_before_secs(50, vinegar_share(1000, 10_000)).unwrap();
    assert_eq!(sped, 44_100 * 6 / 10);
    let soured = best_before_secs(50, 1000).unwrap();
    assert_eq!(soured, 4410);
    assert_eq!(vinegar_share(5, 0), 1000);
}

#[test]
fn notice_in_the_last_tenth() {
    assert!(!notice_due(1000, 899));
    assert!(notice_due(1000, 900));
    assert!(notice_due(1000, 999));
    assert!(!notice_due(1000, 1000));
}

#[test]
fn opened_juice_turns_to_a_wild_cider() {
    let cider = wild_cider(fruit("apple"), 5000).unwrap();
    assert_eq!(cider.kind, DrinkKind::Cider);
    assert_eq!(cider.strength, 58);
    assert_eq!(wild_cider(fruit("apple"), 0), None);
}
