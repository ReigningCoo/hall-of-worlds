use std::collections::BTreeSet;

use super::*;
use crate::alchemy::{
    AlchemyOrder, AlchemySkills, Apparatus, OrderRefused, OrderStep, circulated_quality,
    worked_quality,
};
use crate::door::Side;
use crate::dungeon::{Cell, Level};
use crate::facing::Heading;
use crate::furniture::{Furniture, Material, Piece, Wood};
use crate::herb::{Herbs, Stat};
use crate::id::Id;
use crate::item::{Inventory, Item, ItemKind, Quality, StackLimits};
use crate::poi::PoiKind;
use crate::potion::{Contents, Dose};
use crate::rng::Seed;
use crate::site::{Site, SiteLevel};
use crate::spot::Spot;

fn tool(kind: ItemKind, id: u128) -> Item {
    Item::new(Id::for_test(id), kind)
}

#[test]
fn crafts_have_lengths_and_tools() {
    for craft in Craft::ALL {
        assert!((1..=3).contains(&craft.length()), "{craft:?}");
        assert_eq!(craft.tools().is_empty(), craft == Craft::Stove);
    }
    assert_eq!(Craft::Alchemy.length(), 2);
    let apparatus: Vec<_> = Apparatus::ALL.iter().filter_map(|a| a.item()).collect();
    assert_eq!(Craft::Alchemy.tools(), apparatus);
    assert!(Craft::Alchemy.takes(ItemKind::Pelican));
    assert!(!Craft::Alchemy.takes(ItemKind::Bottle));
}

#[test]
fn a_tool_of_a_kind_already_there_swaps() {
    let mut bench = BenchTools::new(Craft::Alchemy);
    assert_eq!(bench.put(tool(ItemKind::Retort, 1)), Ok(None));
    assert_eq!(bench.put(tool(ItemKind::MortarAndPestle, 2)), Ok(None));
    let kinds: Vec<_> = bench.tools().iter().map(|t| t.kind).collect();
    assert_eq!(
        kinds,
        [ItemKind::MortarAndPestle, ItemKind::Retort],
        "craft order"
    );
    let old = bench.put(tool(ItemKind::Retort, 3)).unwrap().unwrap();
    assert_eq!(old.id, Id::for_test(1));
    assert_eq!(bench.tool(ItemKind::Retort).unwrap().id, Id::for_test(3));
    assert_eq!(bench.tools().len(), 2);
    assert_eq!(
        bench.take(Id::for_test(2)).unwrap().kind,
        ItemKind::MortarAndPestle
    );
    assert!(!bench.has(ItemKind::MortarAndPestle));
    assert_eq!(bench.take(Id::for_test(2)), None);
}

#[test]
fn other_things_get_the_one_refusal() {
    let mut bench = BenchTools::new(Craft::Alchemy);
    assert_eq!(bench.put(tool(ItemKind::Bottle, 1)), Err(NotTaken));
    let mut two = tool(ItemKind::Filter, 2);
    two.count = 2;
    assert_eq!(bench.put(two), Err(NotTaken));
    assert!(bench.tools().is_empty());
}

fn bench_piece(at: (i64, i64), facing: Side) -> Piece {
    let furniture = Furniture::CraftingBench {
        craft: Craft::Alchemy,
    };
    Piece {
        at,
        size: crate::furniture::footprint(furniture, (2, 1), facing),
        facing,
        material: Material::Wood(Wood::Oak),
        furniture,
    }
}

#[test]
fn a_bench_is_used_within_reach_and_faced() {
    let mut level = Level::new(0, 0, 10, 10);
    for y in 0..10 {
        for x in 0..10 {
            level.set(x, y, Cell::Floor);
        }
    }
    let piece = bench_piece((5, 5), Side::North);
    assert!(piece.is_well_made());
    level.place_furniture(piece);
    for x in [5, 6] {
        let front = Spot::middle_of((x, 4));
        assert_eq!(bench_faced(&level, front, Heading::SOUTH), Some(piece));
        assert_eq!(bench_faced(&level, front, Heading::NORTH), None, "behind");
    }
    let far = Spot::middle_of((5, 2));
    assert_eq!(bench_faced(&level, far, Heading::SOUTH), None, "too far");
}

