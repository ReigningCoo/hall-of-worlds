//! Vegetable plants: growing, picking, harvests, places and farms' fields.

use super::*;
use crate::climate::Biome;
use crate::herb::{MINUTES_PER_DAY, STAGES, growth_stage_minutes};
use crate::rng::Seed;
use crate::site::{FieldLook, Rect};

fn def(key: &str) -> &'static VegetableDef {
    Vegetables::built_in().get(key).unwrap()
}

fn plant(key: &str, phase: u32) -> VegetablePlant {
    VegetablePlant {
        vegetable: key.into(),
        phase,
        picked: None,
    }
}

/// A plant grows up through the stages at its speed, can be picked only full
/// grown, and dug up it is gone for a whole growth.
#[test]
fn a_dug_plant_grows_and_is_gone() {
    let turnip = def("turnip");
    let step = growth_stage_minutes(turnip.growth);
    assert_eq!(step * u64::from(STAGES), 2 * MINUTES_PER_DAY);
    let mut sown = plant("turnip", 0);
    let stages: Vec<u8> = (0..5).map(|s| sown.stage(turnip, s * step)).collect();
    assert_eq!(stages, [1, 2, 3, 4, 4]);
    assert!(!sown.can_pick(turnip, 2 * step));
    let ripe = 4 * step;
    assert!(sown.can_pick(turnip, ripe));
    sown.picked = Some(ripe);
    assert_eq!(sown.stage(turnip, ripe + 1), 0);
    assert!(!sown.can_pick(turnip, ripe + 1));
    assert_eq!(sown.stage(turnip, ripe + 4 * step), 1);
}

/// Picked leaves come back a stage short of full grown, so they can't be
/// picked again until they have grown that stage.
#[test]
fn picked_leaves_grow_back() {
    let kale = def("kale");
    assert_eq!(kale.regrow, crate::herb::Regrow::Pick);
    let step = growth_stage_minutes(kale.growth);
    let mut leaves = plant("kale", 0);
    let ripe = 4 * step;
    assert!(leaves.can_pick(kale, ripe));
    leaves.picked = Some(ripe);
    assert_eq!(leaves.stage(kale, ripe), 3);
    assert!(!leaves.can_pick(kale, ripe + step - 1));
    assert!(leaves.can_pick(kale, ripe + step));
}

/// A harvest is its tier untrained, Foraging adding to the most and Crops to
/// both.
#[test]
fn harvests_follow_the_tier_and_skills() {
    let turnip = def("turnip");
    assert_eq!(harvest_range(turnip, 0, 0), turnip.harvest.range());
    let (least, most) = turnip.harvest.range();
    assert_eq!(harvest_range(turnip, 10_000, 0), (least, most + 4));
    assert_eq!(harvest_range(turnip, 0, 10_000), (least + 4, most + 4));
    let mut rng = Seed::new(7).rng();
    for _ in 0..50 {
        let count = harvest(turnip, (0, 0), &mut rng);
        assert!((least..=most).contains(&count), "{count}");
        assert!(count >= 1);
    }
}

/// Every biome farms some crop, never a wild-only one.
#[test]
fn every_biome_farms_cultivated_crops() {
    let vegetables = Vegetables::built_in();
    for biome in Biome::ALL {
        let crops = field_crops(vegetables, biome);
        assert!(!crops.is_empty(), "{biome:?}");
        assert!(crops.iter().all(|def| !def.wild_only), "{biome:?}");
        assert!(crops.iter().all(|def| def.grows_wild(biome)), "{biome:?}");
        let wild = wild_crops(vegetables, biome);
        assert!(wild.iter().all(|def| !def.wild_only));
        let only = wild_only_vegetables(vegetables, biome);
        assert!(only.iter().all(|def| def.wild_only));
    }
}

fn field(x: i64, livestock: bool, orchard: bool) -> FieldLook {
    FieldLook {
        rect: Rect {
            x,
            y: 0,
            width: 4,
            height: 3,
        },
        livestock,
        variety: 0,
        orchard,
    }
}

/// Each field of crops grows one crop suiting the farm on every cell, all
/// sown together; pasture and orchards grow none; the same whenever made.
#[test]
fn fields_grow_one_crop_each() {
    let vegetables = Vegetables::built_in();
    let fields = [
        field(0, false, false),
        field(10, true, false),
        field(20, false, true),
        field(30, false, false),
    ];
    let biome = Biome::SeasonalForest;
    let plants = plant_fields(vegetables, &fields, biome, Seed::new(42));
    assert_eq!(
        plants,
        plant_fields(vegetables, &fields, biome, Seed::new(42))
    );
    assert_eq!(plants.len(), 2 * 12);
    for sown in [&fields[0], &fields[3]] {
        let here: Vec<_> = (plants.iter())
            .filter(|((x, y), _)| sown.contains(*x, *y))
            .map(|(_, plant)| plant)
            .collect();
        assert_eq!(here.len(), 12);
        assert!(here.iter().all(|plant| *plant == here[0]));
        let def = vegetables.get(&here[0].vegetable).unwrap();
        assert!(!def.wild_only && def.grows_wild(biome), "{}", def.key);
    }
    // Over many farms, several crops are grown.
    let kinds: std::collections::BTreeSet<_> = (0..40)
        .flat_map(|seed| plant_fields(vegetables, &fields[..1], biome, Seed::new(seed)))
        .map(|(_, plant)| plant.vegetable)
        .collect();
    assert!(kinds.len() >= 5, "{kinds:?}");
}
