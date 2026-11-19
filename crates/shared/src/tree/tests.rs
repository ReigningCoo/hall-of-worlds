use super::fruit_wood::{FRUIT_KINDLING, FRUIT_WOODS};
use super::*;
use crate::climate::Biome;
use crate::fruit::Fruits;
use crate::herb::MINUTES_PER_DAY;

fn setting(biome: Biome, celsius: i32) -> Setting {
    Setting {
        biome,
        celsius,
        narrow: 500,
        lush: 0,
        near_water: false,
    }
}

/// How many of 2,000 cells in `setting` grow each kind.
fn counts(setting: Setting) -> std::collections::HashMap<TreeKind, u32> {
    let mut counts = std::collections::HashMap::new();
    for cell in 0..2000u64 {
        let hash = crate::rng::Seed::new(7).at(&[cell as i64, 3]);
        *counts.entry(TreeKind::roll(setting, hash)).or_default() += 1;
    }
    counts
}

#[test]
fn the_table_is_in_the_enums_order_with_tree_woods() {
    for (place, def) in TREES.iter().enumerate() {
        assert_eq!(def.kind as usize, place, "{}", def.name);
        assert!((1..=12).contains(&def.logs), "{}", def.name);
        assert!(def.wood.wood().is_some(), "{} has a tree's wood", def.name);
        let (low, high) = def.celsius;
        assert!(low < high && !def.biomes.is_empty(), "{}", def.name);
    }
}

#[test]
fn every_trunked_fruit_tree_has_a_wood_but_three() {
    let fruits = Fruits::built_in();
    let no_wood = ["banana", "papaya", "saguaro_fruit"];
    for def in fruits.iter().filter(|def| def.plant.has_trunk()) {
        let listed = fruit_wood(&def.key).is_some();
        assert_eq!(listed, !no_wood.contains(&def.key.as_str()), "{}", def.key);
    }
    for (key, wood, pieces) in FRUIT_KINDLING {
        assert!(no_wood.contains(&key), "{key}");
        assert!(wood.wood().is_some() && (5..=15).contains(&pieces), "{key}");
    }
    for (key, wood, logs) in FRUIT_WOODS {
        let def = fruits
            .get(key)
            .unwrap_or_else(|| panic!("{key} is a fruit"));
        assert!(def.plant.has_trunk(), "{key} has a trunk");
        assert!(wood.wood().is_some() && (1..=12).contains(&logs), "{key}");
    }
}

#[test]
fn trees_suit_their_place_and_the_same_cell_rolls_the_same() {
    let savanna = setting(Biome::Savanna, 24);
    for (kind, _) in counts(savanna) {
        assert!(kind.def().biomes.contains(&Biome::Savanna), "{kind:?}");
    }
    let hash = 123_456_789;
    assert_eq!(TreeKind::roll(savanna, hash), TreeKind::roll(savanna, hash));
    let needles = Setting {
        narrow: 1000,
        ..setting(Biome::BorealForest, 2)
    };
    for (kind, _) in counts(needles) {
        assert_eq!(kind.def().group, Group::Conifer, "{kind:?}");
    }
}

#[test]
fn rainforest_kinds_where_lush() {
    let lush = Setting {
        lush: 900,
        narrow: 0,
        ..setting(Biome::Savanna, 25)
    };
    for (kind, _) in counts(lush) {
        let biomes = kind.def().biomes;
        assert!(biomes.contains(&Biome::TropicalRainforest), "{kind:?}");
    }
}

#[test]
fn willow_and_alder_are_common_by_water() {
    let dry = setting(Biome::SeasonalForest, 10);
    let wet = Setting {
        near_water: true,
        ..dry
    };
    let by_water = |counts: std::collections::HashMap<TreeKind, u32>| {
        counts.get(&TreeKind::WhiteWillow).copied().unwrap_or(0)
            + counts.get(&TreeKind::BlackAlder).copied().unwrap_or(0)
    };
    assert!(by_water(counts(wet)) > 3 * by_water(counts(dry)));
}

#[test]
fn where_nothing_suits_the_nearest_kind_grows() {
    let frozen = setting(Biome::Tundra, -20);
    let kind = TreeKind::roll(frozen, 5);
    assert!(kind.def().biomes.contains(&Biome::Tundra), "{kind:?}");
    assert_eq!(kind, TreeKind::Larch, "the coldest");
}

#[test]
fn a_felled_tree_grows_back_a_stage_a_week() {
    let day = MINUTES_PER_DAY;
    assert_eq!(TreeStage::after_felling(0), TreeStage::Stump);
    assert_eq!(TreeStage::after_felling(day - 1), TreeStage::Stump);
    assert_eq!(
        TreeStage::after_felling(day),
        TreeStage::Cleared,
        "the stump gone"
    );
    assert_eq!(TreeStage::after_felling(5 * day - 1), TreeStage::Cleared);
    assert_eq!(TreeStage::after_felling(5 * day), TreeStage::Sapling);
    assert_eq!(TreeStage::after_felling(12 * day), TreeStage::Young);
    assert_eq!(TreeStage::after_felling(19 * day), TreeStage::HalfGrown);
    assert_eq!(TreeStage::after_felling(26 * day), TreeStage::Grown);
}

#[test]
fn walking_through_each_stage() {
    assert_eq!(TreeStage::Stump.cost_per_mille(), Some(1500));
    assert_eq!(TreeStage::Sapling.cost_per_mille(), Some(1000));
    assert_eq!(TreeStage::Cleared.cost_per_mille(), Some(1000));
    assert_eq!(TreeStage::Young.cost_per_mille(), Some(1500));
    assert_eq!(TreeStage::HalfGrown.cost_per_mille(), Some(3000));
    assert_eq!(TreeStage::Grown.cost_per_mille(), None, "blocks");
}

#[test]
fn young_trees_give_fewer_logs() {
    assert_eq!(TreeStage::Grown.logs(12), 12);
    assert_eq!(TreeStage::HalfGrown.logs(12), 6);
    assert_eq!(TreeStage::Young.logs(12), 3);
    assert_eq!(TreeStage::Young.logs(1), 1, "at least one");
    assert_eq!(TreeStage::Sapling.logs(12), 0);
    assert!(!TreeStage::Sapling.can_fell() && TreeStage::Young.can_fell());
    let oak = Tree::Wild(TreeKind::Oak);
    assert_eq!(
        (oak.wood(), oak.logs(TreeStage::Grown)),
        (Some(crate::material::Material::Oak), 11)
    );
    let banana = Tree::Fruit("banana".to_owned());
    assert_eq!(
        (banana.wood(), banana.logs(TreeStage::Grown)),
        (Some(crate::material::Material::Banana), 0)
    );
    assert_eq!(banana.kindling(TreeStage::Grown), 10);
    assert_eq!(banana.kindling(TreeStage::Young), 3, "a quarter, as logs");
    assert_eq!(oak.kindling(TreeStage::Grown), 0, "oak gives logs");
    let apple = Tree::Fruit("apple".to_owned());
    assert_eq!(apple.logs(TreeStage::Young), 1);
}
