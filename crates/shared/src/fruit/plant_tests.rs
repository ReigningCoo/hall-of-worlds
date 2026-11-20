//! Fruit plants: growing, picking, harvests, orchards and trunks.

use super::*;
use crate::dungeon::{Cell, Level};
use crate::herb::{Growth, MINUTES_PER_DAY};
use crate::rng::Seed;
use crate::site::{FieldLook, Rect};

fn def(key: &str) -> &'static FruitDef {
    Fruits::built_in().get(key).unwrap()
}

fn plant(key: &str, phase: u32) -> FruitPlant {
    FruitPlant {
        fruit: key.into(),
        phase,
        picked: None,
    }
}

/// Families grow at their speeds, and the giants very slowly.
#[test]
fn growth_follows_the_family() {
    assert_eq!(def("wild_strawberry").growth(), Growth::Fast);
    assert_eq!(def("apple").growth(), Growth::Slow);
    assert_eq!(def("mango").growth(), Growth::VerySlow);
    for key in GIANTS {
        assert!(Fruits::built_in().get(key).is_some(), "{key}");
    }
}

/// Young for a round, then leaves, flowers, unripe and ripe fruit a quarter
/// of each round, and picking starts a round again from its leaves.
#[test]
fn a_plant_goes_round_its_stages() {
    let apple = def("apple");
    let round = round_minutes(apple);
    assert_eq!(round, 8 * MINUTES_PER_DAY);
    let young = plant("apple", 0);
    assert_eq!(young.stage(apple, 0), FruitStage::Young);
    let stages: Vec<FruitStage> = (0..4)
        .map(|q| young.stage(apple, round + q * round / 4))
        .collect();
    assert_eq!(
        stages,
        [
            FruitStage::Leafy,
            FruitStage::Flowering,
            FruitStage::Fruiting { ripe: false },
            FruitStage::Fruiting { ripe: true },
        ]
    );
    let ripe_at = round + 3 * round / 4;
    assert!(young.can_pick(apple, ripe_at));
    assert!(!young.can_pick(apple, round));
    // A life later it is young again.
    assert_eq!(
        young.stage(apple, round * (1 + LIFE_ROUNDS)),
        FruitStage::Young
    );
    let picked = FruitPlant {
        picked: Some(ripe_at),
        ..young
    };
    assert_eq!(picked.stage(apple, ripe_at), FruitStage::Leafy);
    assert!(!picked.can_pick(apple, ripe_at + round / 2));
    assert!(picked.can_pick(apple, ripe_at + 3 * round / 4));
}

/// The harvest tier untrained; Foraging adds to the most, Orchards to both.
#[test]
fn harvests_grow_with_skill() {
    let apple = def("apple");
    assert_eq!(harvest_range(apple, 0, 0), apple.harvest.range());
    let (least, most) = apple.harvest.range();
    assert_eq!(harvest_range(apple, 10_000, 0), (least, most + 4));
    assert_eq!(harvest_range(apple, 0, 10_000), (least + 4, most + 4));
    let mut rng = Seed::new(3).rng();
    for _ in 0..50 {
        let count = harvest(apple, (0, 0), &mut rng);
        assert!((least..=most).contains(&count));
    }
}

/// Orchards grow rows of fruit suited to the warmth inside their fence,
/// trunks apart, the edge left clear; other fields grow nothing.
#[test]
fn orchards_grow_rows_that_suit_the_place() {
    let field = |orchard| FieldLook {
        rect: Rect {
            x: 0,
            y: 0,
            width: 10,
            height: 9,
        },
        livestock: false,
        variety: 1,
        orchard,
        pen: None,
    };
    let fruits = Fruits::built_in();
    assert!(plant_orchards(fruits, &[field(false)], 12, Seed::new(5)).is_empty());
    for celsius in [-12, 0, 12, 26, 40] {
        let plants = plant_orchards(fruits, &[field(true)], celsius, Seed::new(5));
        assert!(!plants.is_empty());
        let rows: std::collections::BTreeSet<i64> = plants.iter().map(|((_, y), _)| *y).collect();
        assert_eq!(rows.into_iter().collect::<Vec<_>>(), [1, 4, 7]);
        for ((x, y), plant) in &plants {
            assert!((1..9).contains(x) && (1..8).contains(y));
            let def = fruits.get(&plant.fruit).unwrap();
            assert!(def.is_item());
            if def.plant.has_trunk() {
                assert_eq!((x - 1) % ROW_GAP, 0, "{}", def.key);
            }
        }
        if celsius == 12 {
            assert!(
                plants
                    .iter()
                    .all(|(_, p)| fruits.get(&p.fruit).unwrap().suits_warmth(12))
            );
        }
    }
}

/// A plant with a trunk is in the way as a wild tree: not walked on, but
/// stood beside; one without is walked through.
#[test]
fn trunks_block_like_trees() {
    let mut level = Level::new(0, 0, 3, 1);
    for x in 0..3 {
        level.set(x, 0, Cell::Grass);
    }
    level.set_fruit(0, 0, plant("apple", 0));
    level.set_fruit(1, 0, plant("wild_strawberry", 0));
    assert!(level.has_trunk(0, 0) && !level.has_trunk(1, 0));
    assert!(!level.is_walkable(0, 0) && level.is_walkable(1, 0));
    assert!(crate::clearance::has_footing(&level, 0, 0));
    let mut blockers = Vec::new();
    crate::clearance::cell_blockers(&level, 0, 0, &mut blockers);
    assert_eq!(blockers.len(), 1);
    blockers.clear();
    crate::clearance::cell_blockers(&level, 1, 0, &mut blockers);
    assert!(blockers.is_empty());
}
