//! Hearths (cooking plan PR 8): their crafts, storage, fire, where they go
//! and what they start with.

use super::*;
use crate::campfire::{Campfire, can_make_fire};
use crate::door::Side;
use crate::dungeon::{Cell, Level};
use crate::facing::Heading;
use crate::furniture::{Furniture, Material, Piece, Stone, Wood, footprint};
use crate::id::Id;
use crate::item::{Item, ItemKind};
use crate::poi::PoiKind;
use crate::rng::Seed;
use crate::site::Site;
use crate::spot::Spot;

fn item(kind: ItemKind, id: u128) -> Item {
    Item::new(Id::for_test(id), kind)
}

fn hearth_piece(craft: Craft, at: (i64, i64)) -> Piece {
    let furniture = Furniture::CraftingBench { craft };
    Piece {
        at,
        size: footprint(furniture, (craft.length(), 1), Side::South),
        facing: Side::South,
        material: Material::Stone(Stone::Granite),
        furniture,
    }
}

#[test]
fn hearths_are_three_sizes_of_stone() {
    assert_eq!(
        [Craft::Stove, Craft::Hearth, Craft::Range].map(Craft::length),
        [1, 2, 3]
    );
    assert!(Craft::ALL.iter().filter(|c| c.is_hearth()).count() == 3);
    assert!(!Craft::Alchemy.is_hearth());
    assert!(Craft::Stove.tools().is_empty());
    assert_eq!(Craft::Hearth.tools(), KITCHEN_TOOLS);
    assert!(Craft::Range.has_storage() && !Craft::Hearth.has_storage());
    assert!(hearth_piece(Craft::Range, (1, 1)).is_well_made());
    let wooden = Piece {
        material: Material::Wood(Wood::Oak),
        ..hearth_piece(Craft::Hearth, (1, 1))
    };
    assert!(!wooden.is_well_made(), "hearths are stone");
    for tool in KITCHEN_TOOLS {
        assert!(crate::cooking::Tool::of_item(tool).is_some(), "{tool:?}");
    }
}

#[test]
fn only_a_range_stores_food() {
    let mut range = BenchTools::new(Craft::Range);
    let mut hearth = BenchTools::new(Craft::Hearth);
    assert_eq!(hearth.put(item(ItemKind::Dish, 1)), Err(NotTaken));
    assert_eq!(range.put(item(ItemKind::Herb, 2)), Err(NotTaken));
    assert_eq!(range.put(item(ItemKind::Pan, 3)), Ok(None));
    for n in 0..STORAGE_PLACES as u128 {
        assert_eq!(range.put(item(ItemKind::Intermediate, 10 + n)), Ok(None));
    }
    assert_eq!(range.put(item(ItemKind::Dish, 99)), Err(NotTaken), "full");
    assert_eq!(range.storage().len(), STORAGE_PLACES);
    assert_eq!(range.items().count(), STORAGE_PLACES + 1);
    let taken = range.take(Id::for_test(10)).expect("stored");
    assert_eq!(taken.kind, ItemKind::Intermediate);
    assert_eq!(range.put(item(ItemKind::Dish, 99)), Ok(None));
    assert!(range.take(Id::for_test(3)).is_some(), "tools come off too");
    assert_eq!(hearth.put(item(ItemKind::Sieve, 4)), Ok(None));
}

#[test]
fn hearths_are_lit_always_or_by_their_fire() {
    let mut tools = BenchTools::new(Craft::Stove);
    assert!(!tools.is_lit(0));
    tools.fire = Some(Campfire::lit(100, crate::material::Material::Oak));
    assert!(tools.is_lit(101));
    assert!(!tools.is_lit(100_000), "burnt out");
    tools.always_lit = true;
    assert!(tools.is_lit(100_000));
    let mut alchemy = BenchTools::new(Craft::Alchemy);
    alchemy.always_lit = true;
    assert!(!alchemy.is_lit(0), "only hearths give heat");
    assert!(always_lit(PoiKind::Castle) && always_lit(PoiKind::StoneTower));
    assert!(!always_lit(PoiKind::ThatchedHut));
    assert!(town_hearth(PoiKind::Village) && !town_hearth(PoiKind::Castle));
    assert_eq!(town_tools(), KITCHEN_TOOLS);
}

