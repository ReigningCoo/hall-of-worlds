use super::*;
use crate::poi::{ChunkPos, PoiMap};
use crate::world::WorldParams;

/// Fungi lie on open ground or grass free of herbs, or wait on trees for
/// felling, each suiting its point, never in a cutout, the same whenever made;
/// fewer than herbs.
#[test]
fn wild_fungi_suit_their_place() {
    let world = WorldParams::from_seed(Seed::new(12345));
    let wild = Wilderness::new(&world);
    let pois = PoiMap::new(&world, Seed::new(999));
    let fungi = Fungi::built_in();
    let herbs = crate::herb::Herbs::built_in();
    let (mut open, mut on_trees, mut herb_count) = (0, 0, 0);
    let mut kinds = BTreeSet::new();
    for cy in -3..3 {
        for cx in 0..6 {
            let chunk = ChunkPos { x: cx, y: cy };
            let plan = ChunkPlan::with_new_site(&wild, &pois, chunk);
            let (x, y, _, _) = wild.chunk_bounds(chunk);
            let block = wild.block_of(x, y);
            let mut cells = wild.block(block, &plan);
            for ((x, y), patch) in wild.herb_patches(block, &plan, &cells, &herbs) {
                cells.set_herb(x, y, patch);
                herb_count += 1;
            }
            let patches = wild.fungus_patches(block, &plan, &cells, fungi);
            assert_eq!(patches, wild.fungus_patches(block, &plan, &cells, fungi));
            for ((x, y), patch) in &patches {
                assert!(!plan.cutout().is_some_and(|c| c.contains(*x, *y)));
                let def = fungi.get(&patch.fungus).unwrap();
                let (px, py) = wild.point_of(*x, *y);
                let habitat = wild.habitat_at(px, py);
                assert!(def.biomes.contains(&habitat.biome), "{}", def.key);
                match cells.cell(*x, *y) {
                    Cell::Tree => {
                        assert!(def.feeds_on_wood(), "{}", def.key);
                        assert!(cells.fungus_at(*x, *y).is_none());
                        on_trees += 1;
                    }
                    Cell::Ground | Cell::Grass => {
                        assert!(cells.herb_at(*x, *y).is_none());
                        assert!(matches!(
                            def.grows,
                            crate::fungus::Where::Surface | crate::fungus::Where::Dug
                        ));
                        open += 1;
                    }
                    cell => panic!("{cell:?}"),
                }
                kinds.insert(patch.fungus.clone());
            }
        }
    }
    assert!(
        open > 5 && open < herb_count,
        "{open} fungi, {herb_count} herbs"
    );
    assert!(on_trees > 0, "{on_trees}");
    assert!(kinds.len() > 3, "{kinds:?}");
}