#[test]
fn a_bench_raises_the_purity_score_with_pelicans() {
    let mut raised = false;
    for step in 1..30 {
        let contents = Contents::new([Dose {
            stat: Stat::Aura,
            step,
        }]);
        for circulated in 0..3 {
            let plain = worked_quality(&contents, circulated, false);
            let bench = worked_quality(&contents, circulated, true);
            assert_eq!(plain, circulated_quality(&contents, circulated));
            assert!(bench >= plain);
            raised |= bench > plain;
        }
    }
    assert!(raised, "some product is a quality better at a bench");
}

fn carrying(items: Vec<Item>) -> Inventory {
    let mut inventory = Inventory::default();
    let stand_ins = &mut crate::id::stand_ins();
    for item in items {
        (inventory.give(item, StackLimits::default(), stand_ins)).unwrap();
    }
    inventory
}

#[test]
fn a_bench_s_tools_count_as_apparatus() {
    let herbs = Herbs::built_in();
    let def = herbs.iter().next().unwrap();
    let herb = crate::herb::new_item(def, 3, Id::for_test(1), None);
    let carried = carrying(vec![herb]);
    let stack = carried.herb_stacks()[0].stack;
    let order = AlchemyOrder {
        bench: None,
        source: None,
        steps: vec![OrderStep::Mortar {
            herbs: vec![stack],
            solvent: None,
        }],
        count: 1,
    };
    let skills = AlchemySkills::default();
    let refused = carried.check_alchemy(&order, &herbs, skills);
    assert_eq!(refused, Err(OrderRefused::Apparatus(Apparatus::Mortar)));
    let mut bench = BenchTools::new(Craft::Alchemy);
    bench.put(tool(ItemKind::MortarAndPestle, 9)).unwrap();
    let at = carried.check_alchemy_at(&order, &herbs, skills, Some(&bench));
    assert!(at.is_ok(), "the bench's mortar is used");
    let empty = BenchTools::new(Craft::Alchemy);
    let without = carried.check_alchemy_at(&order, &herbs, skills, Some(&empty));
    assert_eq!(without, refused);
}

/// Every level of `count` sites of `kind`, with the site's level count.
fn levels(kind: PoiKind, count: u64) -> Vec<(u64, u32, u32, SiteLevel)> {
    let mut all = Vec::new();
    for n in 0..count {
        let site = Site::new(kind, Seed::new(n * 7919 + 101));
        for index in 0..site.levels() {
            all.push((n, site.levels(), index, site.level(index)));
        }
    }
    all
}

/// The Alchemy benches on a level (hearths are `hearth_tests.rs`').
fn benches(level: &Level) -> Vec<Piece> {
    let is_bench = |p: &&Piece| matches!(p.furniture, Furniture::CraftingBench { craft } if !craft.is_hearth());
    level.furniture().filter(is_bench).copied().collect()
}

