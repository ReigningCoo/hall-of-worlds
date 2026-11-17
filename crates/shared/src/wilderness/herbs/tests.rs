use super::*;
use crate::poi::{ChunkPos, PoiMap};
use crate::world::WorldParams;

/// Patches lie on open ground or grass, in clumps of a few of one herb that
/// suits their point, never in a cutout, one a cell, the same whenever made.
#[test]
fn wild_patches_suit_their_place() {
    let world = WorldParams::from_seed(Seed::new(12345));
    let wild = Wilderness::new(&world);
    let pois = PoiMap::new(&world, Seed::new(999));
    let herbs = Herbs::built_in();
    let mut found = 0;
    let mut kinds = BTreeSet::new();
    for cy in -3..3 {
        for cx in 0..6 {
            let chunk = ChunkPos { x: cx, y: cy };
            let plan = ChunkPlan::with_new_site(&wild, &pois, chunk);
            let (x, y, _, _) = wild.chunk_bounds(chunk);
            let block = wild.block_of(x, y);
            let cells = wild.block(block, &plan);
            let patches = wild.herb_patches(block, &plan, &cells, &herbs);
            assert_eq!(patches, wild.herb_patches(block, &plan, &cells, &herbs));
            let taken: BTreeSet<_> = patches.iter().map(|(at, _)| *at).collect();
            assert_eq!(taken.len(), patches.len(), "one patch a cell");
            for ((x, y), patch) in &patches {
                assert!(matches!(cells.cell(*x, *y), Cell::Ground | Cell::Grass));
                assert!(!plan.cutout().is_some_and(|c| c.contains(*x, *y)));
                let (px, py) = wild.point_of(*x, *y);
                let def = herbs.get(&patch.herb).unwrap();
                assert!(def.suits(&wild.habitat_at(px, py)), "{}", def.key);
                kinds.insert(patch.herb.clone());
            }
            found += patches.len();
        }
    }
    assert!(found > 100, "{found}");
    assert!(kinds.len() > 5, "{kinds:?}");
}
