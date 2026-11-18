use std::collections::BTreeSet;

use super::*;
use crate::poi::{ChunkPos, PoiMap};
use crate::world::WorldParams;

/// Wild plants are rare, lie on open land with open land all round, suit
/// their point, never grow on herbs or orchard-only fruit, and are the same
/// whenever made.
#[test]
fn wild_fruit_is_rare_and_suits_its_place() {
    let world = WorldParams::from_seed(Seed::new(12345));
    let wild = Wilderness::new(&world);
    let pois = PoiMap::new(&world, Seed::new(999));
    let herbs = crate::herb::Herbs::built_in();
    let fruits = Fruits::built_in();
    let (mut found, mut points) = (0, 0);
    let mut kinds = BTreeSet::new();
    for cy in -6..6 {
        for cx in 0..12 {
            let chunk = ChunkPos { x: cx, y: cy };
            let plan = ChunkPlan::with_new_site(&wild, &pois, chunk);
            let (x, y, _, _) = wild.chunk_bounds(chunk);
            let block = wild.block_of(x, y);
            let mut cells = wild.block(block, &plan);
            for ((x, y), patch) in wild.herb_patches(block, &plan, &cells, &herbs) {
                cells.set_herb(x, y, patch);
            }
            let plants = wild.fruit_plants(block, &plan, &cells);
            assert_eq!(plants, wild.fruit_plants(block, &plan, &cells));
            for ((x, y), plant) in &plants {
                assert!(cells.herb_at(*x, *y).is_none());
                for (dx, dy) in [(0, 0), (1, 1), (-1, -1), (1, -1), (-1, 1)] {
                    let cell = cells.cell(x + dx, y + dy);
                    assert!(matches!(cell, Cell::Ground | Cell::Grass), "{cell:?}");
                }
                assert!(!plan.cutout().is_some_and(|c| c.contains(*x, *y)));
                let (px, py) = wild.point_of(*x, *y);
                let def = fruits.get(&plant.fruit).unwrap();
                assert!(def.grows_wild(&wild.habitat_at(px, py)), "{}", def.key);
                assert_ne!(def.abundance, Abundance::OrchardOnly);
                kinds.insert(plant.fruit.clone());
            }
            found += plants.len();
            let (_, _, w, h) = wild.block_bounds(block);
            points += (w / SCALE) * (h / SCALE);
        }
    }
    assert!(found >= 5, "{found}");
    assert!(
        (found as i64) * 100 < points * WILD_FRUIT_PERCENT as i64,
        "{found} of {points}"
    );
    assert!(kinds.len() >= 3, "{kinds:?}");
}