/// Cells a 4-way fill reaches from `start` (every door open), round
/// furniture, all of it or all but the benches.
fn fill(level: &Level, start: (i64, i64), round_benches: bool) -> BTreeSet<(i64, i64)> {
    let open = level.with_all_doors_open();
    let blocked = |x, y| {
        level.furniture_at(x, y).is_some_and(|piece| {
            round_benches || !matches!(piece.furniture, Furniture::CraftingBench { .. })
        })
    };
    let mut reached = BTreeSet::new();
    let mut queue = vec![start];
    while let Some((x, y)) = queue.pop() {
        if !open.is_walkable(x, y) || blocked(x, y) || !reached.insert((x, y)) {
            continue;
        }
        queue.extend([(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]);
    }
    reached
}

#[test]
fn herbalists_have_benches_against_walls_cutting_nothing_off() {
    let mut per_site: std::collections::BTreeMap<u64, (u32, usize)> = Default::default();
    for (n, count, _, site) in levels(PoiKind::HerbalistHut, 12) {
        let level = &site.level;
        let found = benches(level);
        per_site.entry(n).or_insert((count, 0)).1 += found.len();
        for piece in &found {
            assert!(piece.is_well_made(), "{piece:?}");
            let (dx, dy) = match piece.facing {
                Side::South => (0, -1),
                Side::West => (1, 0),
                Side::North => (0, 1),
                Side::East => (-1, 0),
            };
            for (x, y) in piece.cells() {
                assert_eq!(level.cell(x + dx, y + dy), Cell::Wall, "back to a wall");
                assert_eq!(level.cell(x, y), Cell::Floor);
                assert!(level.room(x, y).is_some(), "in a room");
            }
        }
        let starts = site.stairs.iter().map(|s| (s.x, s.y));
        for start in site.entrance.into_iter().chain(starts) {
            let without = fill(level, start, false);
            let with = fill(level, start, true);
            let lost = without.difference(&with);
            let all = level.furniture();
            let all = all.filter(|p| matches!(p.furniture, Furniture::CraftingBench { .. }));
            let bench_cells: BTreeSet<_> = all.flat_map(|p| p.cells()).collect();
            assert!(lost.into_iter().all(|cell| bench_cells.contains(cell)));
        }
    }
    for (count, found) in per_site.values() {
        assert!(
            *found as u32 <= count.div_ceil(5),
            "one a craft per 5 floors"
        );
    }
    assert!(
        per_site.values().any(|&(_, found)| found > 0),
        "some placed"
    );
}

#[test]
fn alchemy_benches_go_only_in_some_sites_and_are_the_same_every_time() {
    for (_, _, _, site) in levels(PoiKind::StoneCottage, 6) {
        assert!(benches(&site.level).is_empty());
    }
    for kind in [PoiKind::Castle, PoiKind::StoneTower] {
        let some = levels(kind, 30);
        assert!(
            some.iter()
                .any(|(.., site)| !benches(&site.level).is_empty())
        );
        assert!(
            some.iter()
                .any(|(.., site)| benches(&site.level).is_empty())
        );
    }
    let site = Site::new(PoiKind::HerbalistHut, Seed::new(5));
    let again = Site::new(PoiKind::HerbalistHut, Seed::new(5));
    assert_eq!(
        benches(&site.level(0).level),
        benches(&again.level(0).level)
    );
}

#[test]
fn starting_tools_are_the_fewest_useful() {
    let seed = Seed::new(3);
    let piece = bench_piece((1, 1), Side::South);
    let kinds = |kind| -> Vec<ItemKind> {
        let tools = starting_tools(kind, seed, 0, &piece);
        tools.into_iter().map(|(kind, _)| kind).collect()
    };
    assert_eq!(kinds(PoiKind::HerbalistHut), [ItemKind::MortarAndPestle]);
    assert_eq!(
        kinds(PoiKind::StoneTower),
        [ItemKind::MortarAndPestle, ItemKind::Retort]
    );
    for n in 0..20 {
        let tools = starting_tools(PoiKind::Castle, Seed::new(n), 0, &piece);
        assert!(tools.len() >= 2);
        assert!(
            tools
                .iter()
                .all(|&(kind, q)| { Craft::Alchemy.takes(kind) && q != Quality::Artifact })
        );
    }
    let table = Piece {
        furniture: Furniture::Table {
            style: crate::furniture::TableStyle::Trestle,
        },
        ..piece
    };
    assert!(starting_tools(PoiKind::Castle, seed, 0, &table).is_empty());
}
