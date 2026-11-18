use std::collections::BTreeSet;

use super::*;
use crate::herb::Herbs;
use crate::poi::{ChunkPos, PoiMap};
use crate::vegetable::Abundance;
use crate::world::WorldParams;

/// Wild vegetables lie on open land, never on herbs or in cutouts, suit their
/// point's biome, are the same whenever made, and are mostly wild-only plants,
/// the farmed crops among them rare.
#[test]
fn wild_vegetables_suit_their_place() {
    let world = WorldParams::from_seed(Seed::new(12345));
    let wild = Wilderness::new(&world);
    let pois = PoiMap::new(&world, Seed::new(999));
    let herbs = Herbs::built_in();
    let vegetables = Vegetables::built_in();
    let (mut crops, mut wild_only, mut points) = (0, 0, 0);
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
            let plants = wild.vegetable_patches(block, &plan, &cells);
            assert_eq!(plants, wild.vegetable_patches(block, &plan, &cells));
            let mut seen = BTreeSet::new();
            for ((x, y), plant) in &plants {
                assert!(seen.insert((*x, *y)), "two on ({x}, {y})");
                assert!(cells.herb_at(*x, *y).is_none());
                assert!(matches!(cells.cell(*x, *y), Cell::Ground | Cell::Grass));
                assert!(!plan.cutout().is_some_and(|c| c.contains(*x, *y)));
                let (px, py) = wild.point_of(*x, *y);
                let def = vegetables.get(&plant.vegetable).unwrap();
                assert!(def.grows_wild(wild.habitat_at(px, py).biome), "{}", def.key);
                if def.wild_only {
                    wild_only += 1;
                } else {
                    crops += 1;
                    assert_ne!(def.abundance, Abundance::Common);
                }
                kinds.insert(plant.vegetable.clone());
            }
            let (_, _, w, h) = wild.block_bounds(block);
            points += (w / SCALE) * (h / SCALE);
        }
    }
    assert!(crops >= 3, "{crops}");
    assert!(
        (crops as i64) * 1000 < points * (WILD_CROP_PER_MILLE * WILD_PATCH_MOST) as i64,
        "{crops} of {points}"
    );
    assert!(wild_only > crops, "{wild_only} wild-only, {crops} crops");
    assert!(kinds.len() >= 5, "{kinds:?}");
}
