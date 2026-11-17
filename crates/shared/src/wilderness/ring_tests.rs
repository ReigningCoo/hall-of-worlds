//! Nothing grows on a cutout's hatched border: no trees, no herb patches.

use super::*;
use crate::dungeon::Cell;
use crate::herb::Herbs;
use crate::poi::{ChunkPos, PoiMap};
use crate::site::Generator;
use std::collections::BTreeSet;

/// Every block touching a cutout's border, round many pois of every sort
/// (towns, set-in sites, cave mouths whose border follows the hill, ways
/// down whose border rings a 5 × 5 clearing), holds no tree and no herb
/// patch on any border cell.
#[test]
fn nothing_grows_on_a_border() {
    let world = WorldParams::from_seed(Seed::new(12345));
    let wild = Wilderness::new(&world);
    let pois = PoiMap::new(&world, Seed::new(999));
    let herbs = Herbs::built_in();
    let (mut caves, mut ways_down, mut ring_cells, mut near) = (0, 0, 0, 0);
    let mut kinds = BTreeSet::new();
    for n in 0..800 {
        let chunk = ChunkPos {
            x: n % 40,
            y: n / 40 - 10,
        };
        let plan = ChunkPlan::with_new_site(&wild, &pois, chunk);
        let Some(cutout) = plan.cutout() else {
            continue;
        };
        kinds.insert(format!("{:?}", cutout.poi().kind));
        match cutout.offset() {
            None => ways_down += 1,
            Some(_) if matches!(Generator::for_poi(cutout.poi().kind), Generator::Cave(_)) => {
                caves += 1
            }
            Some(_) => {}
        }
        let border: BTreeSet<_> = cutout.border().into_iter().collect();
        let blocks: BTreeSet<_> = border.iter().map(|&(x, y)| wild.block_of(x, y)).collect();
        for block in blocks {
            let cells = wild.block(block, &plan);
            for &(x, y) in &border {
                if cells.contains(x, y) {
                    assert!(plan.keeps_clear(x, y));
                    assert_ne!(cells.cell(x, y), Cell::Tree, "tree at ({x}, {y})");
                    ring_cells += 1;
                }
            }
            for ((x, y), _) in wild.herb_patches(block, &plan, &cells, &herbs) {
                assert!(!border.contains(&(x, y)), "herb at ({x}, {y})");
                if cutout.distance_squared(x, y) <= 25 {
                    near += 1;
                }
            }
        }
    }
    assert!(
        caves > 0 && ways_down > 0,
        "{caves} caves, {ways_down} ways down"
    );
    assert!(kinds.len() > 5, "{kinds:?}");
    assert!(ring_cells > 10_000, "{ring_cells}");
    // Herbs still grow just beyond the border.
    assert!(near > 0, "no herb near any cutout");
}