#[test]
fn ash_comes_from_cooking_time() {
    assert_eq!(hearth_ash(0), 0);
    assert_eq!(hearth_ash(1), 1);
    assert_eq!(hearth_ash(ASH_HEAT_SECONDS), 1);
    assert_eq!(hearth_ash(ASH_HEAT_SECONDS + 1), 2);
}

#[test]
fn a_hearth_faced_can_be_lit() {
    let mut level = Level::new(0, 0, 8, 8);
    for x in 0..8 {
        for y in 0..8 {
            level.set(x, y, if y == 0 { Cell::Wall } else { Cell::Floor });
        }
    }
    level.place_furniture(hearth_piece(Craft::Stove, (3, 1)));
    let at = Spot::middle_of((3, 2));
    let north = Heading::NORTH;
    assert!(hearth_faced(&level, at, north).is_some());
    assert!(can_make_fire(&level, at, north));
    let mut alchemy = hearth_piece(Craft::Stove, (6, 1));
    alchemy.furniture = Furniture::CraftingBench {
        craft: Craft::Alchemy,
    };
    alchemy.size = (2, 1);
    level.place_furniture(alchemy);
    assert!(hearth_faced(&level, Spot::middle_of((6, 2)), north).is_none());
}

#[test]
fn hearths_go_in_households_and_huts() {
    let hearths = |kind: PoiKind, n: u64| -> Vec<Craft> {
        let site = Site::new(kind, Seed::new(n * 7919 + 101));
        (0..site.levels())
            .flat_map(|index| {
                let level = site.level(index).level;
                let crafts: Vec<Craft> = (level.furniture())
                    .filter_map(|piece| match piece.furniture {
                        Furniture::CraftingBench { craft } if craft.is_hearth() => Some(craft),
                        _ => None,
                    })
                    .collect();
                crafts
            })
            .collect()
    };
    for kind in [PoiKind::StoneTower, PoiKind::Castle, PoiKind::ThatchedHut] {
        let found: Vec<Craft> = (0..20).flat_map(|n| hearths(kind, n)).collect();
        assert!(!found.is_empty(), "{kind:?}");
        let allowed: Vec<Craft> = hearth_weights(kind).iter().map(|&(c, _)| c).collect();
        assert!(found.iter().all(|c| allowed.contains(c)), "{kind:?}");
    }
    for n in 0..5 {
        assert!(hearths(PoiKind::GrassyCave, n).is_empty());
        let one: Vec<Craft> = hearths(PoiKind::StoneTower, n);
        assert!(one.windows(2).all(|w| w[0] == w[1]), "one craft a site");
    }
    assert_eq!(bench_percent(Craft::Range, PoiKind::Castle), 100);
    assert_eq!(bench_percent(Craft::Stove, PoiKind::Castle), 0);
}

#[test]
fn hearths_start_with_a_pan_and_a_pot() {
    let range = hearth_piece(Craft::Range, (1, 1));
    for n in 0..10 {
        let tools = starting_tools(PoiKind::ThatchedHut, Seed::new(n), 0, &range);
        let kinds: Vec<ItemKind> = tools.iter().map(|&(kind, _)| kind).collect();
        assert!(kinds.contains(&ItemKind::Pan) && kinds.contains(&ItemKind::Cauldron));
        assert!(kinds.iter().all(|&kind| Craft::Range.takes(kind)));
    }
    let stove = hearth_piece(Craft::Stove, (1, 1));
    assert!(starting_tools(PoiKind::Castle, Seed::new(1), 0, &stove).is_empty());
}
